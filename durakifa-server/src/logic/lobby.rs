use std::collections::HashMap;

use bevy_ecs::prelude::Entity;
use durakifa_protocol::protocol::Protocol;
use naia_bevy_server::{shared::DefaultChannels, RoomKey, Server, UserKey};

struct LobbyRoom {
    entity: Entity,
    players: HashMap<UserKey, Entity>,
}

pub struct Lobby {
    lobby_key: RoomKey,
    users: HashMap<UserKey, Entity>,
    rooms: HashMap<RoomKey, LobbyRoom>,
}

impl Lobby {
    pub fn new(lobby_key: RoomKey) -> Self {
        Lobby {
            lobby_key,
            users: HashMap::new(),
            rooms: HashMap::new(),
        }
    }

    pub fn clear_user<'world, 'state>(
        &mut self,
        server: &mut Server<'world, 'state, Protocol, DefaultChannels>,
        user_key: UserKey,
    ) -> Option<(Entity, Entity)> {
        let mut res = None;
        for (room_key, room) in self.rooms.iter_mut() {
            if let Some(player) = room.players.remove(&user_key) {
                server.entity_mut(&player).leave_room(room_key).despawn();
                if let Some(&successor) = room.players.values().next() {
                    res = Some((room.entity, successor));
                }
            }
        }

        self.tidy(server);
        if let Some(user) = self.users.remove(&user_key) {
            server
                .entity_mut(&user)
                .leave_room(&self.lobby_key)
                .despawn();
        }

        res
    }

    pub fn enter_room<'world, 'state>(
        &mut self,
        room: Entity,
        server: &mut Server<'world, 'state, Protocol, DefaultChannels>,
        user_key: UserKey,
    ) -> Option<Entity> {
        for (room_key, lobby_room) in self.rooms.iter_mut() {
            if lobby_room.entity == room {
                let player = server.spawn().enter_room(room_key).id();
                lobby_room.players.insert(user_key, player);
                server.user_mut(&user_key).enter_room(room_key);
                return Some(player);
            }
        }

        None
    }

    pub fn get_user(&self, user_key: UserKey) -> Option<Entity> {
        if let Some(&user) = self.users.get(&user_key) {
            return Some(user);
        }

        None
    }

    pub fn leave_room<'world, 'state>(
        &mut self,
        server: &mut Server<'world, 'state, Protocol, DefaultChannels>,
        user_key: UserKey,
    ) -> Option<(Entity, Entity)> {
        let mut res = None;
        for (room_key, room) in &mut self.rooms {
            if let Some(player) = room.players.remove(&user_key) {
                server.entity_mut(&player).despawn();
                server.user_mut(&user_key).leave_room(&room_key);
                if let Some(&successor) = room.players.values().next() {
                    res = Some((room.entity, successor));
                }
            }
        }

        self.tidy(server);
        res
    }

    pub fn register<'world, 'state>(
        &mut self,
        server: &mut Server<'world, 'state, Protocol, DefaultChannels>,
        user_key: UserKey,
    ) -> Entity {
        let user = server.spawn().enter_room(&self.lobby_key).id();
        self.users.insert(user_key, user);
        server.user_mut(&user_key).enter_room(&self.lobby_key);
        user
    }

    pub fn spawn_room<'world, 'state>(
        &mut self,
        server: &mut Server<'world, 'state, Protocol, DefaultChannels>,
        user_key: UserKey,
    ) -> (Entity, Entity) {
        let room = server.spawn().enter_room(&self.lobby_key).id();
        let room_key = server.make_room().key();
        self.rooms.insert(
            room_key,
            LobbyRoom {
                entity: room,
                players: HashMap::new(),
            },
        );

        (self.enter_room(room, server, user_key).unwrap(), room)
    }

    fn tidy<'world, 'state>(
        &mut self,
        server: &mut Server<'world, 'state, Protocol, DefaultChannels>,
    ) {
        self.rooms.retain(|room_key, room| {
            let retain = !room.players.is_empty();
            if !retain {
                server.room_mut(room_key).destroy();
                server
                    .entity_mut(&room.entity)
                    .leave_room(&self.lobby_key)
                    .despawn();
            }

            retain
        });
    }
}
