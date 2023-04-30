use crate::deck::Deck;
use crate::objective::{setup_end_tile, setup_start_tile};
use crate::state::GameState;
use crate::tile::Tile;
use crate::world::{PlaceTile, WorldMap};
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_deck.in_schedule(OnEnter(GameState::Level4)))
            .add_systems(
                (
                    setup_board,
                    apply_system_buffers,
                    WorldMap::apply_borders,
                    place_tiles,
                )
                    .chain()
                    .in_schedule(OnEnter(GameState::Level4)),
            );
    }
}

fn setup_board(mut cmds: Commands, mut wm: ResMut<WorldMap>) {
    for x in -1..8 {
        for y in 0..3 {
            wm.create_tile(x, y, &mut cmds);
        }
    }
    for x in 2..4 {
        for y in 3..6 {
            wm.create_tile(x, y, &mut cmds);
        }
    }
    for x in 2..4 {
        for y in -3..0 {
            wm.create_tile(x, y, &mut cmds);
        }
    }
}

fn place_tiles(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut tile_placed: EventWriter<PlaceTile>,
) {
    setup_start_tile(
        -1,
        1,
        Tile::create("trb"),
        &mut cmds,
        &asset_server,
        &mut tile_placed,
    );
    setup_end_tile(
        3,
        5,
        Tile::create("bl"),
        &mut cmds,
        &asset_server,
        &mut tile_placed,
    );
    setup_end_tile(
        3,
        -3,
        Tile::create("tl"),
        &mut cmds,
        &asset_server,
        &mut tile_placed,
    );
}

fn setup_deck(mut deck: ResMut<Deck>) {
    deck.add_all_tiles();
    deck.fill_pile(100, 43);
}
