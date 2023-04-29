use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::colors;

#[derive(Default, Component, Debug)]
pub struct Tile {
    pub placed: bool,
    pub top: Border,
    pub right: Border,
    pub bottom: Border,
    pub left: Border,
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub enum Border {
    #[default]
    Any,
    Empty,
    Road,
}

impl Tile {
    pub fn has_road(&self) -> bool {
        self.top == Border::Road
            || self.right == Border::Road
            || self.bottom == Border::Road
            || self.left == Border::Road
    }

    pub fn empty() -> Self {
        Self {
            placed: false,
            top: Border::Empty,
            right: Border::Empty,
            bottom: Border::Empty,
            left: Border::Empty,
        }
    }

    pub fn create(pattern: &'static str) -> Self {
        let mut new = Self::empty();
        for v in pattern.as_bytes() {
            match v {
                b't' => new.top = Border::Road,
                b'r' => new.right = Border::Road,
                b'b' => new.bottom = Border::Road,
                b'l' => new.left = Border::Road,
                _ => {}
            }
        }
        new
    }

    pub fn rotate270(&self) -> Self {
        Self {
            placed: self.placed,
            top: self.right,
            right: self.bottom,
            bottom: self.left,
            left: self.top,
        }
    }

    pub fn rotate90(&self) -> Self {
        Self {
            placed: self.placed,
            top: self.left,
            right: self.top,
            bottom: self.right,
            left: self.bottom,
        }
    }

    pub fn rotate180(&self) -> Self {
        Self {
            placed: self.placed,
            top: self.bottom,
            right: self.left,
            bottom: self.top,
            left: self.right,
        }
    }

    pub fn placeable(&self, other: &Self) -> bool {
        !other.placed
            && (other.top == Border::Any || self.top == other.top)
            && (other.right == Border::Any || self.right == other.right)
            && (other.bottom == Border::Any || self.bottom == other.bottom)
            && (other.left == Border::Any || self.left == other.left)
    }
}

impl From<&Tile> for u32 {
    fn from(value: &Tile) -> Self {
        (value.top as u32)
            + (value.right as u32) * (1 << 4)
            + (value.bottom as u32) * (1 << 8)
            + (value.left as u32) * (1 << 16)
    }
}

impl From<Tile> for u32 {
    fn from(value: Tile) -> Self {
        (value.top as u32)
            + (value.right as u32) * (1 << 4)
            + (value.bottom as u32) * (1 << 8)
            + (value.left as u32) * (1 << 16)
    }
}

#[derive(Bundle)]
pub struct TileBundle {
    tile: Tile,
    sprite: SpriteBundle,
}

impl TileBundle {
    pub fn new(x: i32, y: i32) -> Self {
        let transform = Transform::from_translation(Vec3::new(x as f32, y as f32, 0.0));
        let sprite = Sprite {
            color: colors::light_green(),
            flip_x: false,
            flip_y: false,
            custom_size: Some(Vec2 { x: 1., y: 1. }),
            ..default()
        };
        Self {
            tile: Tile::default(),
            sprite: SpriteBundle {
                sprite,
                transform,
                ..default()
            },
        }
    }
}

#[derive(Resource, Default)]
pub struct TileServer(HashMap<u32, Handle<Image>>);

impl TileServer {
    pub fn find_texture(&self, tile: &Tile) -> (&Handle<Image>, f32) {
        if let Some(s) = self.0.get(&tile.into()) {
            return (s, 0.);
        }
        if let Some(s) = self.0.get(&tile.rotate90().into()) {
            return (s, PI * 0.5);
        }
        if let Some(s) = self.0.get(&tile.rotate180().into()) {
            return (s, PI);
        }
        if let Some(s) = self.0.get(&tile.rotate270().into()) {
            return (s, -PI * 0.5);
        }
        panic!("Could not find a tile: {:?}", tile);
    }

    pub fn load_assets(asset_server: Res<AssetServer>, mut ts: ResMut<TileServer>) {
        ts.0.insert(Tile::create("t").into(), asset_server.load("tile_t.png"));
        ts.0.insert(Tile::create("tr").into(), asset_server.load("tile_tr.png"));
        ts.0.insert(Tile::create("tb").into(), asset_server.load("tile_tb.png"));
        ts.0.insert(
            Tile::create("trb").into(),
            asset_server.load("tile_trb.png"),
        );
        ts.0.insert(
            Tile::create("trbl").into(),
            asset_server.load("tile_trbl.png"),
        );
    }
}

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TileServer>()
            .add_startup_system(TileServer::load_assets);
    }
}
