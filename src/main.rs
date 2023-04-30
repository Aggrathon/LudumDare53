mod camera;
mod colors;
mod deck;
mod levels;
mod main_menu;
mod objective;
mod state;
mod tile;
mod ui;
mod world;

use bevy::prelude::*;
use bevy_easings::EasingsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EasingsPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(tile::TilePlugin)
        .add_plugin(ui::UiPlugin)
        .add_plugin(deck::DeckPlugin)
        .add_plugin(world::WorldPlugin)
        .add_state::<state::GameState>()
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(levels::LevelPlugin)
        .run();
}
