use bevy::{
    math::Vec2,
    prelude::{App, Plugin, Res, ResMut, Resource},
    window::Windows,
};

pub const GRID_SZE: usize = 10;

#[derive(Default, Resource)]
pub struct Dimensions {
    pub block: f32,
    pub height: f32,
    pub size: f32,
    pub width: f32,
}

impl Dimensions {
    pub fn translate(&self, x: usize, y: usize) -> Vec2 {
        let offs = 0.5 * self.size - 0.5 * self.block;
        Vec2::new(self.block * x as f32 - offs, offs - self.block * y as f32)
    }
}

pub struct DimensionsPlugin;
impl Plugin for DimensionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update)
            .insert_resource(Dimensions::default());
    }
}

fn update(mut dimensions: ResMut<Dimensions>, windows: Res<Windows>) {
    if !windows.is_changed() {
        return;
    }

    let win = windows.get_primary().unwrap();
    let height = win.height();
    if dimensions.height != height {
        dimensions.height = height;
    }

    let width = win.width();
    if dimensions.width != width {
        dimensions.width = width;
    }

    let size = f32::min(height, width);
    if dimensions.size != size {
        dimensions.size = size;
    }

    let block = size / GRID_SZE as f32;
    if dimensions.block != block {
        dimensions.block = block;
    }
}
