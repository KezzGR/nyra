use nyra::App;
use winit::{error::EventLoopError, keyboard::KeyCode};

fn main() -> Result<(), EventLoopError> {
    App::new().run(|game| {
        if game.input.key_just_pressed(KeyCode::Space) {
            println!("Hello from Nyra!");
        }
    })
}
