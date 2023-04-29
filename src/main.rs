mod camera;
mod colors;
mod levels;
mod menu;
mod state;
mod tile;
mod world;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(tile::TilePlugin)
        .add_state::<state::GameState>()
        .add_plugin(menu::MenuPlugin)
        .add_plugin(levels::LevelPlugin)
        .run();
}
