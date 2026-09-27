use std::sync::Arc;

use wgpu::{
    Color, ColorTargetState, ColorWrites, CommandEncoderDescriptor, CurrentSurfaceTexture, Device,
    DeviceDescriptor, ExperimentalFeatures, Features, FragmentState, Instance, Limits, LoadOp,
    MemoryHints, MultisampleState, Operations, PipelineCompilationOptions,
    PipelineLayoutDescriptor, PowerPreference, PrimitiveState, PrimitiveTopology, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor,
    RequestAdapterOptions, ShaderModuleDescriptor, ShaderSource, StoreOp, Surface,
    SurfaceConfiguration, TextureViewDescriptor, Trace, VertexState,
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
    pipeline: RenderPipeline,
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

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Triangle Shader"),
            source: ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Triangle Pipeline Layout"),
            bind_group_layouts: &[],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Triangle Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[],
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });

        Self {
            window,
            surface,
            device,
            queue,
            config,
            pipeline,
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
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
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

            render_pass.set_pipeline(&self.pipeline);
            render_pass.draw(0..3, 0..1);
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
