use std::collections::VecDeque;

use bevy::prelude::*;
use fastrand::Rng;

use crate::tile::Tile;

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Deck>().add_event::<TopTileRotated>();
    }
}

pub struct TopTileRotated(pub Tile);

#[derive(Resource, Default)]
pub struct Deck {
    deck: Vec<(f32, Tile)>,
    pile: VecDeque<Tile>,
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
                    self.pile.push_back(match rng.u8(0..4) {
                        1 => t.rotate90(),
                        2 => t.rotate180(),
                        3 => t.rotate270(),
                        _ => t.clone(),
                    });
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
                self.pile.push_back(match rng.u8(0..4) {
                    1 => t.rotate90(),
                    2 => t.rotate180(),
                    3 => t.rotate270(),
                    _ => t.clone(),
                });
            } else {
                r -= p;
            }
        }
    }

    pub fn add_to_pile(&mut self, tile: Tile) {
        self.pile.push_back(tile);
    }

    pub fn get_top(&self) -> Option<&Tile> {
        self.pile.front()
    }

    pub fn rotate(&mut self, mut event: EventWriter<TopTileRotated>) {
        if let Some(t) = self.pile.front_mut() {
            let t2 = t.rotate90();
            *t = t2.clone();
            event.send(TopTileRotated(t2));
        }
    }

    pub fn next(&mut self) -> &Tile {
        self.pile.pop_front();
        if self.pile.is_empty() {
            self.add_rnd_to_pile();
        }
        self.pile.front().unwrap()
    }
}
