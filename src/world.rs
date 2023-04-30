use crate::tile::{self, Border, OpenTile, Tile, TileServer};
use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldMap>().add_event::<TilePlaced>();
    }
}

pub struct TilePlaced;

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
        cmds: &mut Commands,
        ts: Res<TileServer>,
        mut query: Query<(&mut Tile, &mut Sprite, &mut Transform, &mut Handle<Image>)>,
        mut tile_placed: EventWriter<TilePlaced>,
    ) {
        let entity = self.map.get(&(x, y)).expect("Tile does not exist");
        if let Some(e) = self.map.get(&(x + 1, y)) {
            let mut t = query.get_mut(*e).expect("Could not find entity").0;
            t.left = tile.right;
            if !t.placed {
                cmds.entity(*e).insert(OpenTile);
            }
        }
        if let Some(e) = self.map.get(&(x - 1, y)) {
            let mut t = query.get_mut(*e).expect("Could not find entity").0;
            t.right = tile.left;
            if !t.placed {
                cmds.entity(*e).insert(OpenTile);
            }
        }
        if let Some(e) = self.map.get(&(x, y + 1)) {
            let mut t = query.get_mut(*e).expect("Could not find entity").0;
            t.bottom = tile.top;
            if !t.placed {
                cmds.entity(*e).insert(OpenTile);
            }
        }
        if let Some(e) = self.map.get(&(x, y - 1)) {
            let mut t = query.get_mut(*e).expect("Could not find entity").0;
            t.top = tile.bottom;
            if !t.placed {
                cmds.entity(*e).insert(OpenTile);
            }
        }
        let (mut t, mut s, mut tr, mut h) =
            query.get_mut(*entity).expect("Could not find tile entity");
        assert!(tile.placeable(&t), "Could not place tile");
        let (img, rot) = ts.find_texture(&tile);
        *t = tile;
        t.placed = true;
        *h = img;
        tr.rotate_z(rot);
        s.color = Color::WHITE;
        cmds.entity(*entity).remove::<OpenTile>();
        tile_placed.send(TilePlaced);
    }
}
