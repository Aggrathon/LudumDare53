use crate::deck::Deck;
use crate::objective::{setup_end_tile, setup_start_tile};
use crate::state::GameState;
use crate::tile::Tile;
use crate::world::{PlaceTile, WorldMap};
use bevy::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_deck.in_schedule(OnEnter(GameState::Level7)))
            .add_systems(
                (
                    setup_board,
                    apply_system_buffers,
                    WorldMap::apply_borders,
                    place_tiles,
                )
                    .chain()
                    .in_schedule(OnEnter(GameState::Level7)),
            );
    }
}

fn setup_board(mut cmds: Commands, mut wm: ResMut<WorldMap>) {
    let pattern = [
        b"###  #####  ##### @ #####  #####  ###",
        b"##    ###    ###     ###    ###    ##",
        b"#      #      #       #      #      #",
        b"#      #                     #      #",
        b"                                     ",
        b"#      #                     #      #",
        b"#      #      #       #      #      #",
        b"##    ###    ###     ###    ###    ##",
        b"###  #####  #####   #####  #####  ###",
        b"###  #####  #####   #####  #####  ###",
        b"##    ###    ###     ###    ###    ##",
        b"#      #      #       #      #      #",
        b"#      #      #       #      #      #",
        b"_                                   _",
        b"#      #      #       #      #      #",
        b"#      #      #       #      #      #",
        b"##    ###    ###     ###    ###    ##",
        b"###  #####  #####   #####  #####  ###",
    ];
    pattern.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, v)| {
            if *v != b'#' {
                // if *v == b'@' || *v == b'_' {
                //     dbg!((i, j));
                // }
                wm.create_tile(j as i32, i as i32 - 13, &mut cmds);
            }
        })
    });
}

fn place_tiles(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut tile_placed: EventWriter<PlaceTile>,
) {
    setup_start_tile(
        18,
        -13,
        Tile::create("trl"),
        &mut cmds,
        &asset_server,
        &mut tile_placed,
    );
    setup_end_tile(
        0,
        0,
        Tile::create("r"),
        &mut cmds,
        &asset_server,
        &mut tile_placed,
    );
    setup_end_tile(
        36,
        0,
        Tile::create("l"),
        &mut cmds,
        &asset_server,
        &mut tile_placed,
    );
}

fn setup_deck(mut deck: ResMut<Deck>) {
    deck.add_all_tiles();
    deck.add_tile(Tile::create("tb"), 0.1);
    deck.fill_pile(100, 45);
}
