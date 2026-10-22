use std::sync::Arc;
use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    BufferBindingType, Color, ColorTargetState, ColorWrites, CommandEncoderDescriptor,
    CompareFunction, CurrentSurfaceTexture, DepthBiasState, DepthStencilState, Device,
    DeviceDescriptor, ExperimentalFeatures, Extent3d, Features, FragmentState, IndexFormat,
    Instance, Limits, LoadOp, MemoryHints, MultisampleState, Operations,
    PipelineCompilationOptions, PipelineLayoutDescriptor, PowerPreference, PrimitiveState,
    PrimitiveTopology, Queue, RenderPassColorAttachment, RenderPassDepthStencilAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions,
    ShaderModuleDescriptor, ShaderSource, ShaderStages, StencilState, StoreOp, Surface,
    SurfaceConfiguration, Texture, TextureDescriptor, TextureDimension, TextureFormat,
    TextureUsages, TextureView, TextureViewDescriptor, Trace, VertexState,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    render::{Camera, GpuMesh, GpuThing, TransformUniform, Vertex},
    world::World,
};

const CLEAR_COLOR: Color = Color {
    r: 0.015,
    g: 0.019,
    b: 0.035,
    a: 1.0,
};

pub struct Renderer {
    window: Arc<Window>,
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    depth_texture: Texture,
    depth_view: TextureView,
    uniform_bind_group_layout: BindGroupLayout,
    pipeline: RenderPipeline,
    gpu_meshes: Vec<GpuMesh>,
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

        let depth_texture = device.create_texture(&TextureDescriptor {
            label: Some("Depth Texture"),
            size: Extent3d {
                width: config.width.max(1),
                height: config.height.max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[TextureFormat::Depth32Float],
        });

        let depth_view = depth_texture.create_view(&TextureViewDescriptor::default());

        let uniform_bind_group_layout =
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

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Triangle Shader"),
            source: ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Triangle Pipeline Layout"),
            bind_group_layouts: &[Some(&uniform_bind_group_layout)],
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
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: Some(true),
                depth_compare: Some(CompareFunction::Less),
                stencil: StencilState::default(),
                bias: DepthBiasState::default(),
            }),
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
            depth_texture,
            depth_view,
            uniform_bind_group_layout,
            pipeline,
            gpu_meshes: Vec::default(),
        }
    }

    pub fn render(&mut self, camera: &Camera, world: &World) {
        self.sync_meshes(world);

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
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &self.depth_view,
                    depth_ops: Some(Operations {
                        load: LoadOp::Clear(1.0),
                        store: StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });

            let view = camera.view_matrix();
            let projection =
                camera.projection_matrix(self.config.width as f32 / self.config.height as f32);

            let projection_view = projection * view;

            render_pass.set_pipeline(&self.pipeline);

            for thing in &world.things {
                let gpu_thing = GpuThing::new(&self.device, &self.uniform_bind_group_layout);
                let gpu_mesh = &self.gpu_meshes[thing.mesh_id];

                self.queue.write_buffer(
                    &gpu_thing.uniform_buffer,
                    0,
                    bytemuck::bytes_of(&TransformUniform::new(
                        thing.transform.model_matrix().to_cols_array_2d(),
                        projection_view.to_cols_array_2d(),
                    )),
                );

                render_pass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
                render_pass.set_index_buffer(gpu_mesh.index_buffer.slice(..), IndexFormat::Uint32);

                render_pass.set_bind_group(0, Some(&gpu_thing.uniform_bind_group), &[]);
                render_pass.draw_indexed(0..gpu_mesh.index_count, 0, 0..1);
            }
        }

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

        self.depth_texture = self.device.create_texture(&TextureDescriptor {
            label: Some("Depth Texture"),
            size: Extent3d {
                width: self.config.width.max(1),
                height: self.config.height.max(1),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Depth32Float,
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[TextureFormat::Depth32Float],
        });

        self.depth_view = self
            .depth_texture
            .create_view(&TextureViewDescriptor::default());

        self.surface.configure(&self.device, &self.config);
    }

    fn reconfigure_surface(&self) {
        self.surface.configure(&self.device, &self.config);
        self.window.request_redraw();
    }

    pub fn fill_gpu_meshes(&mut self, world: &World) {
        for thing in &world.things {
            if self.gpu_meshes.get(thing.mesh_id).is_some() {
                continue;
            }

            let gpu_mesh = GpuMesh::new(&world.mesh_storage[thing.mesh_id], &self.device);

            self.gpu_meshes.push(gpu_mesh);
        }
    }

    fn sync_meshes(&mut self, world: &World) {
        for mesh in world.mesh_storage.iter().skip(self.gpu_meshes.len()) {
            self.gpu_meshes.push(GpuMesh::new(mesh, &self.device));
        }
    }
}
