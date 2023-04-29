mod camera;
mod colors;
mod tile;
mod world;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(tile::TilePlugin)
        .add_plugin(world::WorldPlugin)
        .run();
}
