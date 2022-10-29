use std::collections::HashMap;

use bevy::{
    input::Input,
    math::Vec3,
    prelude::{
        default, App, Changed, Color, Commands, Component, DetectChanges, Entity, EventWriter,
        MouseButton, Plugin, Query, Res, ResMut, SystemSet, Transform, With,
    },
    sprite::{Sprite, SpriteBundle},
    text::{Text, Text2dBundle, TextAlignment, TextStyle},
};

use crate::{AppState, FontAssets};

use super::{dimensions::Dimensions, mouse::Mouse};

#[derive(Component)]
pub struct Button {
    pub color_bg: Color,
    pub color_fg: Color,
    pub position: usize,
    pub text: String,
}

#[derive(Component)]
struct ButtonBounds {
    button: Entity,
    color: Color,
    position: usize,
}

pub struct ButtonEvent {
    pub entity: Entity,
}

#[derive(Component)]
struct ButtonLabel {
    button: Entity,
    color: Color,
    position: usize,
    text: String,
}

#[derive(Default)]
struct Global {
    buttons: HashMap<Entity, (Entity, Entity)>,
}

#[derive(Component)]
struct MenuComponent;

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        add_menu_to_state(app, AppState::Lobby);
        add_menu_to_state(app, AppState::Room);
        app.add_event::<ButtonEvent>();
    }
}

fn add_menu_to_state(app: &mut App, state: AppState) {
    app.add_system_set(SystemSet::on_enter(state.clone()).with_system(setup))
        .add_system_set(SystemSet::on_exit(state.clone()).with_system(cleanup))
        .add_system_set(
            SystemSet::on_update(state.clone())
                .with_system(input)
                .with_system(spawn_buttons)
                .with_system(update_bounds)
                .with_system(update_buttons)
                .with_system(update_labels),
        );
}

fn cleanup(mut commands: Commands, global: Res<Global>, query: Query<Entity, With<MenuComponent>>) {
    commands.remove_resource::<Global>();
    for entity in global.buttons.keys() {
        commands.entity(*entity).remove::<Button>();
    }

    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn input(
    mut event_writer: EventWriter<ButtonEvent>,
    mouse: Res<Mouse>,
    mut mouse_buttons: ResMut<Input<MouseButton>>,
    mut query: Query<(&ButtonBounds, &mut Sprite, &Transform)>,
) {
    if !mouse.is_changed() && !mouse_buttons.is_changed() {
        return;
    }

    for (btn, mut sp, tf) in query.iter_mut() {
        let sze = 0.5 * tf.scale.y;
        let bottom = tf.translation.y - sze;
        let top = tf.translation.y + sze;
        if mouse.y > bottom && mouse.y < top {
            if let Color::Hsla {
                hue,
                saturation,
                lightness,
                alpha,
            } = btn.color.as_hsla()
            {
                sp.color = Color::hsla(hue, saturation, lightness + 0.1, alpha);
                if mouse_buttons.pressed(MouseButton::Left) {
                    mouse_buttons.release(MouseButton::Left);
                    event_writer.send(ButtonEvent { entity: btn.button });
                }
            }
        } else {
            sp.color = btn.color;
        }
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(Global::default());
}

fn spawn_buttons(
    mut commands: Commands,
    fonts: Res<FontAssets>,
    mut global: ResMut<Global>,
    query: Query<(&Button, Entity)>,
) {
    global.buttons.retain(|btn, (bnd, lbl)| {
        let retain = query.contains(*btn);
        if !retain {
            commands.entity(*bnd).despawn();
            commands.entity(*lbl).despawn();
        }

        retain
    });

    for (btn, entity) in query.iter() {
        if global.buttons.contains_key(&entity) {
            continue;
        }

        global.buttons.insert(
            entity,
            (
                commands
                    .spawn_bundle(SpriteBundle::default())
                    .insert(ButtonBounds {
                        button: entity,
                        color: btn.color_bg,
                        position: btn.position,
                    })
                    .insert(MenuComponent)
                    .id(),
                commands
                    .spawn_bundle(Text2dBundle {
                        text: Text::from_section(
                            "",
                            TextStyle {
                                font: fonts.regular.clone(),
                                ..default()
                            },
                        )
                        .with_alignment(TextAlignment::CENTER),
                        transform: Transform::from_translation(Vec3::Z),
                        ..default()
                    })
                    .insert(ButtonLabel {
                        button: entity,
                        color: btn.color_fg,
                        position: btn.position,
                        text: btn.text.clone(),
                    })
                    .insert(MenuComponent)
                    .id(),
            ),
        );
    }
}

fn update_bounds(
    dimensions: Res<Dimensions>,
    mut query: Query<(&ButtonBounds, &mut Transform), With<Sprite>>,
) {
    for (btn, mut tf) in query.iter_mut() {
        let scale = dimensions.size * Vec3::X + dimensions.block * Vec3::Y + Vec3::Z;
        if tf.scale != scale {
            tf.scale = scale;
        }

        let translation = dimensions.translate(0, btn.position).y;
        if tf.translation.y != translation {
            tf.translation.y = translation;
        }
    }
}

fn update_buttons(
    mut bounds: Query<&mut ButtonBounds>,
    mut labels: Query<&mut ButtonLabel>,
    query: Query<(&Button, Entity), Changed<Button>>,
) {
    for (btn, entity) in query.iter() {
        for mut bnd in bounds.iter_mut() {
            if bnd.button != entity {
                continue;
            }

            if bnd.color != btn.color_bg {
                bnd.color = btn.color_bg;
            }

            if bnd.position != btn.position {
                bnd.position = btn.position;
            }
        }

        for mut lbl in labels.iter_mut() {
            if lbl.button != entity {
                continue;
            }

            if lbl.color != btn.color_fg {
                lbl.color = btn.color_fg;
            }

            if lbl.position != btn.position {
                lbl.position = btn.position;
            }

            if lbl.text != btn.text {
                lbl.text = btn.text.clone();
            }
        }
    }
}

fn update_labels(
    dimensions: Res<Dimensions>,
    mut query: Query<(&ButtonLabel, &mut Text, &mut Transform)>,
) {
    for (btn, mut txt, mut tf) in query.iter_mut() {
        if txt.sections[0].style.color != btn.color {
            txt.sections[0].style.color = btn.color;
        }

        let font_size = 0.25 * dimensions.block;
        if txt.sections[0].style.font_size != font_size {
            txt.sections[0].style.font_size = font_size;
        }

        let text = btn.text.clone();
        if txt.sections[0].value != text {
            txt.sections[0].value = text;
        }

        let translation = dimensions.translate(0, btn.position).y;
        if tf.translation.y != translation {
            tf.translation.y = translation;
        }
    }
}
