use std::sync::Arc;

use wgpu::{
    Color, CommandEncoderDescriptor, CurrentSurfaceTexture, Device, DeviceDescriptor,
    ExperimentalFeatures, Features, Instance, Limits, LoadOp, MemoryHints, Operations,
    PowerPreference, Queue, RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions,
    StoreOp, Surface, SurfaceConfiguration, TextureViewDescriptor, Trace,
};
use winit::{dpi::PhysicalSize, window::Window};

const CLEAR_COLOR: Color = Color {
    r: 0.05,
    g: 0.05,
    b: 0.15,
    a: 1.0,
};

pub struct Renderer {
    window: Arc<Window>,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance = Instance::default();

        let surface = instance
            .create_surface(window.clone())
            .expect("Failed to create the Surface.");

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                compatible_surface: Some(&surface),
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                apply_limit_buckets: false,
            })
            .await
            .expect("We couldn’t find a suitable GPU.");

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: Some("Device"),
                required_features: Features::default(),
                required_limits: Limits::defaults(),
                experimental_features: ExperimentalFeatures::disabled(),
                memory_hints: MemoryHints::Performance,
                trace: Trace::Off,
            })
            .await
            .expect("Failed to obtain Device");

        let config = surface
            .get_default_config(&adapter, size.width.max(1), size.height.max(1))
            .expect("Failed to retrieve the Surface config");

        surface.configure(&device, &config);

        Self {
            window,
            surface,
            device,
            queue,
            config,
        }
    }

    pub fn render(&self) {
        let mut needs_reconfigure = false;

        let frame = match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(t) => t,

            CurrentSurfaceTexture::Suboptimal(t) => {
                needs_reconfigure = true;
                t
            }

            CurrentSurfaceTexture::Timeout => return,

            CurrentSurfaceTexture::Occluded => return,

            CurrentSurfaceTexture::Outdated => {
                self.reconfigure_surface();
                return;
            }

            CurrentSurfaceTexture::Lost => {
                self.reconfigure_surface();
                return;
            }

            CurrentSurfaceTexture::Validation => {
                panic!("wgpu surface validation error — see wgpu logs for details");
            }
        };

        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());

        {
            let _pass = encoder.begin_render_pass(&RenderPassDescriptor {
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(CLEAR_COLOR),
                        store: StoreOp::Store,
                    },
                })],

                ..Default::default()
            });
        }

        self.window.pre_present_notify();
        self.queue.submit([encoder.finish()]);
        self.queue.present(frame);

        if needs_reconfigure {
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width == 0 || size.height == 0 {
            return;
        }

        self.config.width = size.width;
        self.config.height = size.height;

        self.surface.configure(&self.device, &self.config);
    }

    fn reconfigure_surface(&self) {
        self.surface.configure(&self.device, &self.config);
        self.window.request_redraw();
    }
}
