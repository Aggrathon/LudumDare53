use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::tile::{self, Border, Tile, TileServer};

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

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldMap>().add_startup_systems(
            (
                level0,
                apply_system_buffers,
                WorldMap::apply_borders,
                place_tile,
            )
                .chain(),
        );
    }
}

fn level0(mut cmds: Commands, mut wm: ResMut<WorldMap>) {
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

fn place_tile(
    wm: Res<WorldMap>,
    ts: Res<TileServer>,
    query: Query<(&mut Tile, &mut Sprite, &mut Transform, &mut Handle<Image>)>,
) {
    let tile = Tile::create("lr");
    wm.set_tile(0, 0, tile, ts, query);
}
