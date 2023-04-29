use std::time::Duration;

use crate::colors;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy_easings::*;

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

pub fn ease_camera_to(
    mut commands: Commands,
    query: Query<(&Transform, Entity), With<Camera>>,
    target: Vec3,
) {
    for (tr, e) in &query {
        commands.entity(e).insert(tr.ease_to(
            tr.with_translation(target),
            EaseFunction::QuadraticInOut,
            EasingType::Once {
                duration: Duration::from_millis(500),
            },
        ));
    }
}
