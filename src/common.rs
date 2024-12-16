use std:: {iter, mem, vec };
use cgmath::*;
//use futures::sink::Buffer;
use wgpu::{util::DeviceExt, BindGroup};
use winit::{
    event::*,
    window::Window,
    event_loop::{ControlFlow, EventLoop},
};
use bytemuck:: {Pod, Zeroable, cast_slice};
use std::collections::HashMap;
use image::GenericImageView;
use rust_embed::RustEmbed;
use serde::Deserialize;
use rand::rngs::ThreadRng;
use noise::Perlin;
use std::fs;
use std::io::Read;
use std::sync::{Arc, Mutex};
//use std::path::Path;

use crate::{chunk, world::{self, WorldData}};
use crate::interact;

#[path="../src/transforms.rs"]
mod transforms;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

const ANIMATION_SPEED:f32 = 1.0;
const IS_PERSPECTIVE:bool = true;

#[derive(Debug, Deserialize)]
struct ModelData {
    block_name: String,
    creator: String,
    textures: Textures,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Textures {
    Individual { top: i8, sides: i8, bottom: i8 },
    Uniform { all: i8 },
}
impl Textures {
    fn top(&self) -> i8 {
        match self {
            Textures::Individual { top, .. } => *top,
            Textures::Uniform { all } => *all,
        }
    }

    fn sides(&self) -> i8 {
        match self {
            Textures::Individual { sides, .. } => *sides,
            Textures::Uniform { all } => *all,
        }
    }

    fn bottom(&self) -> i8 {
        match self {
            Textures::Individual { bottom, .. } => *bottom,
            Textures::Uniform { all } => *all,
        }
    }
}

#[derive(Clone)]
pub struct RandomnessFunctions {
    pub rng: ThreadRng,
    pub noise: Perlin
}
impl RandomnessFunctions {
    pub fn new() -> Self {
        RandomnessFunctions {
            rng: rand::thread_rng(),
            noise: Perlin::new(78685746)
        }
    }
}

#[derive(Clone)]
pub struct GameData {
    pub objects: Vec<Vec<Vertex>>,
    pub gui_objects: Vec<Vec<Vertex>>,
    pub _text_objects: Vec<Vec<Vertex>>,
    pub positions: Vec<(f32, f32, f32)>,
    pub gui_positions: Vec<(f32, f32, f32)>,
    pub _text_positions: Vec<(f32, f32, f32)>,
    pub model_matrices: Vec<Matrix4<f32>>,
    pub normal_matrices: Vec<Matrix4<f32>>,
    pub gui_scale: Vec<(f32, f32, f32)>,
    pub _text_scale: Vec<(f32, f32, f32)>,
    pub active: Vec<bool>,
    pub gui_active: Vec<bool>,
    pub _text_active: Vec<bool>,
    pub camera_position: Point3<f32>,
    pub camera_rotation: Point3<f32>,
}
impl GameData {
    pub fn new() -> Self {
        GameData {
            objects: Vec::new(),
            gui_objects: Vec::new(),
            _text_objects: Vec::new(),
            positions: Vec::new(),
            gui_positions: Vec::new(),
            _text_positions: Vec::new(),
            model_matrices: Vec::new(),
            normal_matrices: Vec::new(),
            gui_scale: Vec::new(),
            _text_scale: Vec::new(),
            active: Vec::new(),
            gui_active: Vec::new(),
            _text_active: Vec::new(),
            camera_position: (-0.0, 64.0, 0.0).into(),
            camera_rotation: (0.0, 0.0, 0.0).into(),
        }
    }

    pub fn _add_text_object(&mut self, item: Vec<Vertex>, position: (f32, f32, f32), scale: (f32, f32, f32), active: bool) {
        self._text_objects.push(item);
        self._text_positions.push(position);
        self._text_scale.push(scale);
        self._text_active.push(active);
    }

    pub fn add_gui_object(&mut self, item: Vec<Vertex>, position: (f32, f32, f32), scale: (f32, f32, f32), active: bool) {
        self.gui_objects.push(item);
        self.gui_positions.push(position);
        self.gui_scale.push(scale);
        self.gui_active.push(active);
    }

