use bevy::{
    math::{Vec2, Vec3},
    prelude::{
        default, App, Changed, Color, Commands, Component, Entity, EventReader, Local, Plugin,
        Query, Res, ResMut, State, SystemSet, Transform, With, Without,
    },
    sprite::SpriteBundle,
};
use durakifa_protocol::protocol::{LeaveRoom, Name, Owner, Player, Protocol};
use naia_bevy_client::{shared::DefaultChannels, Client};

use crate::{AppState, ImageAssets};

use super::{
    dimensions::{Dimensions, GRID_SZE},
    menu::{Button, ButtonEvent},
};

const LEAVEGAME_TXT: &str = "LÉAVE GAMÉ";
const STARTGAME_TXT: &str = "555TART GAMÉ";

#[derive(Component)]
struct BtnLeave;

#[derive(Component)]
struct BtnStart;

#[derive(Component)]
struct Crown;

#[derive(Component)]
struct RoomComponent;

pub struct RoomPlugin;
impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Room).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::Room).with_system(cleanup))
            .add_system_set(
                SystemSet::on_update(AppState::Room)
                    .with_system(input)
                    .with_system(update_owner)
                    .with_system(update_player_names)
                    .with_system(update_players),
            );
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<RoomComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn input(
    mut app_state: ResMut<State<AppState>>,
    btn_leave: Query<&BtnLeave>,
    btn_start: Query<&BtnStart>,
    mut client: Client<Protocol, DefaultChannels>,
    mut event_reader: EventReader<ButtonEvent>,
) {
    for event in event_reader.iter() {
        if btn_leave.get(event.entity).is_ok() {
            app_state.set(AppState::Lobby).unwrap();
            client.send_message(DefaultChannels::UnorderedReliable, &LeaveRoom::new());
            return;
        }

        if btn_start.get(event.entity).is_ok() {
            app_state.set(AppState::Game).unwrap();
            return;
        }
    }
}

fn setup(mut commands: Commands, dimensions: Res<Dimensions>, images: Res<ImageAssets>) {
    commands
        .spawn(SpriteBundle {
            sprite: bevy::sprite::Sprite {
                custom_size: Some(Vec2::ONE),
                ..default()
            },
            texture: images.crown.clone(),
            transform: Transform::from_scale(Vec2::splat(dimensions.block).extend(Vec3::ONE.z)),
            ..default()
        })
        .insert(Crown)
        .insert(RoomComponent);

    commands
        .spawn_empty()
        .insert(BtnLeave)
        .insert(Button {
            color_bg: Color::MAROON,
            color_fg: Color::WHITE,
            position: GRID_SZE - 1,
            text: LEAVEGAME_TXT.to_string(),
        })
        .insert(RoomComponent);

    commands
        .spawn_empty()
        .insert(BtnStart)
        .insert(Button {
            color_bg: Color::DARK_GREEN,
            color_fg: Color::WHITE,
            position: GRID_SZE - 2,
            text: STARTGAME_TXT.to_string(),
        })
        .insert(RoomComponent);
}

fn update_owner(
    dimensions: Res<Dimensions>,
    mut crown: Query<&mut Transform, With<Crown>>,
    query: Query<&Button, With<Owner>>,
) {
    let mut tf = crown.single_mut();
    if dimensions.is_changed() {
        let scale = Vec2::splat(dimensions.block).extend(tf.scale.z);
        if tf.scale != scale {
            tf.scale = scale;
        }
    }

    if let Ok(btn) = query.get_single() {
        let translation = dimensions
            .translate(GRID_SZE - 1, btn.position)
            .extend(tf.translation.z)
            - 0.5 * dimensions.block * Vec3::X;

        if tf.translation != translation {
            tf.translation = translation;
        }
    }
}

fn update_player_names(
    client: Client<Protocol, DefaultChannels>,
    names: Query<(Entity, &Name), Changed<Name>>,
    mut query: Query<(&mut Button, &Player)>,
) {
    for (entity, name) in names.iter() {
        for (mut btn, player) in query.iter_mut() {
            if let Some(user) = player.user.get(&client) {
                if entity == user {
                    btn.text = (*name.name).clone();
                }
            }
        }
    }
}

fn update_players(
    mut buttons: Query<&mut Button>,
    client: Client<Protocol, DefaultChannels>,
    mut commands: Commands,
    names: Query<&Name>,
    mut players: Local<Vec<Entity>>,
    query: Query<(Entity, &Player), Without<Button>>,
) {
    let len = players.len();
    players.retain(|entity| buttons.contains(*entity));
    if players.len() != len {
        for (i, entity) in players.iter().enumerate() {
            buttons.get_mut(*entity).unwrap().position = i;
        }
    }

    for (entity, player) in query.iter() {
        if let Some(user) = player.user.get(&client) {
            if let Ok(name) = names.get(user) {
                commands.entity(entity).insert(Button {
                    color_bg: Color::MIDNIGHT_BLUE,
                    color_fg: Color::PINK,
                    position: players.len(),
                    text: (*name.name).clone(),
                });

                players.push(entity);
            }
        }
    }
}
