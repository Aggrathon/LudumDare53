use crate::tile::{self, Border, SelectTile, SelectTileBundle, Tile, TileServer};
use bevy::prelude::*;
use bevy::utils::HashMap;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldMap>()
            .add_event::<PlaceTile>()
            .add_system(handle_open_tiles)
            .add_system(place_tile);
    }
}

pub struct PlaceTile {
    pub x: i32,
    pub y: i32,
    pub tile: Tile,
    pub silent: bool,
}

impl PlaceTile {
    pub fn new(x: i32, y: i32, tile: Tile) -> Self {
        Self {
            x,
            y,
            tile,
            silent: false,
        }
    }

    pub fn new_slient(x: i32, y: i32, tile: Tile) -> Self {
        Self {
            x,
            y,
            tile,
            silent: true,
        }
    }

    pub fn dummy() -> Self {
        Self {
            x: 0,
            y: i32::MAX,
            silent: true,
            tile: default(),
        }
    }
}

#[derive(Default, Resource)]
pub struct WorldMap {
    map: HashMap<(i32, i32), Entity>,
}

impl WorldMap {
    pub fn create_tile(&mut self, x: i32, y: i32, cmds: &mut Commands) -> Entity {
        let entity = cmds.spawn(tile::TileBundle::new(x, y)).id();
        self.map
            .insert((x, y), entity)
            .map_or((), |e| cmds.entity(e).despawn());
        entity
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
}

fn place_tile(
    mut event: EventReader<PlaceTile>,
    wm: Res<WorldMap>,
    ts: Res<TileServer>,
    mut query: Query<(&mut Tile, &mut Sprite, &mut Transform, &mut Handle<Image>)>,
) {
    for ev in event.iter() {
        let entity = wm.map.get(&(ev.x, ev.y));
        if ev.silent && entity.is_none() {
            continue;
        }
        let entity = entity.expect("Tile does not exist");
        let (mut t, mut s, mut tr, mut h) =
            query.get_mut(*entity).expect("Could not find tile entity");
        assert!(
            ev.tile.placeable(&t),
            "Could not place tile x={} y={} tile={:?} t={:?}",
            ev.x,
            ev.y,
            ev.tile,
            &t
        );
        let (img, rot) = ts.find_texture(&ev.tile);
        *t = ev.tile.clone();
        t.placed = true;
        *h = img;
        tr.rotate_z(rot);
        s.color = Color::WHITE;
    }
}

fn handle_open_tiles(
    mut event: EventReader<PlaceTile>,
    wm: Res<WorldMap>,
    mut cmds: Commands,
    mut query: Query<&mut Tile>,
    sel_query: Query<(&Parent, Entity), With<SelectTile>>,
    asset_server: Res<AssetServer>,
) {
    for ev in event.iter() {
        let (x, y, tile) = (ev.x, ev.y, &ev.tile);
        let entity = wm.map.get(&(ev.x, ev.y));
        if ev.silent && entity.is_none() {
            continue;
        }
        let entity = entity.expect("Tile does not exist");
        if let Some(e) = wm.map.get(&(x + 1, y)) {
            let mut t = query.get_mut(*e).expect("Could not find entity");
            t.left = tile.right;
            if !ev.silent && !t.placed && tile.right == Border::Road {
                cmds.entity(*e).with_children(|p| {
                    p.spawn(SelectTileBundle::new(&asset_server));
                });
            }
        }
        if let Some(e) = wm.map.get(&(x - 1, y)) {
            let mut t = query.get_mut(*e).expect("Could not find entity");
            t.right = tile.left;
            if !ev.silent && !t.placed && tile.left == Border::Road {
                cmds.entity(*e).with_children(|p| {
                    p.spawn(SelectTileBundle::new(&asset_server));
                });
            }
        }
        if let Some(e) = wm.map.get(&(x, y + 1)) {
            let mut t = query.get_mut(*e).expect("Could not find entity");
            t.bottom = tile.top;
            if !ev.silent && !t.placed && tile.top == Border::Road {
                cmds.entity(*e).with_children(|p| {
                    p.spawn(SelectTileBundle::new(&asset_server));
                });
            }
        }
        if let Some(e) = wm.map.get(&(x, y - 1)) {
            let mut t = query.get_mut(*e).expect("Could not find entity");
            t.top = tile.bottom;
            if !ev.silent && !t.placed && tile.bottom == Border::Road {
                cmds.entity(*e).with_children(|p| {
                    p.spawn(SelectTileBundle::new(&asset_server));
                });
            }
        }
        for (p, e) in &sel_query {
            if p.get() == *entity {
                cmds.entity(*entity).remove_children(&[e]);
                cmds.entity(e).despawn();
            }
        }
    }
}
