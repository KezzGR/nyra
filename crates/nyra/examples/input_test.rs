use nyra::{App, KeyCode, RunError};

fn main() -> Result<(), RunError> {
    App::new().run_each_frame(|game| {
        if game.input.key_just_pressed(KeyCode::Space) {
            println!("Hello from Nyra!");
        }
    })
}
