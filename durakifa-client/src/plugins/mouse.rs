use bevy::{
    prelude::{App, Plugin, Res, ResMut, Resource},
    window::Windows,
};

#[derive(Default, Resource)]
pub struct Mouse {
    pub x: f32,
    pub y: f32,
}

pub struct MousePlugin;
impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update).insert_resource(Mouse::default());
    }
}

fn update(mut mouse: ResMut<Mouse>, windows: Res<Windows>) {
    if !windows.is_changed() {
        return;
    }

    let win = windows.get_primary().unwrap();
    if let Some(cursor) = win.cursor_position() {
        let x = cursor.x - 0.5 * win.width();
        let y = cursor.y - 0.5 * win.height();
        if mouse.x != x || mouse.y != y {
            mouse.x = cursor.x - 0.5 * win.width();
            mouse.y = cursor.y - 0.5 * win.height();
        }
    }
}