    pub fn add_object(&mut self, item: Vec<Vertex>, position: (i64, i64, i64), active: bool) {
        let position_new = ((position.0 * 32) as f32, (position.1 * 32) as f32, (position.2 * 32) as f32);
        self.objects.push(item);
        self.positions.push(position_new);
        self.active.push(active);
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Light {
    color: [f32; 4],
    specular_color : [f32; 4],
    ambient_intensity: f32,
    diffuse_intensity :f32,
    specular_intensity: f32,
    specular_shininess: f32,
}

pub fn light(c:[f32; 3], sc:[f32;3], ai: f32, di: f32, si: f32, ss: f32) -> Light {
    Light {
        color:[c[0], c[1], c[2], 1.0],
        specular_color: [sc[0], sc[1], sc[2], 1.0],
        ambient_intensity: ai,
        diffuse_intensity: di,
        specular_intensity: si,
        specular_shininess: ss,
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 4],
    pub normal: [f32; 4],
    pub color: [f32; 4],
    pub uv: [f32; 4],
}

#[allow(dead_code)]
pub fn vertex(p:[i8;3], n:[i8; 3], c:[f32; 3], u:[f32; 2]) -> Vertex {
    return Vertex {
        position: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
        normal: [n[0] as f32, n[1] as f32, n[2] as f32, 1.0],
        color: [c[0], c[1], c[2], 1.0],
        uv: [u[0], u[1], 0.0, 0.0],
    }
}

pub fn create_vertices(vertices: Vec<[i8; 3]>, normals: Vec<[i8; 3]>, colors: Vec<[f32; 3]>, uvs: Vec<[f32; 2]>) -> Vec<Vertex> {
    let mut data:Vec<Vertex> = Vec::with_capacity(vertices.len());
    for i in 0..vertices.len() {
        data.push(vertex(vertices[i], normals[i], colors[i], uvs[i]));
    }
    return data.to_vec()
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4, 2=>Float32x4, 3=>Float32x4];
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

struct State {
    init: transforms::InitWgpu,
    pipeline: wgpu::RenderPipeline,
    gui_pipeline: wgpu::RenderPipeline,
    vertex_buffers: Vec<wgpu::Buffer>,
    gui_vertex_buffers: Vec<wgpu::Buffer>,
    uniform_bind_groups: Vec<wgpu::BindGroup>,
    gui_uniform_bind_groups: Vec<wgpu::BindGroup>,
    vertex_uniform_buffers: Vec<wgpu::Buffer>,
    gui_vertex_uniform_buffers: Vec<wgpu::Buffer>,
    project_mat: Matrix4<f32>,
    num_vertices: Vec<u32>,
    gui_num_vertices: Vec<u32>,
    game_data: GameData,
    previous_frame_time: std::time::Instant,
    uniform_bind_group_layout: wgpu::BindGroupLayout,
    light_data: Light,
    frame: usize,
    randomness_functions: RandomnessFunctions,
    world_data: Arc<Mutex<world::WorldData>>
}

impl State {
    fn create_object(
        game_data: &GameData, init: &transforms::InitWgpu, light_data: Light, 
        uniform_bind_group_layout: &wgpu::BindGroupLayout, i: usize) -> (BindGroup, wgpu::Buffer, wgpu::Buffer, u32) {
        // create vertex uniform buffer
        // model_mat and view_projection_mat will be stored in vertex_uniform_buffer inside the update function
        let vertex_uniform_buffer = init.device.create_buffer(&wgpu::BufferDescriptor{
            label: Some("Vertex Uniform Buffer"),
            size: 192,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
       
        // create fragment uniform buffer. here we set eye_position = camera_position and light_position = eye_position
        let fragment_uniform_buffer = init.device.create_buffer(&wgpu::BufferDescriptor{
            label: Some("Fragment Uniform Buffer"),
            size: 32,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // store light and eye positions
        let light_position:&[f32; 3] = &Point3::new(0.0, 128.0, 0.0).into();
        let eye_position:&[f32; 3] = &Point3::new(0.0, 128.0, 0.0).into();
        init.queue.write_buffer(&fragment_uniform_buffer, 0, bytemuck::cast_slice(light_position));
        init.queue.write_buffer(&fragment_uniform_buffer, 16, bytemuck::cast_slice(eye_position));

        // create light uniform buffer
        let light_uniform_buffer = init.device.create_buffer(&wgpu::BufferDescriptor{
            label: Some("Light Uniform Buffer"),
            size: 48,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // store light parameters
        init.queue.write_buffer(&light_uniform_buffer, 0, bytemuck::cast_slice(&[light_data]));

        let texture_data = Assets::get("textures/blocks/atlas.png").expect("Failed to load embedded texture");
        let img = image::load_from_memory(&texture_data.data).expect("Failed to load texture");
        let rgba = img.to_rgba8();
        let (width, height) = img.dimensions();
        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        
        let texture = init.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        
        init.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            texture_size,
        );
        
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = init.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let uniform_bind_group = init.device.create_bind_group(&wgpu::BindGroupDescriptor{
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: vertex_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: fragment_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: light_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("Uniform Bind Group"),
        });

        let vertex_buffer = init.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: cast_slice(&game_data.objects[i]),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let num_vertices = game_data.objects[i].len() as u32;

        return (uniform_bind_group, vertex_uniform_buffer, vertex_buffer, num_vertices)
    }
    fn create_object_gui(
        game_data: &GameData, init: &transforms::InitWgpu, light_data: Light, 
        uniform_bind_group_layout: &wgpu::BindGroupLayout, i: usize) -> (BindGroup, wgpu::Buffer, wgpu::Buffer, u32) {
        // create vertex uniform buffer
        // model_mat and view_projection_mat will be stored in vertex_uniform_buffer inside the update function
        let vertex_uniform_buffer = init.device.create_buffer(&wgpu::BufferDescriptor{
            label: Some("Vertex Uniform Buffer"),
            size: 192,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
       
        // create fragment uniform buffer. here we set eye_position = camera_position and light_position = eye_position
        let fragment_uniform_buffer = init.device.create_buffer(&wgpu::BufferDescriptor{
            label: Some("Fragment Uniform Buffer"),
            size: 32,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // store light and eye positions
        let light_position:&[f32; 3] = &Point3::new(0.0, 128.0, 0.0).into();
        let eye_position:&[f32; 3] = &Point3::new(0.0, 128.0, 0.0).into();
        init.queue.write_buffer(&fragment_uniform_buffer, 0, bytemuck::cast_slice(light_position));
        init.queue.write_buffer(&fragment_uniform_buffer, 16, bytemuck::cast_slice(eye_position));

        // create light uniform buffer
        let light_uniform_buffer = init.device.create_buffer(&wgpu::BufferDescriptor{
            label: Some("Light Uniform Buffer"),
            size: 48,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // store light parameters
        init.queue.write_buffer(&light_uniform_buffer, 0, bytemuck::cast_slice(&[light_data]));

        let texture_data = Assets::get("textures/gui/gui_atlas.png").expect("Failed to load embedded texture");
        let img = image::load_from_memory(&texture_data.data).expect("Failed to load texture");
        let rgba = img.to_rgba8();
        let (width, height) = img.dimensions();
        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        
        let texture = init.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        
        init.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            texture_size,
        );
        
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = init.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let uniform_bind_group = init.device.create_bind_group(&wgpu::BindGroupDescriptor{
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: vertex_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: fragment_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: light_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("Uniform Bind Group"),
        });

        let vertex_buffer = init.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: cast_slice(&game_data.gui_objects[i]),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let num_vertices = game_data.gui_objects[i].len() as u32;

        return (uniform_bind_group, vertex_uniform_buffer, vertex_buffer, num_vertices)
    }
    fn create_object_from_chunk(
        chunk: &Vec<Vertex>, init: &transforms::InitWgpu, light_data: Light, 
        uniform_bind_group_layout: &wgpu::BindGroupLayout) -> (BindGroup, wgpu::Buffer, wgpu::Buffer, u32) {
        // create vertex uniform buffer
        // model_mat and view_projection_mat will be stored in vertex_uniform_buffer inside the update function
        let vertex_uniform_buffer = init.device.create_buffer(&wgpu::BufferDescriptor{
            label: Some("Vertex Uniform Buffer"),
            size: 192,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
       
        // create fragment uniform buffer. here we set eye_position = camera_position and light_position = eye_position
        let fragment_uniform_buffer = init.device.create_buffer(&wgpu::BufferDescriptor{
            label: Some("Fragment Uniform Buffer"),
            size: 32,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // store light and eye positions
        let light_position:&[f32; 3] = &Point3::new(0.0, 128.0, 0.0).into();
        let eye_position:&[f32; 3] = &Point3::new(0.0, 128.0, 0.0).into();
        init.queue.write_buffer(&fragment_uniform_buffer, 0, bytemuck::cast_slice(light_position));
        init.queue.write_buffer(&fragment_uniform_buffer, 16, bytemuck::cast_slice(eye_position));

        // create light uniform buffer
        let light_uniform_buffer = init.device.create_buffer(&wgpu::BufferDescriptor{
            label: Some("Light Uniform Buffer"),
            size: 48,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // store light parameters
        init.queue.write_buffer(&light_uniform_buffer, 0, bytemuck::cast_slice(&[light_data]));

        let texture_data = Assets::get("textures/blocks/atlas.png").expect("Failed to load embedded texture");
        let img = image::load_from_memory(&texture_data.data).expect("Failed to load texture");
        let rgba = img.to_rgba8();
        let (width, height) = img.dimensions();

        let texture_size = wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };
        
        let texture = init.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        
        init.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            texture_size,
        );
        
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = init.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let uniform_bind_group = init.device.create_bind_group(&wgpu::BindGroupDescriptor{
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: vertex_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: fragment_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: light_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("Uniform Bind Group"),
        });

        let vertex_buffer = init.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: cast_slice(&chunk),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let num_vertices = chunk.len() as u32;

        return (uniform_bind_group, vertex_uniform_buffer, vertex_buffer, num_vertices)
    }

    async fn new(window: &Window, game_data: GameData, light_data: Light, randomness_functions: RandomnessFunctions, world_data: Arc<Mutex<WorldData>>) -> Self {        
        let init =  transforms::InitWgpu::init_wgpu(window).await;

        let shader = init.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        let gui_shader = init.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("gui_shader.wgsl").into()),
        });

        // uniform data
        let look_direction = (0.0, 0.0, 0.0).into();
        let up_direction = cgmath::Vector3::unit_y();
        
        let (_, project_mat, _) = transforms::create_view_projection(game_data.camera_position, look_direction, up_direction, 
            init.config.width as f32 / init.config.height as f32, IS_PERSPECTIVE);

        let uniform_bind_group_layout: wgpu::BindGroupLayout = init.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("Uniform Bind Group Layout"),
        });

        let pipeline_layout = init.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = init.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: init.config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState{
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            //depth_stencil: None,
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None
        });
        let gui_pipeline = init.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("GUI Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &gui_shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &gui_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: init.config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState{
                topology: wgpu::PrimitiveTopology::TriangleList,
                cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            //depth_stencil: None,
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: false,
                depth_compare: wgpu::CompareFunction::Always,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            multiview: None
        });

        let mut vertex_buffers: Vec<wgpu::Buffer> = Vec::new();
        let mut num_vertices: Vec<u32> = Vec::new();
        let mut uniform_bind_groups: Vec<wgpu::BindGroup> = Vec::new();
        let mut vertex_uniform_buffers: Vec<wgpu::Buffer> = Vec::new();

        for i in 0..game_data.objects.len() {
            let (uniform_bind_group, vertex_uniform_buffer, vertex_buffer, num_vertices_) = Self::create_object(&game_data, &init, light_data, &uniform_bind_group_layout, i);
            vertex_buffers.push(vertex_buffer);
            num_vertices.push(num_vertices_);
            uniform_bind_groups.push(uniform_bind_group);
            vertex_uniform_buffers.push(vertex_uniform_buffer);
        }

        let mut gui_vertex_buffers: Vec<wgpu::Buffer> = Vec::new();
        let mut gui_num_vertices: Vec<u32> = Vec::new();
        let mut gui_uniform_bind_groups: Vec<wgpu::BindGroup> = Vec::new();
        let mut gui_vertex_uniform_buffers: Vec<wgpu::Buffer> = Vec::new();

        for i in 0..game_data.gui_objects.len() {
            let (uniform_bind_group, vertex_uniform_buffer, vertex_buffer, num_vertices_) = Self::create_object_gui(&game_data, &init, light_data, &uniform_bind_group_layout, i);
            gui_vertex_buffers.push(vertex_buffer);
            gui_num_vertices.push(num_vertices_);
            gui_uniform_bind_groups.push(uniform_bind_group);
            gui_vertex_uniform_buffers.push(vertex_uniform_buffer);
        }

        let previous_frame_time = std::time::Instant::now();

        let frame = 0;

        Self {
            init,
            pipeline,
            gui_pipeline,
            vertex_buffers,
            gui_vertex_buffers,
            uniform_bind_groups,
            gui_uniform_bind_groups,
            vertex_uniform_buffers,
            gui_vertex_uniform_buffers,
            project_mat,
            num_vertices,
            gui_num_vertices,
            game_data,
            previous_frame_time,
            uniform_bind_group_layout,
            light_data,
            frame,
            randomness_functions,
            world_data
        }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.init.instance.poll_all(true);
            self.init.size = new_size;
            self.init.config.width = new_size.width;
            self.init.config.height = new_size.height;
            self.init.surface.configure(&self.init.device, &self.init.config);
            self.project_mat = transforms::create_projection(new_size.width as f32 / new_size.height as f32, IS_PERSPECTIVE);
        }
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn mouse_input(&mut self, button: i8) {
        if button == 0 {
            let mut world_data = self.world_data.lock().unwrap();
            let (vertex_data_chunk, buffer_index) = interact::break_block(&mut self.game_data, &mut world_data);
            if buffer_index != -1 {
                let (uniform_bind_group, vertex_uniform_buffer, vertex_buffer, num_vertices_) = Self::create_object_from_chunk(&vertex_data_chunk, &self.init, self.light_data, &self.uniform_bind_group_layout);
                self.vertex_buffers[buffer_index as usize] = vertex_buffer;
                self.num_vertices[buffer_index as usize] = num_vertices_;
                self.uniform_bind_groups[buffer_index as usize] = uniform_bind_group;
                self.vertex_uniform_buffers[buffer_index as usize] = vertex_uniform_buffer;
                world_data.updated_chunks.push(buffer_index as usize);
            }
        } else if  button == 1 {
            let mut world_data = self.world_data.lock().unwrap();
            let (vertex_data_chunk, buffer_index) = interact::place_block(&mut self.game_data, &mut world_data);
            if buffer_index != -1 {
                let (uniform_bind_group, vertex_uniform_buffer, vertex_buffer, num_vertices_) = Self::create_object_from_chunk(&vertex_data_chunk, &self.init, self.light_data, &self.uniform_bind_group_layout);
                self.vertex_buffers[buffer_index as usize] = vertex_buffer;
                self.num_vertices[buffer_index as usize] = num_vertices_;
                self.uniform_bind_groups[buffer_index as usize] = uniform_bind_group;
                self.vertex_uniform_buffers[buffer_index as usize] = vertex_uniform_buffer;
                world_data.updated_chunks.push(buffer_index as usize);
            }
        }
    }

    fn update(&mut self, dt: std::time::Duration, keys_down: &HashMap<&str, bool>, mouse_movement: &Vec<f64>, slot_selected: i8) {
        self.frame += 1;
        let current_time = std::time::Instant::now();
        let frame_time = current_time.duration_since(self.previous_frame_time).as_secs_f32() * 20.0;
        //let fps = 1.0 / current_time.duration_since(self.previous_frame_time).as_secs_f32();
        //println!("fps: {}", fps);
        self.previous_frame_time = current_time;

        if let Some(is_pressed) = keys_down.get("right") {
            if is_pressed == &true {
                self.game_data.camera_rotation[1] += frame_time / 5.0;
            }
        }
        if let Some(is_pressed) = keys_down.get("left") {
            if is_pressed == &true {
                self.game_data.camera_rotation[1] -= frame_time / 5.0;
            }
        }

        self.game_data.camera_rotation[1] -= mouse_movement[0] as f32 * (frame_time * 0.003);
        self.game_data.camera_rotation[0] += mouse_movement[1] as f32 * (frame_time * 0.003);
        self.game_data.camera_rotation[0] = self.game_data.camera_rotation[0].clamp(-std::f32::consts::FRAC_PI_2 / 1.1, std::f32::consts::FRAC_PI_2 / 1.1);

        let forward = Vector3::new(
            self.game_data.camera_rotation[1].cos() * self.game_data.camera_rotation[0].cos(),
            self.game_data.camera_rotation[0].sin(),
            self.game_data.camera_rotation[1].sin() * self.game_data.camera_rotation[0].cos(),
        ).normalize();
        let right = Vector3::new(
            self.game_data.camera_rotation[1].sin(),
            0.0,
            -self.game_data.camera_rotation[1].cos(),
        ).normalize();

        if let Some(is_pressed) = keys_down.get("w") {
            if is_pressed == &true {
                self.game_data.camera_position[0] += frame_time * forward[0];
                self.game_data.camera_position[1] += frame_time * forward[1];
                self.game_data.camera_position[2] += frame_time * forward[2];
            }
        }
        if let Some(is_pressed) = keys_down.get("s") {
            if is_pressed == &true {
                self.game_data.camera_position[0] -= frame_time * forward[0];
                self.game_data.camera_position[1] -= frame_time * forward[1];
                self.game_data.camera_position[2] -= frame_time * forward[2];
            }
        }
        if let Some(is_pressed) = keys_down.get("a") {
            if is_pressed == &true {
                self.game_data.camera_position[0] += frame_time * right[0];
                self.game_data.camera_position[1] += frame_time * right[1];
                self.game_data.camera_position[2] += frame_time * right[2];
            }
        }
        if let Some(is_pressed) = keys_down.get("d") {
            if is_pressed == &true {
                self.game_data.camera_position[0] -= frame_time * right[0];
                self.game_data.camera_position[1] -= frame_time * right[1];
                self.game_data.camera_position[2] -= frame_time * right[2];
            }
        }

        let chunk_position_x = (self.game_data.camera_position.x / 32.0).floor();
        let chunk_position_y = (self.game_data.camera_position.y / 32.0).floor();
        let chunk_position_z = (self.game_data.camera_position.z / 32.0).floor();

        let mut world_data = self.world_data.lock().unwrap();

        if self.frame % 30 == 0 {
            for active in world_data.active_chunks.clone() {
                self.game_data.active[active] = false;
            }
            world_data.active_chunks = Vec::new();
            for x in -4..4 {
                for y in -2..2 {
                    for z in -4..4 {
                        let chunk_position_x_with_offset = chunk_position_x as i64 + x;
                        let chunk_position_y_with_offset = chunk_position_y as i64 + y;
                        let chunk_position_z_with_offset = chunk_position_z as i64 + z;
                        if let Some(chunk_index) = world_data.chunk_buffer_index.get(&(chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset)) {
                            let chunk_index = *chunk_index as usize;
                            self.game_data.active[chunk_index] = true;
                            world_data.active_chunks.push(chunk_index);
                        } else {
                            if !world_data.chunk_queue.contains(&(chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset)) {
                                world_data.chunk_queue.insert((chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset));
                            }
                        }
                    }
                }
            }
        }
        /*if world_data.chunk_queue.len() == 0 && world_data.chunk_update_queue.len() > 0 {
            let chunk_position = world_data.chunk_buffer_coordinates[world_data.chunk_update_queue[0]];
            let chunk_data = world_data.chunks[&(chunk_position.0, chunk_position.1, chunk_position.2)].clone();
            let (chunk_vertices, chunk_normals, chunk_colors, chunk_uvs) = chunk::render_chunk(&chunk_data, &self.game_data, &mut world_data, 
                chunk_position.0, chunk_position.1, chunk_position.2
            );
            let vertex_data_chunk = create_vertices(chunk_vertices, chunk_normals, chunk_colors, chunk_uvs);
            let mut buffer_index: usize = 0;
            if let Some(chunk_index) = world_data.chunk_buffer_index.get(&(chunk_position.0, chunk_position.1, chunk_position.2)) {
                buffer_index = *chunk_index as usize;
            }
            let (uniform_bind_group, vertex_uniform_buffer, vertex_buffer, num_vertices_) = Self::create_object_from_chunk(&vertex_data_chunk, &self.init, self.light_data, &self.uniform_bind_group_layout);
            self.vertex_buffers[buffer_index as usize] = vertex_buffer;
            self.num_vertices[buffer_index as usize] = num_vertices_;
            self.uniform_bind_groups[buffer_index as usize] = uniform_bind_group;
            self.vertex_uniform_buffers[buffer_index as usize] = vertex_uniform_buffer;
            world_data.updated_chunks.push(buffer_index as usize);
            world_data.chunk_update_queue.remove(0);
        }*/
        if let Some(chunk_coordinates) = world_data.chunk_queue.iter().next() {
            let chunk_position_x_with_offset = chunk_coordinates.0;
            let chunk_position_y_with_offset = chunk_coordinates.1;
            let chunk_position_z_with_offset = chunk_coordinates.2;
            world_data.chunk_queue.remove(&(chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset));
            let chunk_data = chunk::generate_chunk(
                chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset, &mut self.game_data, &mut self.randomness_functions, &mut world_data
            );
            let (chunk_vertices, chunk_normals, chunk_colors, chunk_uvs) = chunk::render_chunk(&chunk_data, &self.game_data, &mut world_data, 
                chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset
            );
            let vertex_data_chunk = create_vertices(chunk_vertices, chunk_normals, chunk_colors, chunk_uvs);
            world_data.set_chunk(chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset, chunk_data);
            self.game_data.add_object(vertex_data_chunk.clone(), (chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset), true);
            world_data.add_object((chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset));
            let (uniform_bind_group, vertex_uniform_buffer, vertex_buffer, num_vertices_) = Self::create_object_from_chunk(&vertex_data_chunk, &self.init, self.light_data, &self.uniform_bind_group_layout);
            self.vertex_buffers.push(vertex_buffer);
            self.num_vertices.push(num_vertices_);
            self.uniform_bind_groups.push(uniform_bind_group);
            self.vertex_uniform_buffers.push(vertex_uniform_buffer);
            world_data.active_chunks.push(self.vertex_buffers.len() - 1);
            world_data.updated_chunks.push(self.vertex_buffers.len() - 1);
            world_data.chunk_update_queue.push(self.vertex_buffers.len() - 1);
            let model_mat = transforms::create_transforms([
                chunk_position_x_with_offset as f32 * 32.0, 
                chunk_position_y_with_offset as f32 * 32.0, 
                chunk_position_z_with_offset as f32 * 32.0], 
                [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]
            );
            let normal_mat = (model_mat.invert().unwrap()).transpose();
            self.game_data.model_matrices.push(model_mat);
            self.game_data.normal_matrices.push(normal_mat);
        }

        let up_direction = cgmath::Vector3::unit_y();
        let (view_mat, project_mat, _) = transforms::create_view_rotation(
            self.game_data.camera_position, self.game_data.camera_rotation[1], self.game_data.camera_rotation[0], 
            up_direction, self.init.config.width as f32 / self.init.config.height as f32, IS_PERSPECTIVE);

        // update uniform buffer
        let _dt = ANIMATION_SPEED * dt.as_secs_f32(); 
        let view_project_mat = project_mat * view_mat;
        let view_projection_ref:&[f32; 16] = view_project_mat.as_ref();

        for i in &world_data.updated_chunks {
            let model_mat = self.game_data.model_matrices[*i];
            let normal_mat = self.game_data.normal_matrices[*i];
            let model_ref:&[f32; 16] = model_mat.as_ref();
            let normal_ref:&[f32; 16] = normal_mat.as_ref();
            self.init.queue.write_buffer(&self.vertex_uniform_buffers[*i], 0, bytemuck::cast_slice(model_ref));
            self.init.queue.write_buffer(&self.vertex_uniform_buffers[*i], 128, bytemuck::cast_slice(normal_ref));
        }
        world_data.updated_chunks = Vec::new();
        for i in &world_data.active_chunks {
            if self.num_vertices[*i] == 0 { continue; }
            self.init.queue.write_buffer(&self.vertex_uniform_buffers[*i], 64, bytemuck::cast_slice(view_projection_ref));
        }

        self.game_data.gui_positions[2] = (0.11 * (slot_selected as f32 - 4.0), -0.6, 0.0);

        let rotation_x = -self.game_data.camera_rotation.x;
        let rotation_y = -self.game_data.camera_rotation.y + std::f32::consts::FRAC_PI_2;
        let rotation_z = -self.game_data.camera_rotation.z;
        let gui_offset_normal: Matrix4<f32> = transforms::create_transforms(
            [self.game_data.camera_position.x, self.game_data.camera_position.y, self.game_data.camera_position.z], 
            [rotation_x, rotation_y, rotation_z], 
            [1.0, 1.0, 1.0]);

        for i in 0..self.game_data.gui_objects.len() {
            let position_x = gui_offset_normal.x.x * -self.game_data.gui_positions[i].0 + gui_offset_normal.y.x * self.game_data.gui_positions[i].1 + forward.x + self.game_data.camera_position.x;
            let position_y = gui_offset_normal.x.y * -self.game_data.gui_positions[i].0 + gui_offset_normal.y.y * self.game_data.gui_positions[i].1 + forward.y + self.game_data.camera_position.y;
            let position_z = gui_offset_normal.x.z * -self.game_data.gui_positions[i].0 + gui_offset_normal.y.z * self.game_data.gui_positions[i].1 + forward.z + self.game_data.camera_position.z;
            let model_mat = transforms::create_transforms(
                [position_x, position_y, position_z], 
                [rotation_x, rotation_y, rotation_z], 
                [self.game_data.gui_scale[i].0, self.game_data.gui_scale[i].1, self.game_data.gui_scale[i].2]);
            let normal_mat = (model_mat.invert().unwrap()).transpose();
            let model_ref:&[f32; 16] = model_mat.as_ref();
            let normal_ref:&[f32; 16] = normal_mat.as_ref();

            self.init.queue.write_buffer(&self.gui_vertex_uniform_buffers[i], 0, bytemuck::cast_slice(model_ref));
            self.init.queue.write_buffer(&self.gui_vertex_uniform_buffers[i], 64, bytemuck::cast_slice(view_projection_ref));
            self.init.queue.write_buffer(&self.gui_vertex_uniform_buffers[i], 128, bytemuck::cast_slice(normal_ref));
        }

        //let current_time_updated = std::time::Instant::now();
        //let update_time = current_time_updated.duration_since(current_time).as_millis();
        //println!("update time: {}ms", update_time);
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        //let output = self.init.surface.get_current_frame()?.output;
        let output = self.init.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        
        let depth_texture = self.init.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: self.init.config.width,
                height: self.init.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format:wgpu::TextureFormat::Depth24Plus,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .init.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.2,
                            g: 0.247,
                            b: 0.314,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                //depth_stencil_attachment: None,
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: false,
                    }),
                    stencil_ops: None,
                }),
            });

            let world_data = self.world_data.lock().unwrap();

            render_pass.set_pipeline(&self.pipeline);
            for i in &world_data.active_chunks {
                render_pass.set_vertex_buffer(0, self.vertex_buffers[*i].slice(..));           
                render_pass.set_bind_group(0, &self.uniform_bind_groups[*i], &[]);
                render_pass.draw(0..self.num_vertices[*i], 0..1);
            }
            render_pass.set_pipeline(&self.gui_pipeline);
            for i in 0..self.game_data.gui_objects.len() {
                if !self.game_data.gui_active[i] { continue; }
                render_pass.set_vertex_buffer(0, self.gui_vertex_buffers[i].slice(..));           
                render_pass.set_bind_group(0, &self.gui_uniform_bind_groups[i], &[]);
                render_pass.draw(0..self.gui_num_vertices[i], 0..1);
            }
        }

        self.init.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

fn handle_model_data(world_data: &mut world::WorldData, json_content: &str) {
    let model_data: ModelData = serde_json::from_str(json_content).expect("Failed to parse JSON");
    world_data.add_block(
        model_data.block_name,
        vec![
            model_data.textures.sides(),
            model_data.textures.sides(),
            model_data.textures.top(),
            model_data.textures.bottom(),
            model_data.textures.sides(),
            model_data.textures.sides(),
        ],
        model_data.creator
    );
}
pub fn load_block_model_files(world_data_thread: Arc<Mutex<world::WorldData>>) {
    let mut world_data = world_data_thread.lock().unwrap();
    let exe_path = std::env::current_exe().expect("Failed to get current executable path");
    let exe_dir = exe_path.parent().expect("Failed to get executable directory");
    let models_dir = exe_dir.join("assets/models/blocks");
    let mut json_files = Vec::new();
    if models_dir.exists() && models_dir.is_dir() {
        println!("Found the modded directory for models");
        for entry in fs::read_dir(&models_dir).expect("Failed to read models directory") {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "json") {
                    if let Some(file_name) = path.strip_prefix(&exe_dir).ok().and_then(|p| p.to_str()) {
                        println!("Found the modded model file: {}", file_name);
                        json_files.push(file_name.to_string());
                    }
                }
            }
        }
    }
    json_files.extend(
        Assets::iter()
            .filter(|file| file.starts_with("models/") && file.ends_with(".json"))
            .map(|file| file.to_string())
    );
    json_files.sort();
    json_files.dedup();
    world_data.block_index.insert("air".to_string(), 0);
    for file in json_files {
        println!("Found JSON file: {}", file);
        let file_path = exe_dir.join(&file);
        if file_path.exists() {
            let mut file_content = String::new();
            let mut file = fs::File::open(&file_path).expect("Failed to open file");
            file.read_to_string(&mut file_content).expect("Failed to read file");
            handle_model_data(&mut world_data, &file_content);
        } else if let Some(asset) = Assets::get(&file) {
            let json_content = std::str::from_utf8(asset.data.as_ref()).expect("Invalid UTF-8");
            handle_model_data(&mut world_data, json_content);
        }
    }
}

pub fn run(game_data: GameData, randomness_functions: RandomnessFunctions, world_data: Arc<Mutex<world::WorldData>>, light_data: Light, title: &str) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title(title);
    
    if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::Confined) {
        eprintln!("Failed to lock the cursor: {:?}", err);
    }
    window.set_cursor_visible(false);

