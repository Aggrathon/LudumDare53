use crate::camera::move_camera_to;
use crate::deck::Deck;
use crate::objective::{setup_end_tile, setup_start_tile};
use crate::state::GameState;
use crate::tile::Tile;
use crate::world::{PlaceTile, WorldMap};
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_deck.in_schedule(OnEnter(GameState::LevelTest)))
            .add_system(move_camera.in_schedule(OnEnter(GameState::LevelTest)))
            .add_systems(
                (
                    setup_board,
                    apply_system_buffers,
                    WorldMap::apply_borders,
                    place_tiles,
                )
                    .chain()
                    .in_schedule(OnEnter(GameState::LevelTest)),
            );
    }
}

fn setup_board(mut cmds: Commands, mut wm: ResMut<WorldMap>) {
    wm.create_tile(-1, 1, &mut cmds);
    wm.create_tile(-1, 0, &mut cmds);
    wm.create_tile(0, 0, &mut cmds);
    wm.create_tile(1, 0, &mut cmds);
    wm.create_tile(1, -1, &mut cmds);

    for x in -4..=4 {
        for y in 2..5 {
            wm.create_tile(x, y, &mut cmds);
        }
    }
    wm.remove_tile(2, 3, &mut cmds);
}

fn place_tiles(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut tile_placed: EventWriter<PlaceTile>,
) {
    setup_start_tile(
        0,
        2,
        Tile::create("trl"),
        &mut cmds,
        &asset_server,
        &mut tile_placed,
    );
    setup_end_tile(
        -3,
        4,
        Tile::create("b"),
        &mut cmds,
        &asset_server,
        &mut tile_placed,
    );
}

fn move_camera(query: Query<&mut Transform, With<Camera>>) {
    move_camera_to(query, Vec2::new(2., 4.));
}

fn setup_deck(mut deck: ResMut<Deck>) {
    deck.add_all_tiles();
    deck.add_to_pile(Tile::create("lr"));
    deck.fill_pile(100, 42);
}
