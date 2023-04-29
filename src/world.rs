use crate::tile::{self, Border, Tile, TileServer};
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Default, Resource)]
pub struct WorldMap {
    map: HashMap<(i32, i32), Entity>,
}

impl WorldMap {
    pub fn create_tile(&mut self, x: i32, y: i32, cmds: &mut Commands) -> Entity {
        let cmd = cmds.spawn(tile::TileBundle::new(x, y));
        self.map.insert((x, y), cmd.id());
        cmd.id()
    }

    pub fn remove_tile(&mut self, x: i32, y: i32, cmds: &mut Commands) -> bool {
        self.map
            .remove(&(x, y))
            .map(|e| cmds.entity(e).despawn())
            .is_some()
    }

    #[allow(unused)]
    pub fn get_tile(&self, x: i32, y: i32) -> Option<Entity> {
        self.map.get(&(x, y)).copied()
    }

    pub fn apply_borders(wm: Res<WorldMap>, mut query: Query<&mut Tile>) {
        for ((x, y), e) in wm.map.iter() {
            let mut tile = Tile::default();
            if !wm.map.contains_key(&(*x, *y + 1)) {
                tile.top = Border::Empty;
            }
            if !wm.map.contains_key(&(*x + 1, *y)) {
                tile.right = Border::Empty;
            }
            if !wm.map.contains_key(&(*x, *y - 1)) {
                tile.bottom = Border::Empty;
            }
            if !wm.map.contains_key(&(*x - 1, *y)) {
                tile.left = Border::Empty;
            }
            let mut t2 = query.get_mut(*e).expect("Could not find tile entity");
            *t2 = tile;
        }
    }

    pub fn set_tile(
        &self,
        x: i32,
        y: i32,
        tile: Tile,
        ts: Res<TileServer>,
        mut query: Query<(&mut Tile, &mut Sprite, &mut Transform, &mut Handle<Image>)>,
    ) {
        let entity = self.map.get(&(x, y)).expect("Tile does not exist");
        if let Some(e) = self.map.get(&(x + 1, y)) {
            query.get_mut(*e).expect("Could not find entity").0.left = tile.right;
        }
        if let Some(e) = self.map.get(&(x - 1, y)) {
            query.get_mut(*e).expect("Could not find entity").0.right = tile.left;
        }
        if let Some(e) = self.map.get(&(x, y + 1)) {
            query.get_mut(*e).expect("Could not find entity").0.bottom = tile.top;
        }
        if let Some(e) = self.map.get(&(x, y - 1)) {
            query.get_mut(*e).expect("Could not find entity").0.top = tile.bottom;
        }
        let (mut t, mut s, mut tr, mut h) =
            query.get_mut(*entity).expect("Could not find tile entity");
        assert!(tile.placeable(&t), "Could not place tile");
        let (img, rot) = ts.find_texture(&tile);
        *t = tile;
        *h = img.clone();
        tr.rotate_z(rot);
        s.color = Color::WHITE;
    }
}
