use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    error::EventLoopError,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

use crate::{GameContext, Input, Renderer};

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    input: Option<Input>,
    update: Option<Box<dyn FnMut(&GameContext)>>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run<F>(mut self, update: F) -> Result<(), EventLoopError>
    where
        F: FnMut(&GameContext) + 'static,
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

        self.window = Some(window);
        self.renderer = Some(renderer);
        self.input = Some(input);
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
                if let Some(update) = self.update.as_mut() {
                    if let Some(input) = self.input.as_mut() {
                        update(&GameContext { input: input });
                        input.clear_inputs();
                    }
                }
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.render();
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
                if let Some(input) = self.input.as_mut() {
                    input.handle_keyboard_input(&event);
                }
            }

            _ => {}
        }
    }
}
