use std::sync::Arc;

use glam::{Quat, Vec2, Vec3};
use winit::{
    application::ApplicationHandler,
    error::EventLoopError,
    event::{DeviceEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

use crate::{
    GameContext, Input, Mouse, Renderer, Time,
    render::{Camera, Transform},
};

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    update: Option<Box<dyn FnMut(&mut GameContext)>>,
    input: Input,
    mouse: Mouse,
    camera: Camera,
    time: Time,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run<F>(mut self, update: F) -> Result<(), EventLoopError>
    where
        F: FnMut(&mut GameContext) + 'static,
    {
        self.update = Some(Box::new(update));

        let event_loop = EventLoop::new()?;

        event_loop.run_app(&mut self)
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window = Arc::new(
            event_loop
                .create_window(WindowAttributes::default().with_title("Nyra"))
                .expect("Error when creating the window"),
        );

        window.request_redraw();

        let renderer = pollster::block_on(Renderer::new(window.clone()));

        let input = Input::default();

        let mouse = Mouse::default();

        let camera_transform = Transform::new(Vec3::new(0.0, 0.0, -5.0), Quat::IDENTITY, Vec3::ONE);

        let camera = Camera::new(camera_transform, 70.0_f32.to_radians(), 0.01, 100.0);

        let time = Time::default();

        self.window = Some(window);
        self.renderer = Some(renderer);
        self.input = input;
        self.mouse = mouse;
        self.camera = camera;
        self.time = time;
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::RedrawRequested => {
                let dt = self.time.delta_time();

                if let Some(update) = self.update.as_mut() {
                    update(&mut GameContext {
                        input: &self.input,
                        mouse: &self.mouse,
                        camera: &mut self.camera,
                        delta_time: dt,
                    });
                    self.input.clear_inputs();
                    self.mouse.end_frame();
                }
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.render(&self.camera);
                }
                if let Some(window) = self.window.as_mut() {
                    window.request_redraw();
                }
            }

            WindowEvent::Resized(size) => {
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.resize(size);
                }
            }

            WindowEvent::KeyboardInput { event, .. } => {
                self.input.handle_keyboard_input(&event);
            }

            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta, .. } => {
                self.mouse
                    .save_delta(Vec2::new(delta.0 as f32, delta.1 as f32));
            }

            _ => {}
        }
    }
}
