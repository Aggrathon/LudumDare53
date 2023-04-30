mod level0;
mod level1;
mod level2;
mod level3;
mod level_test;
mod ui;
use crate::camera::move_camera_to;
use crate::deck::Deck;
use crate::objective::ObjectiveTile;
use crate::state::GameState;
use crate::tile::Tile;
use crate::world::WorldMap;
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(level_test::LevelPlugin)
            .add_plugin(level0::LevelPlugin)
            .add_plugin(level1::LevelPlugin)
            .add_plugin(level2::LevelPlugin)
            .add_plugin(level3::LevelPlugin);

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
                            ui::on_rotate,
                            ui::on_victory,
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
    obj: Query<Entity, With<ObjectiveTile>>,
    camera: Query<&mut Transform, With<Camera>>,
) {
    for entity in &tiles {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &ui {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &obj {
        commands.entity(entity).despawn_recursive();
    }
    move_camera_to(camera, Vec2::ZERO);
    commands.insert_resource(WorldMap::default());
    commands.insert_resource(Deck::default());
}
