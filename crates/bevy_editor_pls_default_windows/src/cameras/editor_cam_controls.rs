use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_editor_pls_core::EditorState;

pub struct FlycamPlugin;
impl Plugin for FlycamPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(camera_movement.label(CameraSystem::Movement))
            .add_system(camera_look)
            .add_system(toggle_cursor);
    }
}

#[derive(SystemLabel, PartialEq, Eq, Clone, Hash, Debug)]
pub enum CameraSystem {
    Movement,
}

#[derive(Component)]
pub struct Flycam {
    pub yaw: f32,
    pub pitch: f32,
    pub sensitivity: f32,
    pub enabled: bool,
    pub was_initially_positioned: bool,
}
impl Default for Flycam {
    fn default() -> Self {
        Self {
            yaw: Default::default(),
            pitch: Default::default(),
            sensitivity: 6.0,
            enabled: false,
            was_initially_positioned: false,
        }
    }
}

fn camera_movement(
    mut cam: Query<(&Flycam, &mut Transform)>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let (flycam, mut cam_transform) = cam.single_mut();
    if !flycam.enabled {
        return;
    }

    let if_then_1 = |b| if b { 1.0 } else { 0.0 };
    let forward = if_then_1(keyboard_input.pressed(KeyCode::W))
        - if_then_1(keyboard_input.pressed(KeyCode::S));
    let sideways = if_then_1(keyboard_input.pressed(KeyCode::D))
        - if_then_1(keyboard_input.pressed(KeyCode::A));
    let up = if_then_1(keyboard_input.pressed(KeyCode::Space))
        - if_then_1(keyboard_input.pressed(KeyCode::LControl));

    if forward == 0.0 && sideways == 0.0 && up == 0.0 {
        return;
    }

    let speed = if keyboard_input.pressed(KeyCode::LShift) {
        20.0
    } else {
        5.0
    };

    let movement =
        Vec3::new(sideways, forward, up).normalize_or_zero() * speed * time.delta_seconds();

    let diff = cam_transform.forward() * movement.y
        + cam_transform.right() * movement.x
        + cam_transform.up() * movement.z;
    cam_transform.translation += diff;
}

fn camera_look(
    time: Res<Time>,
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_motion_event_reader: EventReader<MouseMotion>,
    mut query: Query<(&mut Flycam, &mut Transform)>,
) {
    let (mut flycam, mut transform) = query.single_mut();
    if !mouse_input.pressed(MouseButton::Right) {
        return;
    }
    if !flycam.enabled {
        return;
    }
    let mut delta: Vec2 = Vec2::ZERO;
    for event in mouse_motion_event_reader.iter() {
        delta += event.delta;
    }
    if delta.is_nan() || delta.abs_diff_eq(Vec2::ZERO, f32::EPSILON) {
        return;
    }

    flycam.yaw -= delta.x * flycam.sensitivity * time.delta_seconds();
    flycam.pitch -= delta.y * flycam.sensitivity * time.delta_seconds();

    flycam.pitch = flycam.pitch.clamp(-89.0, 89.9);
    // println!("pitch: {}, yaw: {}", options.pitch, options.yaw);

    let yaw_radians = flycam.yaw.to_radians();
    let pitch_radians = flycam.pitch.to_radians();

    transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw_radians, pitch_radians, 0.0);
}

fn toggle_cursor(
    keyboard_input: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
    editor_state: Res<EditorState>,
) {
    let window = windows.get_primary_mut().unwrap();

    if !editor_state.active {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::LAlt) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }
    if keyboard_input.just_released(KeyCode::LAlt) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}