    let mut game_state = pollster::block_on(State::new(&window, game_data, light_data, randomness_functions, world_data));    
    let render_start_time = std::time::Instant::now();

    let mut keys_down: HashMap<&str, bool> = HashMap::new();
    keys_down.insert("w", false);
    keys_down.insert("a", false);
    keys_down.insert("s", false);
    keys_down.insert("d", false);
    keys_down.insert("space", false);
    let mut slot_selected: i8 = 0;
    let mut mouse_movement: Vec<f64> = vec![0.0, 0.0];
    let mut mouse_locked = true;

    event_loop.run(move |event, _, control_flow| {
        mouse_movement[0] -= mouse_movement[0] * 0.1;
        mouse_movement[1] -= mouse_movement[1] * 0.1;
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !game_state.input(event) {
                    match event {
                        WindowEvent::KeyboardInput {
                            input: KeyboardInput {
                                    state: key_state,
                                    virtual_keycode: Some(keycode),
                                    ..
                                },
                            ..
                        } => {
                            match key_state {
                                ElementState::Pressed => {
                                    match &keycode {
                                        &VirtualKeyCode::W => { keys_down.insert("w", true); }
                                        &VirtualKeyCode::A => { keys_down.insert("a", true); }
                                        &VirtualKeyCode::S => { keys_down.insert("s", true); }
                                        &VirtualKeyCode::D => { keys_down.insert("d", true); }
                                        &VirtualKeyCode::Space => { keys_down.insert("space", true); }
                                        &VirtualKeyCode::Right => { keys_down.insert("right", true); }
                                        &VirtualKeyCode::Left => { keys_down.insert("left", true); }
                                        &VirtualKeyCode::Key1 => { slot_selected = 0; }
                                        &VirtualKeyCode::Key2 => { slot_selected = 1; }
                                        &VirtualKeyCode::Key3 => { slot_selected = 2; }
                                        &VirtualKeyCode::Key4 => { slot_selected = 3; }
                                        &VirtualKeyCode::Key5 => { slot_selected = 4; }
                                        &VirtualKeyCode::Key6 => { slot_selected = 5; }
                                        &VirtualKeyCode::Key7 => { slot_selected = 6; }
                                        &VirtualKeyCode::Key8 => { slot_selected = 7; }
                                        &VirtualKeyCode::Key9 => { slot_selected = 8; }
                                        //&VirtualKeyCode::Key0 => { slot_selected = 9; }
                                        &VirtualKeyCode::Escape => {
                                            mouse_locked = false;
                                            if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::None) {
                                                eprintln!("Failed to unlock the cursor: {:?}", err);
                                            }
                                            window.set_cursor_visible(true);
                                        }
                                        _ => {}
                                    }
                                }
                                ElementState::Released => {
                                    match &keycode {
                                        &VirtualKeyCode::W => { keys_down.insert("w", false); }
                                        &VirtualKeyCode::A => { keys_down.insert("a", false); }
                                        &VirtualKeyCode::S => { keys_down.insert("s", false); }
                                        &VirtualKeyCode::D => { keys_down.insert("d", false); }
                                        &VirtualKeyCode::Space => { keys_down.insert("space", false); }
                                        &VirtualKeyCode::Right => { keys_down.insert("right", false); }
                                        &VirtualKeyCode::Left => { keys_down.insert("left", false); }
                                        _ => {}
                                    }
                                }
                            }
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            if mouse_locked {
                                let window_size = window.inner_size();
                                let center_x = window_size.width as f64 / 2.0;
                                let center_y = window_size.height as f64 / 2.0;
                                mouse_movement[0] = center_x - position.x;
                                mouse_movement[1] = center_y - position.y;
                                window.set_cursor_position(winit::dpi::PhysicalPosition::new(center_x, center_y)).expect("Failed to set cursor position");
                            } else {
                                mouse_movement[0] = 0.0;
                                mouse_movement[1] = 0.0;
                            }
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            match state {
                                ElementState::Pressed => {
                                    match button {
                                        MouseButton::Left => {
                                            if mouse_locked == false {
                                                if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::Confined) {
                                                    eprintln!("Failed to lock the cursor: {:?}", err);
                                                }
                                                window.set_cursor_visible(false);
                                                mouse_locked = true;
                                            }
                                            game_state.mouse_input(0);
                                        }
                                        MouseButton::Right => {
                                            game_state.mouse_input(1);
                                        }
                                        MouseButton::Middle => {
                                            game_state.mouse_input(2);
                                        }
                                        _ => {}
                                    }
                                }
                                ElementState::Released => {
                                    return
                                }
                            }
                        }
                        WindowEvent::CloseRequested {} => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            game_state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            game_state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(_) => {
                let now = std::time::Instant::now();
                let dt = now - render_start_time;
                game_state.update(dt, &keys_down, &mouse_movement, slot_selected);

                match game_state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => game_state.resize(game_state.init.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}