mod logic;

use bevy::{
    log::LogPlugin,
    prelude::{
        info, App, Commands, Entity, EventReader, ParallelSystemDescriptorCoercion, Query, ResMut,
        With, Without,
    },
    MinimalPlugins,
};
use durakifa_protocol::protocol::{Name, Own, Owner, Player, Protocol, Room};
use logic::lobby::Lobby;
use naia_bevy_server::{
    events::{AuthorizationEvent, DisconnectionEvent, MessageEvent},
    shared::{DefaultChannels, SharedConfig},
    Plugin as ServerPlugin, Server, ServerAddrs, ServerConfig, Stage,
};
use obfstr::obfstr;

const SRV_ADDR: &str = "127.0.0.1";
const SRV_PORT: &str = "55500";
const SRV_PORT_WRTC: &str = "55501";

#[cfg(debug_assertions)]
const SRV_ADDR_PUB: &str = SRV_ADDR;
#[cfg(not(debug_assertions))]
const SRV_ADDR_PUB: &str = env!("SRV_ADDR");

#[cfg(debug_assertions)]
const SRV_PROT: &str = "http";
#[cfg(not(debug_assertions))]
const SRV_PROT: &str = env!("SRV_PROT");

#[cfg(not(debug_assertions))]
const SRV_KEY: &str = env!("SRV_KEY");
#[cfg(debug_assertions)]
const SRV_KEY: &str = "SRV_KEY";

struct Global {
    lobby: Lobby,
}

fn authorize(
    mut event_reader: EventReader<AuthorizationEvent<Protocol>>,
    mut server: Server<Protocol, DefaultChannels>,
) {
    for event in event_reader.iter() {
        if let AuthorizationEvent(user_key, Protocol::Auth(msg)) = event {
            if &*msg.key == obfstr!(SRV_KEY) {
                server.accept_connection(&user_key);
            } else {
                server.reject_connection(&user_key);
            }
        }
    }
}

fn debug<'world, 'state>(
    players: Query<(Entity, &Name), With<Player>>,
    others: Query<Entity, (Without<Player>, Without<Room>)>,
    rooms: Query<(Entity, &Name), With<Room>>,
    server: Server<'world, 'state, Protocol, DefaultChannels>,
) {
    for (i, room_key) in server.room_keys().iter().enumerate() {
        info!(
            "{} (entities:{}, users:{}):",
            i,
            server.room(&room_key).entities_count(),
            server.room(&room_key).users_count(),
        );

        for (entity, player) in players.iter() {
            if server.room(&room_key).has_entity(&entity) {
                info!("  player: {}", (*player.name).clone());
            }
        }

        for entity in others.iter() {
            if server.room(&room_key).has_entity(&entity) {
                info!("  other");
            }
        }

        for (entity, room) in rooms.iter() {
            if server.room(&room_key).has_entity(&entity) {
                info!("  room: {}", (*room.name).clone());
            }
        }
    }
}

fn disconnect<'world, 'state>(
    mut event_reader: EventReader<DisconnectionEvent>,
    mut global: ResMut<Global>,
    query: Query<&Owner>,
    mut server: Server<'world, 'state, Protocol, DefaultChannels>,
) {
    for event in event_reader.iter() {
        let DisconnectionEvent(user_key, _) = event;
        if let Some(player) = global.lobby.get_player(*user_key) {
            if query.get(player).is_ok() {
                if let Some(entity) = global.lobby.get_successor(*user_key) {
                    server.entity_mut(&entity).insert(Owner::new());
                }
            }
        }

        global.lobby.clear_user(&mut server, *user_key);
    }
}

fn enter_room<'world, 'state>(
    mut global: ResMut<Global>,
    mut events: EventReader<MessageEvent<Protocol, DefaultChannels>>,
    mut server: Server<'world, 'state, Protocol, DefaultChannels>,
) {
    let global = &mut *global;
    for event in events.iter() {
        if let MessageEvent(user_key, _, Protocol::Join(msg)) = event {
            if let Some(room) = msg.room.get(&server) {
                if let Some(_player) = global.lobby.enter_room(room, &mut server, *user_key) {
                    //TODO: this would be too early, because client hasn't synced
                    //      entities yet
                    /*
                        let mut own = Own::new();
                        own.player.set(&server, &player);
                        server.send_message(user_key, DefaultChannels::UnorderedReliable, &own);
                    */
                }
            }
        }
    }
}

