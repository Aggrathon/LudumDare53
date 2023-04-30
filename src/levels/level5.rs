use crate::deck::Deck;
use crate::objective::{setup_end_tile, setup_start_tile};
use crate::state::GameState;
use crate::tile::Tile;
use crate::world::{PlaceTile, WorldMap};
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_deck.in_schedule(OnEnter(GameState::Level5)))
            .add_systems(
                (
                    setup_board,
                    apply_system_buffers,
                    WorldMap::apply_borders,
                    place_tiles,
                )
                    .chain()
                    .in_schedule(OnEnter(GameState::Level5)),
            );
    }
}

fn setup_board(mut cmds: Commands, mut wm: ResMut<WorldMap>) {
    for x in -5..6 {
        for y in 0..10 {
            wm.create_tile(x, y, &mut cmds);
        }
    }
    for x in (-3..4).step_by(3) {
        for y in (4..9).step_by(3) {
            wm.remove_tile(x, y, &mut cmds);
        }
    }
}

fn place_tiles(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut tile_placed: EventWriter<PlaceTile>,
) {
    setup_start_tile(
        0,
        0,
        Tile::create("trl"),
        &mut cmds,
        &asset_server,
        &mut tile_placed,
    );
    setup_end_tile(
        3,
        9,
        Tile::create("rbl"),
        &mut cmds,
        &asset_server,
        &mut tile_placed,
    );
    setup_end_tile(
        -3,
        9,
        Tile::create("rbl"),
        &mut cmds,
        &asset_server,
        &mut tile_placed,
    );
}

fn setup_deck(mut deck: ResMut<Deck>) {
    deck.add_all_tiles();
    deck.fill_pile(100, 44);
}
