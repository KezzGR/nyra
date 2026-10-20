use nyra::App;
use winit::error::EventLoopError;

fn main() -> Result<(), EventLoopError> {
    const MOVE_SPEED: f32 = 3.5;

    App::new().run(|game| {
        game.camera
            .transform
            .translate(game.input.wasd_direction_3d() * game.delta_time * MOVE_SPEED);
    })
}
