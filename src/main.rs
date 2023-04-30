mod audio;
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
use bevy::window::WindowResolution;
use bevy_easings::EasingsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tiler's Trucking Co   --   Aggrathon   --   Ludum Dare 53".to_string(),
                resolution: WindowResolution::new(1280., 800.),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(EasingsPlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(tile::TilePlugin)
        .add_plugin(ui::UiPlugin)
        .add_plugin(deck::DeckPlugin)
        .add_plugin(world::WorldPlugin)
        .add_plugin(objective::ObjectivePlugin)
        .add_state::<state::GameState>()
        .add_plugin(audio::AudioPlugin)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(levels::LevelPlugin)
        .run();
}