fn leave_room<'world, 'state>(
    mut global: ResMut<Global>,
    mut events: EventReader<MessageEvent<Protocol, DefaultChannels>>,
    mut server: Server<'world, 'state, Protocol, DefaultChannels>,
) {
    let global = &mut *global;
    for event in events.iter() {
        if let MessageEvent(user_key, _, Protocol::Leave(_)) = event {
            if let Some(player) = global.lobby.to_lobby(&mut server, *user_key) {
                server.entity_mut(&player).remove::<Owner>();

                //TODO: this would be too early, because client hasn't synced
                //      entities yet
                /*
                    let mut own = Own::new();
                    own.player.set(&server, &player);
                    server.send_message(user_key, DefaultChannels::UnorderedReliable, &own);
                */
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_plugin(ServerPlugin::<Protocol, DefaultChannels>::new(
            ServerConfig::default(),
            SharedConfig::default(),
        ))
        .add_startup_system(setup)
        .add_system_to_stage(Stage::ReceiveEvents, authorize)
        .add_system_to_stage(Stage::ReceiveEvents, disconnect)
        .add_system_to_stage(Stage::ReceiveEvents, enter_room)
        .add_system_to_stage(Stage::ReceiveEvents, leave_room)
        .add_system_to_stage(Stage::ReceiveEvents, register)
        .add_system_to_stage(Stage::ReceiveEvents, spawn_room)
        .add_system_to_stage(Stage::Tick, debug)
        .add_system_to_stage(Stage::Tick, update_scope.after(debug))
        .add_system_to_stage(Stage::Tick, update_server.after(update_scope))
        .run();
}

fn register<'world, 'state>(
    mut global: ResMut<Global>,
    mut events: EventReader<MessageEvent<Protocol, DefaultChannels>>,
    mut server: Server<'world, 'state, Protocol, DefaultChannels>,
) {
    for event in events.iter() {
        if let MessageEvent(user_key, _, Protocol::Register(msg)) = event {
            let player = &global.lobby.register(&mut server, *user_key);
            server
                .entity_mut(player)
                .insert(Name::new((*msg.name).clone()))
                .insert(Player::new());

            //TODO: this should be the only place where sending the Own message
            //      is neccessary
            let mut own = Own::new();
            own.player.set(&server, player);
            server.send_message(user_key, DefaultChannels::UnorderedReliable, &own);
        }
    }
}

fn setup(mut commands: Commands, mut server: Server<Protocol, DefaultChannels>) {
    server.listen(&ServerAddrs::new(
        format!("{}:{}", SRV_ADDR, SRV_PORT).parse().unwrap(),
        format!("{}:{}", SRV_ADDR, SRV_PORT_WRTC).parse().unwrap(),
        &format!("{}://{}:{}", SRV_PROT, SRV_ADDR_PUB, SRV_PORT_WRTC),
    ));

    commands.insert_resource(Global {
        lobby: Lobby::new(server.make_room().key()),
    });
}

fn spawn_room<'world, 'state>(
    mut global: ResMut<Global>,
    mut events: EventReader<MessageEvent<Protocol, DefaultChannels>>,
    mut server: Server<'world, 'state, Protocol, DefaultChannels>,
) {
    for event in events.iter() {
        if let MessageEvent(user_key, _, Protocol::Add(msg)) = event {
            let (player, room) = global.lobby.spawn_room(&mut server, *user_key);
            server.entity_mut(&player).insert(Owner::new());
            server
                .entity_mut(&room)
                .insert(Name::new((*msg.name).clone()))
                .insert(Room::new());

            //TODO: this would be too early, because client hasn't synced
            //      entities yet
            /*
                let mut own = Own::new();
                own.player.set(&server, &player);
                server.send_message(user_key, DefaultChannels::UnorderedReliable, &own);
            */
        }
    }
}

fn update_scope(mut server: Server<Protocol, DefaultChannels>) {
    for (_, user_key, entity) in server.scope_checks() {
        server.user_scope(&user_key).include(&entity);
    }
}

fn update_server(mut server: Server<Protocol, DefaultChannels>) {
    server.send_all_updates();
}
