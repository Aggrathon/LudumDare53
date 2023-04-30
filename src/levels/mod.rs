mod level_0;
mod ui;

use crate::deck::Deck;
use crate::state::GameState;
use crate::tile::Tile;
use crate::world::WorldMap;
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(level_0::Level0Plugin);

        for d in GameState::variants() {
            if d != GameState::MainMenu {
                app.add_system(clear_system.in_schedule(OnExit(d)))
                    .add_system(ui::setup_gui.in_schedule(OnEnter(d)))
                    .add_systems(
                        (
                            ui::key_system,
                            ui::button_menu,
                            ui::button_next,
                            ui::button_restart,
                            ui::button_rotate,
                            ui::update_tile,
                        )
                            .in_set(OnUpdate(d)),
                    );
            }
        }
    }
}

fn clear_system(
    mut commands: Commands,
    tiles: Query<Entity, With<Tile>>,
    ui: Query<Entity, With<ui::GameUI>>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    for entity in &tiles {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &ui {
        commands.entity(entity).despawn_recursive();
    }
    for mut tr in &mut camera {
        tr.translation = Vec3::ZERO;
    }
    commands.init_resource::<WorldMap>();
    commands.init_resource::<Deck>();
}
