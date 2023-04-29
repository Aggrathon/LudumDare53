use bevy::prelude::*;

use crate::colors;

#[derive(Default, Component)]
pub struct Tile {
    pub top: Border,
    pub right: Border,
    pub bottom: Border,
    pub left: Border,
}

#[derive(Default, PartialEq, Clone, Copy)]
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

    pub fn create(pattern: &'static str) -> Self {
        let mut new = Self::default();
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
}

#[derive(Default, Clone, Copy, Component, Resource)]
pub struct Network(u16);

impl Network {
    pub fn inc(&mut self) -> Network {
        self.0 += 1;
        *self
    }

    pub fn join(&mut self, other: &Network) -> bool {
        if other.0 > self.0 {
            self.0 = other.0;
            true
        } else {
            false
        }
    }

    pub fn is_part(&self) -> bool {
        self.0 > 0
    }
}

#[derive(Bundle)]
pub struct TileBundle {
    tile: Tile,
    network: Network,
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
            network: Network::default(),
            sprite: SpriteBundle {
                sprite,
                transform,
                ..default()
            },
        }
    }
}
