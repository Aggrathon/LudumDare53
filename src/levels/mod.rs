mod level_0;

use crate::state::GameState;
use crate::tile::Tile;
use crate::world::WorldMap;
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<WorldMap>()
            .add_plugin(level_0::Level0Plugin);

        for d in GameState::variants() {
            if d != GameState::MainMenu {
                app.add_system(key_system.in_set(OnUpdate(d)))
                    .add_system(clear_system.in_schedule(OnExit(d)));
            }
        }
    }
}

fn key_system(
    mut keys: ResMut<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
) {
    if keys.just_pressed(KeyCode::R) {
        keys.reset(KeyCode::R);
        next_state.set(state.0);
    }
    if keys.just_pressed(KeyCode::N) {
        keys.reset(KeyCode::N);
        next_state.set(state.0.next_level());
    }
    if keys.just_pressed(KeyCode::Escape) {
        keys.reset(KeyCode::Escape);
        next_state.set(GameState::MainMenu);
    }
}

fn clear_system(
    mut commands: Commands,
    tiles: Query<Entity, With<Tile>>,
    mut camera: Query<&mut Transform, With<Camera>>,
) {
    for entity in &tiles {
        commands.entity(entity).despawn_recursive();
    }
    for mut tr in &mut camera {
        tr.translation = Vec3::ZERO;
    }
    commands.init_resource::<WorldMap>();
}
