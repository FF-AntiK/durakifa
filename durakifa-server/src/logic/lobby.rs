use std::collections::HashMap;

use bevy::prelude::Entity;
use durakifa_protocol::protocol::Protocol;
use naia_bevy_server::{shared::DefaultChannels, RoomKey, Server, UserKey};

struct LobbyRoom {
    entity: Entity,
    players: HashMap<UserKey, Entity>,
}

pub struct Lobby {
    lobby_key: RoomKey,
    players: HashMap<UserKey, Entity>,
    rooms: HashMap<RoomKey, LobbyRoom>,
}

impl Lobby {
    pub fn new(lobby_key: RoomKey) -> Self {
        Lobby {
            lobby_key,
            players: HashMap::new(),
            rooms: HashMap::new(),
        }
    }

    pub fn clear_user<'world, 'state>(
        &mut self,
        server: &mut Server<'world, 'state, Protocol, DefaultChannels>,
        user_key: UserKey,
    ) {
        for (room_key, room) in self.rooms.iter_mut() {
            if let Some(player) = room.players.remove(&user_key) {
                server.entity_mut(&player).leave_room(room_key).despawn();
            }
        }

        self.tidy(server);
        if let Some(player) = self.players.remove(&user_key) {
            server
                .entity_mut(&player)
                .leave_room(&self.lobby_key)
                .despawn();
        }
    }

    pub fn enter_room<'world, 'state>(
        &mut self,
        room: Entity,
        server: &mut Server<'world, 'state, Protocol, DefaultChannels>,
        user_key: UserKey,
    ) -> Option<Entity> {
        for (room_key, lobby_room) in self.rooms.iter_mut() {
            if lobby_room.entity == room {
                if let Some(player) = self.players.remove(&user_key) {
                    lobby_room.players.insert(user_key, player);
                    server
                        .entity_mut(&player)
                        .leave_room(&self.lobby_key)
                        .enter_room(room_key);

                    server
                        .user_mut(&user_key)
                        .leave_room(&self.lobby_key)
                        .enter_room(room_key);

                    return Some(player);
                }
            }
        }

        None
    }

    pub fn get_player(&self, user_key: UserKey) -> Option<Entity> {
        if let Some(player) = self.players.get(&user_key) {
            return Some(*player);
        }

        for (_, room) in self.rooms.iter() {
            if let Some(player) = room.players.get(&user_key) {
                return Some(*player);
            }
        }

        None
    }

    pub fn get_successor(&self, user_key: UserKey) -> Option<Entity> {
        for (_, room) in self.rooms.iter() {
            if room.players.get(&user_key).is_some() {
                if let Some((_, player)) = room.players.iter().find(|(k, _)| **k != user_key) {
                    return Some(*player);
                }
            }
        }

        None
    }

    pub fn register<'world, 'state>(
        &mut self,
        server: &mut Server<'world, 'state, Protocol, DefaultChannels>,
        user_key: UserKey,
    ) -> Entity {
        let player = server.spawn().enter_room(&self.lobby_key).id();
        self.players.insert(user_key, player);
        server.user_mut(&user_key).enter_room(&self.lobby_key);
        player
    }

    pub fn spawn_room<'world, 'state>(
        &mut self,
        server: &mut Server<'world, 'state, Protocol, DefaultChannels>,
        user_key: UserKey,
    ) -> (Entity, Entity) {
        let player = self.players.remove(&user_key).unwrap();
        let room = server.spawn().enter_room(&self.lobby_key).id();
        let room_key = server.make_room().key();
        self.rooms.insert(
            room_key,
            LobbyRoom {
                entity: room,
                players: [(user_key, player)].iter().cloned().collect(),
            },
        );

        server
            .entity_mut(&player)
            .leave_room(&self.lobby_key)
            .enter_room(&room_key);

        server
            .user_mut(&user_key)
            .leave_room(&self.lobby_key)
            .enter_room(&room_key);

        (player, room)
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

    pub fn to_lobby<'world, 'state>(
        &mut self,
        server: &mut Server<'world, 'state, Protocol, DefaultChannels>,
        user_key: UserKey,
    ) -> Option<Entity> {
        let mut res = None;
        for (room_key, room) in &mut self.rooms {
            if let Some(player) = room.players.remove(&user_key) {
                res = Some(player);
                self.players.insert(user_key, player);
                server
                    .entity_mut(&player)
                    .leave_room(&room_key)
                    .enter_room(&self.lobby_key);

                server
                    .user_mut(&user_key)
                    .leave_room(&room_key)
                    .enter_room(&self.lobby_key);
            }
        }

        self.tidy(server);
        res
    }
}
