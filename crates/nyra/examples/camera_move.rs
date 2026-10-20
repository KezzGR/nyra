use glam::Quat;
use nyra::App;
use winit::error::EventLoopError;

fn main() -> Result<(), EventLoopError> {
    const MOUSE_SENSITIVITY: f32 = 0.002;
    const MOVE_SPEED: f32 = 5.0;

    let mut yaw = 0.0_f32;
    let mut pitch = 0.0_f32;

    App::new().run(move |game| {
        yaw += game.mouse.delta.x * MOUSE_SENSITIVITY;
        pitch -= game.mouse.delta.y * MOUSE_SENSITIVITY;

        pitch = pitch.clamp(-89.0_f32.to_radians(), 89.0_f32.to_radians());

        game.camera.transform.rotation = Quat::from_rotation_y(yaw) * Quat::from_rotation_x(pitch);

        game.camera
            .transform
            .translate_local(game.input.wasd_direction_3d() * game.delta_time * MOVE_SPEED);
    })
}
