use bevy::prelude::*;

use crate::tile::{Border, Tile};
use crate::world::{PlaceTile, WorldMap};

#[derive(Component)]
pub struct ObjectiveTile;

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
        ObjectiveTile,
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
        ObjectiveTile,
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
