use crate::deck::Deck;
use crate::state::GameState;
use crate::tile::Tile;
use crate::world::{PlaceTile, WorldMap};
use bevy::prelude::*;

pub struct Level0Plugin;

impl Plugin for Level0Plugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_deck.in_schedule(OnEnter(GameState::Level0)))
            .add_system(move_camera.in_schedule(OnEnter(GameState::Level0)))
            .add_systems(
                (
                    setup_board,
                    apply_system_buffers,
                    WorldMap::apply_borders,
                    place_tile,
                )
                    .chain()
                    .in_schedule(OnEnter(GameState::Level0)),
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

fn place_tile(mut tile_placed: EventWriter<PlaceTile>) {
    let tile = Tile::create("lr");
    tile_placed.send(PlaceTile::new(0, 0, tile));
}

fn move_camera(mut query: Query<&mut Transform, With<Camera>>) {
    for mut tr in &mut query {
        tr.translation = Vec3::new(2., 4., 0.);
    }
}

fn setup_deck(mut deck: ResMut<Deck>) {
    deck.add_all_tiles();
    deck.add_to_pile(Tile::create("lr"));
    deck.fill_pile(100, 42);
}
