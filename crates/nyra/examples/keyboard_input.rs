use nyra::{App, KeyCode, RunError};

fn main() -> Result<(), RunError> {
    println!("Press Space or W in the Nyra window.");

    App::new().run_each_frame(|ctx| {
        if ctx.input.key_just_pressed(KeyCode::Space) {
            println!("Space pressed");
        }

        if ctx.input.key_just_released(KeyCode::Space) {
            println!("Space released");
        }

        if ctx.input.key_just_pressed(KeyCode::KeyW) {
            println!("W pressed");
        }
    })
}
