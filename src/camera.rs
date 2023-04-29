use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;

use crate::colors;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);
    }
}

fn spawn_camera(mut cmds: Commands) {
    cmds.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(colors::dark_green()),
        },
        projection: OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::AutoMin {
                min_width: 10.,
                min_height: 10.,
            },
            ..default()
        },
        ..default()
    });
}
