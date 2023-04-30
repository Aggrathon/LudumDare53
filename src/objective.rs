use bevy::prelude::*;

use crate::tile::{Border, Tile};
use crate::world::{PlaceTile, WorldMap};

pub struct ObjectivePlugin;

impl Plugin for ObjectivePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_victory).add_event::<Victory>();
    }
}

pub struct Victory;

fn check_victory(
    event: EventReader<PlaceTile>,
    mut victory: EventWriter<Victory>,
    wm: Res<WorldMap>,
    query: Query<&ObjectiveTile>,
    tiles: Query<&Tile>,
) {
    if !event.is_empty() {
        let connected = !query.is_empty()
            && query.iter().all(|ot| {
                if let Some(e) = wm.get_tile(ot.x, ot.y) {
                    let tile = tiles.get(e).expect("Could not find tile");
                    let top = tile.top == Border::Road
                        && wm
                            .get_tile(ot.x, ot.y + 1)
                            .map_or(false, |e| tiles.get(e).expect("Could not find tile").placed);
                    let bottom = tile.bottom == Border::Road
                        && wm
                            .get_tile(ot.x, ot.y - 1)
                            .map_or(false, |e| tiles.get(e).expect("Could not find tile").placed);
                    let right = tile.right == Border::Road
                        && wm
                            .get_tile(ot.x + 1, ot.y)
                            .map_or(false, |e| tiles.get(e).expect("Could not find tile").placed);
                    let left = tile.left == Border::Road
                        && wm
                            .get_tile(ot.x - 1, ot.y)
                            .map_or(false, |e| tiles.get(e).expect("Could not find tile").placed);
                    top || bottom || right || left
                } else {
                    false
                }
            });
        dbg!(connected);
        dbg!(query.is_empty());
        dbg!(query.iter().count());
        if connected {
            victory.send(Victory);
        }
    }
}

#[derive(Component, Debug)]
pub struct ObjectiveTile {
    x: i32,
    y: i32,
}

pub fn setup_start_tile(
    x: i32,
    y: i32,
    tile: Tile,
    cmds: &mut Commands,
    asset_server: &Res<AssetServer>,
    tile_placed: &mut EventWriter<PlaceTile>,
) {
    let mut offset = Vec3::ZERO;
    if tile.top == Border::Road {
        offset.y -= 1.;
    }
    if tile.right == Border::Road {
        offset.x -= 1.;
    }
    if tile.bottom == Border::Road {
        offset.y += 1.;
    }
    if tile.left == Border::Road {
        offset.x += 1.;
    }
    cmds.spawn((
        ObjectiveTile { x, y },
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::ONE),
                ..default()
            },
            transform: Transform::from_translation(
                Vec3::new(x as f32, y as f32, 1.) + offset.normalize() * 0.25,
            ),
            texture: asset_server.load("distribution_center.png"),
            ..default()
        },
    ));
    tile_placed.send(PlaceTile::new(x, y, tile));
}

pub fn setup_end_tile(
    x: i32,
    y: i32,
    tile: Tile,
    cmds: &mut Commands,
    asset_server: &Res<AssetServer>,
    tile_placed: &mut EventWriter<PlaceTile>,
) {
    let mut offset = Vec3::ZERO;
    if tile.top == Border::Road {
        offset.y -= 1.;
    }
    if tile.right == Border::Road {
        offset.x -= 1.;
    }
    if tile.bottom == Border::Road {
        offset.y += 1.;
    }
    if tile.left == Border::Road {
        offset.x += 1.;
    }
    cmds.spawn((
        ObjectiveTile { x, y },
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::ONE),
                ..default()
            },
            transform: Transform::from_translation(
                Vec3::new(x as f32, y as f32, 1.) + offset.normalize() * 0.3,
            ),
            texture: asset_server.load("house.png"),
            ..default()
        },
    ));
    tile_placed.send(PlaceTile::new_slient(x, y, tile));
}
