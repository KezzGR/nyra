use nyra::{App, RunError};

fn main() -> Result<(), RunError> {
    App::new().with_title("Flappy").run()
}
