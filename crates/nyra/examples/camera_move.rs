use glam::Vec3;
use nyra::App;
use winit::{error::EventLoopError, keyboard::KeyCode};

fn main() -> Result<(), EventLoopError> {
    App::new().run(|game| {
        if game.input.key_just_pressed(KeyCode::KeyW) {
            game.camera.transform.translate(Vec3::Z);
        }
        if game.input.key_just_pressed(KeyCode::KeyA) {
            game.camera.transform.translate(Vec3::NEG_X);
        }
        if game.input.key_just_pressed(KeyCode::KeyS) {
            game.camera.transform.translate(Vec3::NEG_Z);
        }
        if game.input.key_just_pressed(KeyCode::KeyD) {
            game.camera.transform.translate(Vec3::X);
        }
    })
}
