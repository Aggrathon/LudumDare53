use bevy::prelude::*;
use bevy::utils::HashMap;

use crate::tile::{self, Border, Network, Tile};

#[derive(Default, Resource)]
pub struct WorldMap {
    pub map: HashMap<(i32, i32), Entity>,
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

    pub fn sanitise_map(
        &self,
        mut net: ResMut<Network>,
        mut query: Query<(&mut Tile, &mut Network)>,
    ) {
        for ((x, y), e) in self.map.iter() {
            let mut tile = Tile::default();
            let mut network = Network::default();
            if let Some(e) = self.map.get(&(*x, *y + 1)) {
                if let Ok((t2, n2)) = query.get(*e) {
                    tile.top = t2.bottom;
                    if tile.top == Border::Road {
                        network.join(n2);
                    }
                }
            } else {
                tile.top = Border::Empty;
            }
            if let Some(e) = self.map.get(&(*x + 1, *y)) {
                if let Ok((t2, n2)) = query.get(*e) {
                    tile.right = t2.left;
                    if tile.right == Border::Road {
                        network.join(n2);
                    }
                }
            } else {
                tile.right = Border::Empty;
            }
            if let Some(e) = self.map.get(&(*x, *y - 1)) {
                if let Ok((t2, n2)) = query.get(*e) {
                    tile.bottom = t2.top;
                    if tile.bottom == Border::Road {
                        network.join(n2);
                    }
                }
            } else {
                tile.bottom = Border::Empty;
            }
            if let Some(e) = self.map.get(&(*x - 1, *y)) {
                if let Ok((t2, n2)) = query.get(*e) {
                    tile.left = t2.right;
                    if tile.left == Border::Road {
                        network.join(n2);
                    }
                }
            } else {
                tile.left = Border::Empty;
            }
            if let Ok((mut t2, mut n2)) = query.get_mut(*e) {
                if t2.has_road() {
                    if network.is_part() {
                        n2.join(&network);
                    } else {
                        *n2 = net.inc();
                    }
                }
                if t2.top == Border::Any {
                    t2.top = tile.top;
                }
                if t2.right == Border::Any {
                    t2.right = tile.right;
                }
                if t2.bottom == Border::Any {
                    t2.bottom = tile.bottom;
                }
                if t2.left == Border::Any {
                    t2.left = tile.left;
                }
            }
        }
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldMap>()
            .init_resource::<Network>()
            .add_startup_system(level0)
            .add_startup_system(sanitise_worldmap.after(level0));
    }
}

fn sanitise_worldmap(
    wm: Res<WorldMap>,
    net: ResMut<Network>,
    query: Query<(&mut Tile, &mut Network)>,
) {
    wm.sanitise_map(net, query);
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
