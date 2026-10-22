use std::sync::Arc;

use glam::{Quat, Vec2, Vec3};
use winit::{
    application::ApplicationHandler,
    error::EventLoopError,
    event::{DeviceEvent, ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::KeyCode,
    window::{Window, WindowAttributes, WindowId},
};

use crate::{
    Cursor, CursorState, GameContext, Input, Mouse, Renderer, Time, Transform, render::Camera,
    world::World,
};

pub struct App<S = ()> {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    setup: Option<Box<dyn FnOnce(&mut GameContext) -> S>>,
    update: Option<Box<dyn FnMut(&mut GameContext, &mut S)>>,
    state: Option<S>,
    input: Input,
    mouse: Mouse,
    camera: Camera,
    time: Time,
    cursor_state: CursorState,
    world: World,
}

impl App<()> {
    pub fn run<F>(mut self, mut update: F) -> Result<(), EventLoopError>
    where
        F: FnMut(&mut GameContext) + 'static,
    {
        if self.setup.is_none() {
            self.setup = Some(Box::new(|_| {}));
        }

        self.run_with_state(move |game, _state| {
            update(game);
        })
    }
}

impl<S> App<S> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn setup<F>(mut self, setup: F) -> Self
    where
        F: FnOnce(&mut GameContext) -> S + 'static,
    {
        self.setup = Some(Box::new(setup));
        self
    }

    pub fn run_with_state<F>(mut self, update: F) -> Result<(), EventLoopError>
    where
        F: FnMut(&mut GameContext, &mut S) + 'static,
        S: 'static,
    {
        self.update = Some(Box::new(update));

        let event_loop = EventLoop::new()?;

        event_loop.run_app(&mut self)
    }
}

impl<S> Default for App<S> {
    fn default() -> Self {
        Self {
            window: Default::default(),
            renderer: Default::default(),
            setup: Default::default(),
            update: Default::default(),
            state: Default::default(),
            input: Default::default(),
            mouse: Default::default(),
            camera: Default::default(),
            time: Default::default(),
            cursor_state: Default::default(),
            world: Default::default(),
        }
    }
}

impl<S> ApplicationHandler for App<S> {
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

        let camera_transform = Transform::new(Vec3::new(0.0, 0.0, -5.0), Quat::IDENTITY, Vec3::ONE);

        let camera = Camera::new(camera_transform, 70.0_f32.to_radians(), 0.01, 100.0);

        self.window = Some(window);
        self.renderer = Some(renderer);
        self.camera = camera;

        if let (Some(setup), Some(window)) = (self.setup.take(), self.window.as_ref()) {
            self.state = Some(setup(&mut GameContext {
                input: &self.input,
                mouse: &self.mouse,
                camera: &mut self.camera,
                world: &mut self.world,
                cursor: Cursor::new(window.as_ref(), &mut self.cursor_state),
                delta_time: 0.0,
            }));
        }
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

                if let (Some(update), Some(window), Some(state)) = (
                    self.update.as_mut(),
                    self.window.as_ref(),
                    self.state.as_mut(),
                ) {
                    update(
                        &mut GameContext {
                            input: &self.input,
                            mouse: &self.mouse,
                            camera: &mut self.camera,
                            world: &mut self.world,
                            cursor: Cursor::new(window.as_ref(), &mut self.cursor_state),
                            delta_time: dt,
                        },
                        state,
                    );
                }
                self.input.end_frame();
                self.mouse.end_frame();

                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.render(&self.camera, &self.world);
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

                if self.input.key_just_pressed(KeyCode::Escape) {
                    if let Some(window) = self.window.as_mut() {
                        self.cursor_state.on_escape(&window);
                    }
                }
            }

            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                if let Some(window) = self.window.as_mut() {
                    self.cursor_state.on_click(&window);
                }
            }

            WindowEvent::Focused(false) => {
                if let Some(window) = self.window.as_mut() {
                    self.cursor_state.on_escape(&window);
                }

                self.input.clear_inputs();
                self.mouse.end_frame();
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
                if let Some(window) = self.window.as_ref() {
                    if window.has_focus() {
                        self.mouse
                            .save_delta(Vec2::new(delta.0 as f32, delta.1 as f32));
                    }
                }
            }

            _ => {}
        }
    }
}
