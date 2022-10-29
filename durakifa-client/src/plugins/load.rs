use std::f32::consts::PI;

use bevy::{
    math::{Quat, Vec2},
    prelude::{
        default, App, AssetServer, Changed, Color, Commands, Component, Entity, Plugin, Query, Res,
        ResMut, State, SystemSet, Timer, Transform, With,
    },
    sprite::SpriteBundle,
    text::{Text, Text2dBundle, TextAlignment, TextStyle},
    time::Time,
};

use crate::{AppState, NetState};

use super::dimensions::Dimensions;

const CONTXT: &str = "Verbinde";
const LOADCLR: Color = Color::CYAN;
const LOADFNT: &str = "fonts/PressStart2P-vaV7.ttf";
const LOADIMG: &str = "images/load.png";
const LOADTXT: &str = "Lade";
const MARQUEE: [&str; 3] = [".  ", " . ", "  ."];
const MARQUEE_SPEED: f32 = 1.0; // Seconds per step
const ROTATION_SPEED: f32 = PI * 0.2; // Angle per Second

pub struct LoadPlugin;
impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Load).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::Connect).with_system(cleanup))
            .add_system_set(SystemSet::on_update(AppState::Connect).with_system(connect))
            .add_system_set(SystemSet::on_update(AppState::Connect).with_system(update_rotation))
            .add_system_set(
                SystemSet::on_update(AppState::Connect).with_system(update_transform_rotation),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Connect).with_system(update_transform_scale),
            )
            .add_system_set(SystemSet::on_update(AppState::Connect).with_system(update_text))
            .add_system_set(SystemSet::on_update(AppState::Load).with_system(update_rotation))
            .add_system_set(
                SystemSet::on_update(AppState::Load).with_system(update_transform_rotation),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Load).with_system(update_transform_scale),
            )
            .add_system_set(SystemSet::on_update(AppState::Load).with_system(update_text));
    }
}

#[derive(Component)]
struct LoadComponent;

#[derive(Component, Default)]
struct Rotation {
    angle: f32,
}

#[derive(Component)]
struct Marquee {
    step: usize,
    timer: Timer,
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<LoadComponent>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn connect(mut app_state: ResMut<State<AppState>>, net_state: Res<State<NetState>>) {
    if net_state.is_changed() && vec![NetState::Online].contains(net_state.current()) {
        app_state.set(AppState::Register).unwrap();
    }
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: bevy::sprite::Sprite {
                custom_size: Some(Vec2::ONE),
                ..Default::default()
            },
            texture: assets.load(LOADIMG),
            ..Default::default()
        })
        .insert(LoadComponent)
        .insert(Rotation::default());

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                LOADTXT,
                TextStyle {
                    color: LOADCLR,
                    font: assets.load(LOADFNT),
                    ..default()
                },
            )
            .with_alignment(TextAlignment::CENTER),
            ..default()
        })
        .insert(LoadComponent)
        .insert(Marquee {
            step: 0,
            timer: Timer::from_seconds(MARQUEE_SPEED, true),
        });
}

fn update_rotation(mut query: Query<&mut Rotation>, time: Res<Time>) {
    for mut rotation in query.iter_mut() {
        rotation.angle += time.delta().as_secs_f32() * ROTATION_SPEED;
    }
}

fn update_text(
    dimensions: Res<Dimensions>,
    mut query: Query<(&mut Marquee, &mut Text)>,
    state: Res<State<AppState>>,
    time: Res<Time>,
) {
    for (mut marquee, mut text) in query.iter_mut() {
        if !marquee.timer.tick(time.delta()).just_finished() {
            continue;
        }

        marquee.step += 1;
        if marquee.step >= MARQUEE.len() {
            marquee.step = 0;
        }

        let str = match state.current() {
            AppState::Connect => CONTXT,
            _ => LOADTXT,
        };

        text.sections[0].value = format!("{}{}", str, MARQUEE[marquee.step]);
        if dimensions.is_changed() {
            text.sections[0].style.font_size = dimensions.block;
        }
    }
}

fn update_transform_rotation(mut query: Query<(&Rotation, &mut Transform), Changed<Rotation>>) {
    for (rotation, mut transform) in query.iter_mut() {
        transform.rotation = Quat::from_rotation_z(rotation.angle);
    }
}

fn update_transform_scale(
    dimensions: Res<Dimensions>,
    mut query: Query<&mut Transform, With<Rotation>>,
) {
    if !dimensions.is_changed() {
        return;
    }

    for mut transform in query.iter_mut() {
        transform.scale = Vec2::splat(dimensions.size).extend(transform.scale.z);
    }
}
