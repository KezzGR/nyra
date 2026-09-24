use nyra::App;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new().expect("Failed to start EventLoop");
    let mut app = App::default();

    event_loop.run_app(&mut app).expect("Error in EventLoop");
}
