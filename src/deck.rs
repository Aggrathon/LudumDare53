use std::collections::VecDeque;
use std::f32::consts::PI;

use bevy::prelude::*;
use fastrand::Rng;

use crate::tile::Tile;

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Deck>();
    }
}

#[derive(Resource, Default)]
pub struct Deck {
    deck: Vec<(f32, Tile)>,
    pile: VecDeque<(Tile, f32)>,
}

impl Deck {
    pub fn add_tile(&mut self, tile: Tile, prob: f32) {
        self.deck.push((prob, tile));
    }

    pub fn add_all_tiles(&mut self) {
        self.deck.push((0.3, Tile::create("tr")));
        self.deck.push((0.3, Tile::create("tb")));
        self.deck.push((0.2, Tile::create("trb")));
        self.deck.push((0.1, Tile::create("trbl")));
        self.deck.push((0.1, Tile::create("t")));
    }

    pub fn fill_pile(&mut self, num: usize, seed: u64) {
        let rng = Rng::with_seed(seed);
        let tot: f32 = self.deck.iter().map(|(p, _)| p).sum();
        self.pile.reserve(num);
        for _ in 0..num {
            let mut r = rng.f32() * tot;
            for (p, t) in self.deck.iter() {
                if r < *p {
                    let rot = (rng.u8(0..4) as f32) * PI * 0.5;
                    self.pile.push_back((t.clone(), rot));
                } else {
                    r -= p;
                }
            }
        }
    }

    fn add_rnd_to_pile(&mut self) {
        let rng = Rng::new();
        let tot: f32 = self.deck.iter().map(|(p, _)| p).sum();
        let mut r = rng.f32() * tot;
        for (p, t) in self.deck.iter() {
            if r < *p {
                let rot = (rng.u8(0..4) as f32) * PI * 0.5;
                self.pile.push_back((t.clone(), rot));
            } else {
                r -= p;
            }
        }
    }

    pub fn add_to_pile(&mut self, tile: Tile, rotation: f32) {
        self.pile.push_back((tile, rotation));
    }

    pub fn get_top(&mut self) -> (Tile, f32) {
        match self.pile.front() {
            Some(v) => v.clone(),
            None => {
                self.add_rnd_to_pile();
                self.pile.front().unwrap().clone()
            }
        }
    }

    pub fn next(&mut self) -> (Tile, f32) {
        self.pile.pop_front();
        self.get_top()
    }
}
