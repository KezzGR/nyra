use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
    BufferBindingType, Color, ColorTargetState, ColorWrites, CommandEncoderDescriptor,
    CompareFunction, CurrentSurfaceTexture, DepthBiasState, DepthStencilState, Device,
    DeviceDescriptor, ExperimentalFeatures, Extent3d, Features, FragmentState, IndexFormat,
    Instance, Limits, LoadOp, MemoryHints, MultisampleState, Operations,
    PipelineCompilationOptions, PipelineLayoutDescriptor, PowerPreference, PrimitiveState,
    PrimitiveTopology, Queue, RenderPassColorAttachment, RenderPassDepthStencilAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions,
    SamplerBindingType, ShaderModuleDescriptor, ShaderSource, ShaderStages, StencilState, StoreOp,
    Surface, SurfaceConfiguration, TextureDescriptor, TextureDimension, TextureFormat,
    TextureSampleType, TextureUsages, TextureView, TextureViewDescriptor, TextureViewDimension,
    Trace, VertexState,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::{
    Assets, GpuTexture, MaterialID, MaterialUniform, MeshID, Texture, TextureID, ThingID,
    render::{Camera, GpuMaterial, GpuMesh, GpuThing, TransformUniform, Vertex},
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
    depth_texture: wgpu::Texture,
    depth_view: TextureView,
    transform_bind_group_layout: BindGroupLayout,
    material_bind_group_layout: BindGroupLayout,
    texture_bind_group_layout: BindGroupLayout,
    pipeline: RenderPipeline,
    fallback_texture: GpuTexture,
    gpu_meshes: HashMap<MeshID, GpuMesh>,
    gpu_things: HashMap<ThingID, GpuThing>,
    gpu_materials: HashMap<MaterialID, GpuMaterial>,
    gpu_textures: HashMap<TextureID, GpuTexture>,
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

        let (depth_texture, depth_view) =
            Renderer::create_depth_resources(&device, size.width.max(1), size.height.max(1));

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

        let material_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Material Bind Group Layout"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Texture Bind Group Layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: true },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Mesh Shader"),
            source: ShaderSource::Wgsl(include_str!("mesh.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Mesh Pipeline Layout"),
            bind_group_layouts: &[
                Some(&transform_bind_group_layout),
                Some(&material_bind_group_layout),
                Some(&texture_bind_group_layout),
            ],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Mesh Pipeline"),
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

        let fallback_texture = GpuTexture::new(
            &Texture::default(),
            &device,
            &queue,
            &texture_bind_group_layout,
        );

        Self {
            window,
            surface,
            device,
            queue,
            config,
            depth_texture,
            depth_view,
            transform_bind_group_layout,
            material_bind_group_layout,
            texture_bind_group_layout,
            pipeline,
            fallback_texture,
            gpu_meshes: HashMap::default(),
            gpu_things: HashMap::default(),
            gpu_materials: HashMap::default(),
            gpu_textures: HashMap::default(),
        }
    }

    pub fn render(&mut self, camera: &Camera, world: &World, assets: &Assets) {
        self.sync_things(world);
        self.sync_meshes(assets);
        self.sync_materials(assets);
        self.sync_textures(assets);

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

        let surface_view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &surface_view,
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

            let view_matrix = camera.view_matrix();
            let projection_matrix =
                camera.projection_matrix(self.config.width as f32 / self.config.height as f32);

            let projection_view = projection_matrix * view_matrix;

            render_pass.set_pipeline(&self.pipeline);

            for thing in &world.things {
                if !thing.visible {
                    continue;
                }

                let material_id: &MaterialID;

                if let Some(material) = &thing.material {
                    material_id = material;

                    if let Some(texture) = &assets
                        .material_storage
                        .get(material)
                        .expect("Material missing")
                        .texture_id
                    {
                        let gpu_texture = self
                            .gpu_textures
                            .get(texture)
                            .expect("GpuTexture missing after synchronization");

                        render_pass.set_bind_group(2, Some(&gpu_texture.bind_group), &[]);
                    } else {
                        render_pass.set_bind_group(2, Some(&self.fallback_texture.bind_group), &[]);
                    }
                } else {
                    continue;
                };

                let gpu_mesh = self
                    .gpu_meshes
                    .get(&thing.mesh_id)
                    .expect("GpuMesh missing after synchronization");

                let gpu_thing = self
                    .gpu_things
                    .get(&thing.id)
                    .expect("GpuThing missing after synchronization");

                let gpu_material = self
                    .gpu_materials
                    .get(material_id)
                    .expect("GpuMaterial missing after synchronization");

                render_pass.set_bind_group(1, Some(&gpu_material.bind_group), &[]);

                self.queue.write_buffer(
                    &gpu_thing.uniform_buffer,
                    0,
                    bytemuck::bytes_of(&TransformUniform::new(
                        thing.transform.model_matrix().to_cols_array(),
                        projection_view.to_cols_array(),
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

        self.update_depth_resources();

        self.surface.configure(&self.device, &self.config);
    }

    fn reconfigure_surface(&self) {
        self.surface.configure(&self.device, &self.config);
        self.window.request_redraw();
    }

    fn create_depth_resources(
        device: &Device,
        width: u32,
        height: u32,
    ) -> (wgpu::Texture, TextureView) {
        let depth_texture = device.create_texture(&TextureDescriptor {
            label: Some("Depth Texture"),
            size: Extent3d {
                width,
                height,
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

        (depth_texture, depth_view)
    }

    fn update_depth_resources(&mut self) {
        let (depth_texture, depth_view) = Renderer::create_depth_resources(
            &self.device,
            self.config.width.max(1),
            self.config.height.max(1),
        );

        self.depth_texture = depth_texture;

        self.depth_view = depth_view;
    }

    fn sync_meshes(&mut self, assets: &Assets) {
        self.gpu_meshes
            .retain(|id, _| assets.mesh_storage.contains_key(id));

        for (id, mesh) in &assets.mesh_storage {
            self.gpu_meshes
                .entry(*id)
                .or_insert_with(|| GpuMesh::new(mesh, &self.device));
        }
    }

    fn sync_things(&mut self, world: &World) {
        let alive: HashSet<ThingID> = world.things.iter().map(|thing| thing.id).collect();

        self.gpu_things.retain(|id, _| alive.contains(id));

        for thing in &world.things {
            self.gpu_things
                .entry(thing.id)
                .or_insert_with(|| GpuThing::new(&self.device, &self.transform_bind_group_layout));
        }
    }

    fn sync_materials(&mut self, assets: &Assets) {
        self.gpu_materials
            .retain(|id, _| assets.material_storage.contains_key(id));

        for (id, material) in &assets.material_storage {
            let gpu_material = self.gpu_materials.entry(*id).or_insert_with(|| {
                GpuMaterial::new(&self.device, &self.material_bind_group_layout)
            });

            self.queue.write_buffer(
                &gpu_material.uniform_buffer,
                0,
                bytemuck::bytes_of(&MaterialUniform::new(
                    material.base_color,
                    material.uv_scale.to_array(),
                    material.uv_offset.to_array(),
                )),
            );
        }
    }

    fn sync_textures(&mut self, assets: &Assets) {
        self.gpu_textures
            .retain(|id, _| assets.texture_storage.contains_key(id));

        for (id, texture) in &assets.texture_storage {
            self.gpu_textures.entry(*id).or_insert_with(|| {
                GpuTexture::new(
                    texture,
                    &self.device,
                    &self.queue,
                    &self.texture_bind_group_layout,
                )
            });
        }
    }
}
