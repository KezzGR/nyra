use nyra::{App, FreeFlyController};
use winit::error::EventLoopError;

fn main() -> Result<(), EventLoopError> {
    let mut controller = FreeFlyController::default();

    App::new().run(move |game| {
        controller.update(game);
    })
}
