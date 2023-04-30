use crate::camera::cursor_to_world;
use crate::colors;
use crate::deck::Deck;
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::f32::consts::PI;

#[derive(Default, Component, Debug, Clone)]
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

#[derive(Component, Default)]
pub struct SelectTile {}

#[derive(Bundle)]
pub struct SelectTileBundle {
    tile: SelectTile,
    sprite: SpriteBundle,
}

impl SelectTileBundle {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        let sprite = Sprite {
            color: colors::yellow(),
            custom_size: Some(Vec2 { x: 1., y: 1. }),
            ..default()
        };
        let texture = asset_server.load("plus.png");
        Self {
            tile: SelectTile::default(),
            sprite: SpriteBundle {
                sprite,
                texture,
                ..default()
            },
        }
    }
}

fn update_select_tile(
    deck: Res<Deck>,
    mut query: Query<(&mut Sprite, &Parent, &GlobalTransform), With<SelectTile>>,
    tile_query: Query<&Tile>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<Camera>>,
) {
    let normal = colors::yellow();
    let hover = colors::orange();
    let disabled = colors::dark_green();
    if let Some(tile) = deck.get_top() {
        let cursor = cursor_to_world(windows, cameras).unwrap_or(Vec2 {
            x: f32::MAX,
            y: f32::MAX,
        });
        for (mut s, p, tr) in query.iter_mut() {
            let t = tile_query.get(p.get()).expect("Could not find parent");
            if tile.placeable(t) {
                if (tr.translation().truncate() - cursor).abs().max_element() < 0.5 {
                    s.color = hover;
                } else {
                    s.color = normal;
                }
            } else {
                s.color = disabled;
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct TileServer(HashMap<u32, Handle<Image>>);

impl TileServer {
    pub fn find_texture(&self, tile: &Tile) -> (Handle<Image>, f32) {
        if let Some(s) = self.0.get(&tile.into()) {
            return (s.clone(), 0.);
        }
        if let Some(s) = self.0.get(&tile.rotate90().into()) {
            return (s.clone(), PI * 0.5);
        }
        if let Some(s) = self.0.get(&tile.rotate180().into()) {
            return (s.clone(), PI);
        }
        if let Some(s) = self.0.get(&tile.rotate270().into()) {
            return (s.clone(), -PI * 0.5);
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
            .add_startup_system(TileServer::load_assets)
            .add_system(update_select_tile);
    }
}
