use nyra::{App, FreeFlyController, RunError};

fn main() -> Result<(), RunError> {
    let mut controller = FreeFlyController::default();

    App::new().run(move |game| {
        controller.update(game);
    })
}
