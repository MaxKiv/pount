use bevy::{input::mouse::MouseWheel, prelude::*, render::camera::ScalingMode};

use crate::schedule::InGameSet;

const SCROLL_LINE_FACTOR: f32 = 0.2;
const SCROLL_PIXEL_FACTOR: f32 = 0.2;

#[derive(Component)]
pub struct CameraMarker;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
        app.add_systems(
            Update,
            (
                drag_camera,
                zoom_control_mouse_scroll,
                zoom_control_keyboard,
            )
                .chain()
                .in_set(InGameSet::MutateCamera),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    let projection = OrthographicProjection {
        near: -1000.0,
        far: 1000.0,
        viewport_origin: Vec2::new(0.0, 0.0),
        scaling_mode: ScalingMode::WindowSize(1.0),
        scale: 1.0,
        ..default()
    };

    commands.spawn((
        Camera2dBundle {
            projection,
            ..default()
        },
        CameraMarker,
    ));
}

fn drag_camera(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<&mut Transform, With<CameraMarker>>,
    windows: Query<&Window>,
    mut last_cursor_position: Local<Option<Vec2>>,
) {
    let window = windows.single();

    if let Some(cursor_position) = window.cursor_position() {
        if mouse_input.pressed(MouseButton::Left) {
            if let Some(last_position) = *last_cursor_position {
                let delta = cursor_position - last_position;

                // Move the camera
                for mut transform in query.iter_mut() {
                    transform.translation.x -= delta.x;
                    transform.translation.y += delta.y;
                }
            }
            *last_cursor_position = Some(cursor_position);
        } else {
            // Mouse was released, reset last mouse position
            *last_cursor_position = None;
        }
    }
}

fn zoom_control_mouse_scroll(
    mut evr_scroll: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<CameraMarker>>,
    mut last_camera_translation: Local<Option<Vec3>>,
) {
    use bevy::input::mouse::MouseScrollUnit;

    if (*last_camera_translation).is_none() {
        for transform in query.iter_mut() {
            *last_camera_translation = Some(transform.translation);
        }
    }

    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                for mut transform in query.iter_mut() {
                    transform.scale += ev.y * SCROLL_LINE_FACTOR;
                }
            }
            MouseScrollUnit::Pixel => {
                for mut transform in query.iter_mut() {
                    transform.translation.z += ev.y * SCROLL_PIXEL_FACTOR;
                }
            }
        }
    }
}

fn zoom_control_keyboard(
    input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut OrthographicProjection, With<CameraMarker>>,
) {
    let mut projection = camera_query.single_mut();

    if input.pressed(KeyCode::Minus) {
        projection.scale += 0.2;
    }

    if input.pressed(KeyCode::Equal) {
        projection.scale -= 0.2;
    }

    projection.scale = projection.scale.clamp(0.2, 5.);
}
