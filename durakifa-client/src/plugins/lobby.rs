use bevy::prelude::{
    App, Changed, Color, Commands, Component, Entity, EventReader, Local, Plugin, Query, Res,
    ResMut, State, SystemSet, With, Without,
};
use durakifa_protocol::protocol::{Add, Join, Name, Protocol, Room};
use naia_bevy_client::{shared::DefaultChannels, Client};

use crate::{AppState, LocalPlayer};

use super::{
    dimensions::GRID_SZE,
    menu::{Button, ButtonEvent},
};

const NEWROOM_TXT: &str = "NÉW ROOM";

#[derive(Component)]
struct BtnNewRoom;

#[derive(Component)]
struct LobbyComponent;

pub struct LobbyPlugin;
impl Plugin for LobbyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Lobby).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::Lobby).with_system(cleanup))
            .add_system_set(
                SystemSet::on_update(AppState::Lobby)
                    .with_system(input)
                    .with_system(update_room_names)
                    .with_system(update_rooms),
            );
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<LobbyComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn input(
    mut app_state: ResMut<State<AppState>>,
    btn_new: Query<&BtnNewRoom>,
    btn_room: Query<&Button>,
    mut client: Client<Protocol, DefaultChannels>,
    mut event_reader: EventReader<ButtonEvent>,
    player: Res<LocalPlayer>,
    names: Query<&Name>,
) {
    for event in event_reader.iter() {
        if btn_new.get(event.entity).is_ok() {
            if let Some(local_player) = player.entity {
                app_state.set(AppState::Room).unwrap();
                //TODO: local_player is invalid if the player switched rooms.
                client.send_message(
                    DefaultChannels::UnorderedReliable,
                    &Add::new((*names.get(local_player).unwrap().name).clone()),
                );
            }

            return;
        }

        if btn_room.get(event.entity).is_ok() {
            app_state.set(AppState::Room).unwrap();
            let mut join = Join::new();
            join.room.set(&client, &event.entity);
            client.send_message(DefaultChannels::UnorderedReliable, &join);
            return;
        }
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn()
        .insert(BtnNewRoom)
        .insert(Button {
            color_bg: Color::DARK_GRAY,
            color_fg: Color::WHITE,
            position: GRID_SZE - 1,
            text: NEWROOM_TXT.to_string(),
        })
        .insert(LobbyComponent);
}

fn update_room_names(mut query: Query<(&mut Button, &Name), Changed<Name>>) {
    for (mut btn, name) in query.iter_mut() {
        btn.text = (*name.name).clone();
    }
}

fn update_rooms(
    mut buttons: Query<&mut Button>,
    mut commands: Commands,
    query: Query<(Entity, &Name), (With<Room>, Without<Button>)>,
    mut rooms: Local<Vec<Entity>>,
) {
    let len = rooms.len();
    rooms.retain(|entity| buttons.contains(*entity));
    if rooms.len() != len {
        for (i, entity) in rooms.iter().enumerate() {
            buttons.get_mut(*entity).unwrap().position = i;
        }
    }

    for (entity, name) in query.iter() {
        commands.entity(entity).insert(Button {
            color_fg: Color::YELLOW,
            color_bg: Color::MIDNIGHT_BLUE,
            position: rooms.len(),
            text: (*name.name).clone(),
        });

        rooms.push(entity);
    }
}
