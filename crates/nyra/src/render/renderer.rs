use std::{sync::Arc, time::Instant};

use glam::{Quat, Vec3};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingType, Buffer, BufferAddress, BufferBindingType, BufferDescriptor,
    BufferUsages, Color, ColorTargetState, ColorWrites, CommandEncoderDescriptor,
    CurrentSurfaceTexture, Device, DeviceDescriptor, ExperimentalFeatures, Features, FragmentState,
    IndexFormat, Instance, Limits, LoadOp, MemoryHints, MultisampleState, Operations,
    PipelineCompilationOptions, PipelineLayoutDescriptor, PowerPreference, PrimitiveState,
    PrimitiveTopology, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, RequestAdapterOptions, ShaderModuleDescriptor, ShaderSource,
    ShaderStages, StoreOp, Surface, SurfaceConfiguration, TextureViewDescriptor, Trace,
    VertexState,
    util::{BufferInitDescriptor, DeviceExt},
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::render::{
    Camera, Transform, TransformUniform,
    mesh::{CUBE_INDEXES, CUBE_VERTICES, Vertex},
};

const ROTATION_SPEED: f32 = 2.0;

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
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    transform_buffer: Buffer,
    transform_bind_group: BindGroup,
    pipeline: RenderPipeline,
    last_frame: Instant,
    angle: f32,
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

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(CUBE_VERTICES),
            usage: BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(CUBE_INDEXES),
            usage: BufferUsages::INDEX,
        });

        let transform_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Transform Buffer"),
            size: size_of::<TransformUniform>() as BufferAddress,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let transform_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Transform Bind Group Layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let transform_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Transform Bind Group"),
            layout: &transform_bind_group_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: transform_buffer.as_entire_binding(),
            }],
        });

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Triangle Shader"),
            source: ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Triangle Pipeline Layout"),
            bind_group_layouts: &[Some(&transform_bind_group_layout)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Triangle Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[Some(Vertex::layout())],
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

        let last_frame = Instant::now();

        Self {
            window,
            surface,
            device,
            queue,
            config,
            vertex_buffer,
            index_buffer,
            transform_buffer,
            transform_bind_group,
            pipeline,
            last_frame,
            angle: 0.0,
        }
    }

    pub fn render(&mut self) {
        let now = Instant::now();
        let delta_time = (now - self.last_frame).as_secs_f32();
        self.last_frame = now;

        self.angle += delta_time * ROTATION_SPEED;

        let mut cube_transform = Transform::default();

        cube_transform.rotate_from_axis_angle(Vec3::ONE, self.angle);

        let camera_transform =
            Transform::new([0.0, 0.0, -5.0], Quat::IDENTITY.to_array(), [1.0, 1.0, 1.0]);

        let camera = Camera::new(camera_transform, 70.0, 0.01, 100.0);

        let model = cube_transform.model_matrix();
        let view = camera.view_matrix();
        let projection =
            camera.projection_matrix(self.config.width as f32 / self.config.height as f32);

        let projection_view_matrix = projection * view;

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
            render_pass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint16);
            render_pass.set_bind_group(0, Some(&self.transform_bind_group), &[]);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw_indexed(0..36, 0, 0..1);
        }

        self.queue.write_buffer(
            &self.transform_buffer,
            0,
            bytemuck::bytes_of(&TransformUniform {
                model: model.to_cols_array_2d(),
                projection_view: projection_view_matrix.to_cols_array_2d(),
            }),
        );
        self.queue.submit([encoder.finish()]);

        self.window.pre_present_notify();
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
