use nyra::{App, FreeFlyController};
use winit::error::EventLoopError;

fn main() -> Result<(), EventLoopError> {
    let mut controller = FreeFlyController::default();

    App::new()
        .setup(|game| {
            game.cursor.set_locked(true);
            game.cursor.set_visible(false);
        })
        .run(move |game| {
            controller.update(game);
        })
}
