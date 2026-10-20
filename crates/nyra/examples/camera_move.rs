use glam::Vec3;
use nyra::App;
use winit::{error::EventLoopError, keyboard::KeyCode};

fn main() -> Result<(), EventLoopError> {
    const MOVE_SPEED: f32 = 3.5;

    App::new().run(move |game| {
        let mut direction = Vec3::default();

        if game.input.key_just_pressed(KeyCode::KeyW) {
            direction += Vec3::Z;
        }
        if game.input.key_just_pressed(KeyCode::KeyA) {
            direction += Vec3::NEG_X;
        }
        if game.input.key_just_pressed(KeyCode::KeyS) {
            direction += Vec3::NEG_Z;
        }
        if game.input.key_just_pressed(KeyCode::KeyD) {
            direction += Vec3::X;
        }

        if game.input.key_just_released(KeyCode::KeyW) {
            direction -= Vec3::Z;
        }
        if game.input.key_just_released(KeyCode::KeyA) {
            direction -= Vec3::NEG_X;
        }
        if game.input.key_just_released(KeyCode::KeyS) {
            direction -= Vec3::NEG_Z;
        }
        if game.input.key_just_released(KeyCode::KeyD) {
            direction -= Vec3::X;
        }

        game.camera
            .transform
            .translate(direction.normalize_or_zero() * game.delta_time * MOVE_SPEED);
    })
}
