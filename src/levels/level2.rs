use crate::deck::Deck;
use crate::objective::{setup_end_tile, setup_start_tile};
use crate::state::GameState;
use crate::tile::Tile;
use crate::ui::big_button_text;
use crate::world::{PlaceTile, WorldMap};
use bevy::prelude::*;

use super::ui::GameUI;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_deck.in_schedule(OnEnter(GameState::Level2)))
            .add_systems(
                (
                    setup_board,
                    apply_system_buffers,
                    WorldMap::apply_borders,
                    place_tiles,
                )
                    .chain()
                    .in_schedule(OnEnter(GameState::Level2)),
            );
    }
}

fn setup_board(mut cmds: Commands, mut wm: ResMut<WorldMap>) {
    for x in -1..=1 {
        for y in 0..3 {
            wm.create_tile(x, y, &mut cmds);
        }
    }
    wm.create_tile(2, 0, &mut cmds);
}

fn place_tiles(
    mut cmds: Commands,
    asset_server: Res<AssetServer>,
    mut tile_placed: EventWriter<PlaceTile>,
) {
    setup_start_tile(
        -1,
        0,
        Tile::create("tr"),
        &mut cmds,
        &asset_server,
        &mut tile_placed,
    );
    setup_end_tile(
        2,
        0,
        Tile::create("l"),
        &mut cmds,
        &asset_server,
        &mut tile_placed,
    );
    let font = asset_server.load("Bungee-Regular.ttf");
    let mut text = big_button_text("Reset the level by pressing R", font);
    text.style.position = UiRect::all(Val::Px(10.));
    text.style.position_type = PositionType::Absolute;
    text.text.alignment = TextAlignment::Center;
    cmds.spawn((GameUI, text));
}

fn setup_deck(mut deck: ResMut<Deck>) {
    deck.add_tile(Tile::create("tr"), 1.0);
    deck.add_to_pile(Tile::create("lr"));
    deck.fill_pile(20, 42);
}
