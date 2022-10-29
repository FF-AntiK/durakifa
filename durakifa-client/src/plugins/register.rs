use bevy::{
    input::Input,
    prelude::{
        default, App, Color, Commands, Component, Entity, EventReader, KeyCode, Plugin, Query, Res,
        ResMut, State, SystemSet, Transform, With,
    },
    text::{Text, Text2dBundle, TextAlignment, TextStyle},
    window::ReceivedCharacter,
};
use durakifa_protocol::protocol::{Protocol, Register};
use naia_bevy_client::{shared::DefaultChannels, Client};

use crate::{AppState, FontAssets, InputState};

use super::{
    dimensions::Dimensions,
    vkeyboard::{Button, Key},
};

const NAMESZE: usize = 30;
const PROMPT: &str = "Enter your Agent ID:";

#[derive(Component)]
struct Name;

#[derive(Default)]
struct PlayerLocal {
    pub name: String,
}

#[derive(Component)]
struct Prompt;

#[derive(Component)]
struct RegisterComponent;

pub struct RegisterPlugin;
impl Plugin for RegisterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Register).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::Register).with_system(cleanup))
            .add_system_set(
                SystemSet::on_update(AppState::Register)
                    .with_system(input_keyboard)
                    .with_system(input_vkeyboard)
                    .with_system(update_name)
                    .with_system(update_prompt),
            )
            .insert_resource(PlayerLocal::default());
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<RegisterComponent>>) {
    commands.remove_resource::<PlayerLocal>();
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn input_keyboard(
    mut app_state: ResMut<State<AppState>>,
    mut client: Client<Protocol, DefaultChannels>,
    mut input: ResMut<Input<KeyCode>>,
    mut input_char: EventReader<ReceivedCharacter>,
    input_state: Res<InputState>,
    mut player: ResMut<PlayerLocal>,
) {
    if !vec![InputState::Keyboard].contains(&input_state) {
        return;
    }

    if input.pressed(KeyCode::Back) {
        input.release(KeyCode::Back);

        if player.name.len() > 0 {
            player.name.pop().unwrap();
        }

        return;
    }

    if input.pressed(KeyCode::Return) {
        input.release(KeyCode::Return);
        app_state.set(AppState::Lobby).unwrap();
        client.send_message(
            DefaultChannels::UnorderedReliable,
            &Register::new(player.name.clone()),
        );

        return;
    }

    for e in input_char.iter() {
        if !e.char.is_control() && player.name.len() <= NAMESZE {
            player.name.push(e.char);
        }
    }
}

fn input_vkeyboard(
    mut app_state: ResMut<State<AppState>>,
    mut client: Client<Protocol, DefaultChannels>,
    mut event_reader: EventReader<Button>,
    mut player: ResMut<PlayerLocal>,
) {
    for btn in event_reader.iter() {
        match btn.key {
            Key::Backspace => {
                if player.name.len() > 0 {
                    player.name.pop().unwrap();
                }
            }
            Key::Return => {
                app_state.set(AppState::Lobby).unwrap();
                client.send_message(
                    DefaultChannels::UnorderedReliable,
                    &Register::new(player.name.clone()),
                );
            }
            _ => player.name.push_str(btn.to_string().as_str()),
        }
    }
}

fn setup(mut commands: Commands, fonts: Res<FontAssets>) {
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                "",
                TextStyle {
                    color: Color::CYAN,
                    font: fonts.regular.clone(),
                    ..default()
                },
            )
            .with_alignment(TextAlignment::TOP_CENTER),
            ..default()
        })
        .insert(Name)
        .insert(RegisterComponent);

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(
                PROMPT,
                TextStyle {
                    color: Color::PINK,
                    font: fonts.regular.clone(),
                    ..default()
                },
            )
            .with_alignment(TextAlignment::TOP_CENTER),
            ..default()
        })
        .insert(Prompt)
        .insert(RegisterComponent);
}

fn update_name(
    dimensions: Res<Dimensions>,
    player: Res<PlayerLocal>,
    mut query: Query<(&mut Text, &mut Transform), With<Name>>,
) {
    if dimensions.is_changed() {
        for (mut txt, mut tf) in query.iter_mut() {
            tf.translation.y = dimensions.translate(0, 1).y;
            txt.sections[0].style.font_size = 0.25 * dimensions.block;
        }
    }

    if player.is_changed() {
        for (mut txt, _) in query.iter_mut() {
            txt.sections[0].value = player.name.clone();
        }
    }
}

fn update_prompt(
    dimensions: Res<Dimensions>,
    mut query: Query<(&mut Text, &mut Transform), With<Prompt>>,
) {
    if !dimensions.is_changed() {
        return;
    }

    for (mut txt, mut tf) in query.iter_mut() {
        tf.translation.y = dimensions.translate(0, 0).y;
        txt.sections[0].style.font_size = 0.25 * dimensions.block;
    }
}
