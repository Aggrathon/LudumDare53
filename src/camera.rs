use std::time::Duration;

use crate::colors;
use crate::world::PlaceTile;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use bevy_easings::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
            .add_system(follow_placed);
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

fn follow_placed(
    commands: Commands,
    mut event: EventReader<PlaceTile>,
    query: Query<(&Transform, Entity), With<Camera>>,
) {
    for ev in event.iter() {
        if let Some((x, y, _)) = &ev.0 {
            ease_camera_to(
                commands,
                query,
                Vec3 {
                    x: *x as f32,
                    y: *y as f32,
                    z: 0.,
                },
            );
            break;
        }
    }
}

pub fn cursor_to_world(
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera>>,
) -> Option<Vec2> {
    let (camera, camera_transform) = camera.single();
    let window = windows.single();
    window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
}
