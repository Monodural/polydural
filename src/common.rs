use std:: {iter, mem, vec };
use cgmath::*;
use rand::Rng;
//use futures::sink::Buffer;
use wgpu::BindGroup;
use winit::{
    event::*,
    window::{Icon, Window},
    event_loop::{ControlFlow, EventLoop},
};
use bytemuck:: {Pod, Zeroable};
use std::collections::HashMap;
use image::GenericImageView;
use rust_embed::RustEmbed;
use serde::Deserialize;
use noise::Perlin;
use std::fs;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::io::Cursor;
use std::path::{Path, PathBuf};
use hound;
use lewton::inside_ogg::OggStreamReader;
extern crate lewton;

use crate::{containers::Inventory, world::{self, WorldData}};
use crate::interact;
use crate::gui::update_frame;

#[path="../src/transforms.rs"]
mod transforms;

#[path="../src/physics.rs"]
mod physics;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

const ANIMATION_SPEED:f32 = 1.0;
const IS_PERSPECTIVE:bool = true;

#[derive(Deserialize, Debug, Clone)]
pub struct Element {
    pub from: [f64; 3],
    pub to: [f64; 3],
}

#[derive(Deserialize, Debug, Clone)]
pub struct ShapeData {
    shape_name: String,
    elements: Vec<Element>,
}

#[derive(Debug, Deserialize)]
struct ModelData {
    block_name: String,
    creator: String,
    textures: Textures,
    shape: String,
    sides: bool,
    transparent: bool,
    collide: bool
}

#[derive(Debug, Deserialize, Clone)]
pub struct Block {
    pub position: [i32; 3],
    pub block: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StructureData {
    pub structure_name: String,
    pub blocks: Vec<Block>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BiomeData {
    pub biome_name: String,
    pub temperature: i8,
    pub moisture: i8,
    pub height: i8,
    pub block_levels: Vec<(Vec<String>, i64)>,
    pub sea_level: i64,
    pub trees: Vec<(String, f32)>,
    pub folliage: Vec<(String, f32)>,
    pub buildings: Vec<(String, f32)>
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Textures {
    Single { top: i8, left: i8, right: i8, front: i8, back: i8, bottom: i8 },
    Individual { top: i8, sides: i8, bottom: i8 },
    Uniform { all: i8 },
}
impl Textures {
    fn top(&self) -> i8 {
        match self {
            Textures::Single { top, .. } => *top,
            Textures::Individual { top, .. } => *top,
            Textures::Uniform { all } => *all,
        }
    }

    fn left(&self) -> i8 {
        match self {
            Textures::Single { left, .. } => *left,
            Textures::Individual { sides, .. } => *sides,
            Textures::Uniform { all } => *all,
        }
    }
    fn right(&self) -> i8 {
        match self {
            Textures::Single { right, .. } => *right,
            Textures::Individual { sides, .. } => *sides,
            Textures::Uniform { all } => *all,
        }
    }
    fn front(&self) -> i8 {
        match self {
            Textures::Single { front, .. } => *front,
            Textures::Individual { sides, .. } => *sides,
            Textures::Uniform { all } => *all,
        }
    }
    fn back(&self) -> i8 {
        match self {
            Textures::Single { back, .. } => *back,
            Textures::Individual { sides, .. } => *sides,
            Textures::Uniform { all } => *all,
        }
    }

    fn bottom(&self) -> i8 {
        match self {
            Textures::Single { bottom, .. } => *bottom,
            Textures::Individual { bottom, .. } => *bottom,
            Textures::Uniform { all } => *all,
        }
    }
}

#[derive(Clone)]
pub struct RandomnessFunctions {
    //pub rng: ThreadRng,
    pub noise: Perlin
}
impl RandomnessFunctions {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let seed: u32 = rng.gen_range(0..1000000);
        println!("seed: {}", seed);

        RandomnessFunctions {
            //rng: rand::thread_rng(),
            noise: Perlin::new(seed)
        }
    }
}

#[derive(Clone)]
pub struct GameData {
    pub objects: Vec<Vec<Vertex>>,
    pub objects_transparent: Vec<Vec<Vertex>>,
    pub gui_objects: Vec<Vec<Vertex>>,
    pub gui_item_block_objects: Vec<Vec<Vertex>>,
    pub positions: Vec<(f32, f32, f32)>,
    pub gui_positions: Vec<(f32, f32, f32)>,
    pub gui_item_block_positions: Vec<(f32, f32, f32)>,
    pub gui_item_block_rotations: Vec<(f32, f32, f32)>,
    pub text_positions: Vec<(f32, f32, f32)>,
    pub gui_scale: Vec<(f32, f32, f32)>,
    pub gui_item_block_scale: Vec<(f32, f32, f32)>,
    pub text_scale: Vec<(f32, f32, f32)>,
    pub active: Vec<bool>,
    pub gui_active: Vec<bool>,
    pub gui_item_block_active: Vec<bool>,
    pub text_active: Vec<bool>,
    pub text: Vec<String>,
    pub camera_position: Point3<f32>,
    pub camera_rotation: Point3<f32>,
    pub camera_acceleration: Point3<f32>,
    pub camera_acceleration_walking: Point3<f32>,
    pub grounded: bool,
    pub jumping: bool
}
impl GameData {
    pub fn new() -> Self {
        GameData {
            objects: Vec::new(),
            objects_transparent: Vec::new(),
            gui_objects: Vec::new(),
            gui_item_block_objects: Vec::new(),
            positions: Vec::new(),
            gui_positions: Vec::new(),
            gui_item_block_positions: Vec::new(),
            gui_item_block_rotations: Vec::new(),
            text_positions: Vec::new(),
            gui_scale: Vec::new(),
            gui_item_block_scale: Vec::new(),
            text_scale: Vec::new(),
            active: Vec::new(),
            gui_active: Vec::new(),
            gui_item_block_active: Vec::new(),
            text_active: Vec::new(),
            text: Vec::new(),
            camera_position: (-0.0, 64.0, 0.0).into(),
            camera_rotation: (0.0, 0.0, 0.0).into(),
            camera_acceleration: (0.0, 0.0, 0.0).into(),
            camera_acceleration_walking: (0.0, 0.0, 0.0).into(),
            grounded: false,
            jumping: false
        }
    }

    pub fn add_text_object(&mut self, position: (f32, f32, f32), scale: (f32, f32, f32), active: bool, text: String) {
        self.text_positions.push(position);
        self.text_scale.push(scale);
        self.text_active.push(active);
        self.text.push(text);
    }

    pub fn add_gui_object(&mut self, item: Vec<Vertex>, position: (f32, f32, f32), scale: (f32, f32, f32), active: bool) {
        self.gui_objects.push(item);
        self.gui_positions.push(position);
        self.gui_scale.push(scale);
        self.gui_active.push(active);
    }

    pub fn add_gui_item_block(&mut self, item: Vec<Vertex>, position: (f32, f32, f32), scale: (f32, f32, f32), rotation: (f32, f32, f32), active: bool) {
        self.gui_item_block_objects.push(item);
        self.gui_item_block_positions.push(position);
        self.gui_item_block_rotations.push(rotation);
        self.gui_item_block_scale.push(scale);
        self.gui_item_block_active.push(active);
    }

    pub fn add_object(&mut self, item: Vec<Vertex>, item_transparent: Vec<Vertex>, position: (i64, i64, i64), active: bool) {
        let position_new = ((position.0 * 32) as f32, (position.1 * 32) as f32, (position.2 * 32) as f32);
        self.objects.push(item);
        self.objects_transparent.push(item_transparent);
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
pub fn vertex(p:[f64;3], n:[i8; 3], c:[f32; 3], u:[f32; 2]) -> Vertex {
    return Vertex {
        position: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
        normal: [n[0] as f32, n[1] as f32, n[2] as f32, 1.0],
        color: [c[0], c[1], c[2], 1.0],
        uv: [u[0], u[1], 0.0, 0.0],
    }
}

pub fn create_vertices(vertices: Vec<[f64; 3]>, normals: Vec<[i8; 3]>, colors: Vec<[f32; 3]>, uvs: Vec<[f32; 2]>) -> Vec<Vertex> {
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
    pipeline_transparent: wgpu::RenderPipeline,
    gui_pipeline: wgpu::RenderPipeline,
    text_pipeline: wgpu::RenderPipeline,
    gui_item_block_pipeline: wgpu::RenderPipeline,
    world_vertex_buffer: wgpu::Buffer,
    world_vertex_buffer_transparent: wgpu::Buffer,
    world_fragment_buffer: wgpu::Buffer,
    world_fragment_buffer_transparent: wgpu::Buffer,
    gui_vertex_buffer: wgpu::Buffer,
    text_vertex_buffer: wgpu::Buffer,
    gui_item_block_vertex_buffer: wgpu::Buffer,
    world_uniform_bind_group: wgpu::BindGroup,
    world_uniform_bind_group_transparent: wgpu::BindGroup,
    gui_uniform_bind_group: wgpu::BindGroup,
    text_uniform_bind_group: wgpu::BindGroup,
    gui_item_block_uniform_bind_group: wgpu::BindGroup,
    vertex_uniform_buffer: wgpu::Buffer,
    gui_vertex_uniform_buffer: wgpu::Buffer,
    text_vertex_uniform_buffer: wgpu::Buffer,
    gui_item_block_vertex_uniform_buffer: wgpu::Buffer,
    project_mat: Matrix4<f32>,
    world_num_vertices: u32,
    world_num_vertices_transparent: u32,
    gui_num_vertices_: u32,
    text_num_vertices_: u32,
    gui_item_block_num_vertices_: u32,
    game_data: GameData,
    previous_frame_time: std::time::Instant,
    frame: usize,
    vertex_data: Vec<Vec<Vertex>>,
    vertex_offset: Vec<u64>,
    vertex_data_transparent: Vec<Vec<Vertex>>,
    vertex_offset_transparent: Vec<u64>,
    model_matrices: Vec<Matrix4<f32>>,
    model_matrices_transparent: Vec<Matrix4<f32>>,
    normal_matrices: Vec<Matrix4<f32>>,
    normal_matrices_transparent: Vec<Matrix4<f32>>,
    world_data: Arc<Mutex<world::WorldData>>,
    chunk_data_terrain: Arc<Mutex<HashMap<(i64, i64, i64), Vec<i8>>>>,
    chunk_data_lighting: Arc<Mutex<HashMap<(i64, i64, i64), Vec<i8>>>>,
    chunks: HashMap<(i64, i64, i64), Vec<i8>>,
    blocks: Vec<(String, Vec<i8>, String, String, bool, bool, bool)>,
    render_ui: bool,
    //rng: rand::prelude::ThreadRng,
    chunk_world_vertices: Vec<Vertex>,
    chunk_transparent_world_vertices: Vec<Vertex>
}

impl State {
    fn create_object_gui(
        init: &transforms::InitWgpu, light_data: Light, 
        uniform_bind_group_layout: &wgpu::BindGroupLayout) -> (BindGroup, wgpu::Buffer, wgpu::Buffer) {
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

        let max_buffer_size = 1024 * 1024 * 16; // 16MB buffer
        let vertex_buffer = init.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: max_buffer_size as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        return (uniform_bind_group, vertex_uniform_buffer, vertex_buffer)
    }
    fn create_object_text(
        _game_data: &GameData, init: &transforms::InitWgpu, light_data: Light, 
        uniform_bind_group_layout: &wgpu::BindGroupLayout) -> (BindGroup, wgpu::Buffer, wgpu::Buffer) {
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

        let max_buffer_size = 1024 * 1024 * 16; // 16MB buffer
        let vertex_buffer = init.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: max_buffer_size as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        return (uniform_bind_group, vertex_uniform_buffer, vertex_buffer)
    }
    fn create_object_gui_item_block(
        init: &transforms::InitWgpu, light_data: Light, 
        uniform_bind_group_layout: &wgpu::BindGroupLayout) -> (BindGroup, wgpu::Buffer, wgpu::Buffer) {
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

        let max_buffer_size = 1024 * 1024 * 16; // 16MB buffer
        let vertex_buffer = init.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: max_buffer_size as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        return (uniform_bind_group, vertex_uniform_buffer, vertex_buffer)
    }
    fn create_world_buffer(
        init: &transforms::InitWgpu, light_data: Light, 
        uniform_bind_group_layout: &wgpu::BindGroupLayout, world_data_thread: &Arc<Mutex<world::WorldData>>, vertex_uniform_buffer: &wgpu::Buffer) -> (BindGroup, wgpu::Buffer, u32, wgpu::Buffer) {
       
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
        
        let texture_size;
        let rgba: Vec<u8>;
        let width: u32;
        let height: u32;

        {
            let world_data = world_data_thread.lock().unwrap();
            texture_size = world_data.textures[0].1;
            rgba = world_data.textures[0].0.to_vec();
            width = world_data.textures[0].2;
            height = world_data.textures[0].3;
        }

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

        let max_buffer_size = 1024 * 1024 * 256; // 256MB buffer
        let vertex_buffer = init.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: max_buffer_size as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let num_vertices = 0;

        return (uniform_bind_group, vertex_buffer, num_vertices, fragment_uniform_buffer)
    }
    
    async fn new(window: &Window, game_data: GameData, light_data: Light, world_data: Arc<Mutex<WorldData>>, chunk_data_terrain: Arc<Mutex<HashMap<(i64, i64, i64), Vec<i8>>>>, chunk_data_lighting: Arc<Mutex<HashMap<(i64, i64, i64), Vec<i8>>>>) -> Self {        
        let init =  transforms::InitWgpu::init_wgpu(window).await;

        let shader = init.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });
        let shader_transparent = init.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader_transparent.wgsl").into()),
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
        let pipeline_transparent = init.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_transparent,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_transparent,
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
        let text_pipeline = init.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
        let gui_item_block_pipeline = init.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("GUI Item Render Pipeline"),
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

        let vertex_uniform_buffer: wgpu::Buffer = init.device.create_buffer(&wgpu::BufferDescriptor{
            label: Some("Vertex Uniform Buffer"),
            size: 192,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let (world_uniform_bind_group, world_vertex_buffer, world_num_vertices, world_fragment_buffer) = 
            Self::create_world_buffer(&init, light_data, &uniform_bind_group_layout, &world_data, &vertex_uniform_buffer);
        let (world_uniform_bind_group_transparent, world_vertex_buffer_transparent, world_num_vertices_transparent, world_fragment_buffer_transparent) = 
            Self::create_world_buffer(&init, light_data, &uniform_bind_group_layout, &world_data, &vertex_uniform_buffer);

        let (
            gui_uniform_bind_group, 
            gui_vertex_uniform_buffer, 
            gui_vertex_buffer
        ) = Self::create_object_gui(&init, light_data, &uniform_bind_group_layout);
        let mut gui_element: Vec<Vertex> = Vec::new();
        for i in 0..game_data.gui_objects.len() {
            gui_element.extend(&game_data.gui_objects[i]);
        }
        init.queue.write_buffer(&gui_vertex_buffer, 0, bytemuck::cast_slice(&gui_element));
        let gui_num_vertices_ = gui_element.len() as u32;

        let (
            text_uniform_bind_group, 
            text_vertex_uniform_buffer, 
            text_vertex_buffer
        ) = Self::create_object_text(&game_data, &init, light_data, &uniform_bind_group_layout);
        let mut text_characters: Vec<Vertex> = Vec::new();
        for i in 0..game_data.text.len() {
            let mut character_offset = 1.0;
            for character in game_data.text[i].chars() {
                let mut uvs = vec![[0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0], [0.0, 0.0]];
                match character {
                    '1' => { uvs = vec![[0.039 + 0.000, 0.242], [0.015 + 0.000, 0.242], [0.015 + 0.000, 0.296], [0.039 + 0.000, 0.242], [0.015 + 0.000, 0.296], [0.039 + 0.000, 0.296]]; }
                    '2' => { uvs = vec![[0.046 + 0.031, 0.242], [0.015 + 0.031, 0.242], [0.015 + 0.031, 0.296], [0.046 + 0.031, 0.242], [0.015 + 0.031, 0.296], [0.046 + 0.031, 0.296]]; }
                    '3' => { uvs = vec![[0.046 + 0.070, 0.242], [0.015 + 0.070, 0.242], [0.015 + 0.070, 0.296], [0.046 + 0.070, 0.242], [0.015 + 0.070, 0.296], [0.046 + 0.070, 0.296]]; }
                    '4' => { uvs = vec![[0.046 + 0.109, 0.242], [0.015 + 0.109, 0.242], [0.015 + 0.109, 0.296], [0.046 + 0.109, 0.242], [0.015 + 0.109, 0.296], [0.046 + 0.109, 0.296]]; }
                    '5' => { uvs = vec![[0.046 + 0.148, 0.242], [0.015 + 0.148, 0.242], [0.015 + 0.148, 0.296], [0.046 + 0.148, 0.242], [0.015 + 0.148, 0.296], [0.046 + 0.148, 0.296]]; }
                    '6' => { uvs = vec![[0.046 + 0.188, 0.242], [0.015 + 0.188, 0.242], [0.015 + 0.188, 0.296], [0.046 + 0.188, 0.242], [0.015 + 0.188, 0.296], [0.046 + 0.188, 0.296]]; }
                    '7' => { uvs = vec![[0.046 + 0.227, 0.242], [0.015 + 0.227, 0.242], [0.015 + 0.227, 0.296], [0.046 + 0.227, 0.242], [0.015 + 0.227, 0.296], [0.046 + 0.227, 0.296]]; }
                    '8' => { uvs = vec![[0.046 + 0.266, 0.242], [0.015 + 0.266, 0.242], [0.015 + 0.266, 0.296], [0.046 + 0.266, 0.242], [0.015 + 0.266, 0.296], [0.046 + 0.266, 0.296]]; }
                    '9' => { uvs = vec![[0.046 + 0.305, 0.242], [0.015 + 0.305, 0.242], [0.015 + 0.305, 0.296], [0.046 + 0.305, 0.242], [0.015 + 0.305, 0.296], [0.046 + 0.305, 0.296]]; }
                    '0' => { uvs = vec![[0.046 + 0.344, 0.242], [0.015 + 0.344, 0.242], [0.015 + 0.344, 0.296], [0.046 + 0.344, 0.242], [0.015 + 0.344, 0.296], [0.046 + 0.344, 0.296]]; }
                    'a' => { uvs = vec![[0.039 + 0.000, 0.242 + 0.063], [0.008 + 0.000, 0.242 + 0.063], [0.008 + 0.000, 0.296 + 0.063], [0.039 + 0.000, 0.242 + 0.063], [0.008 + 0.000, 0.296 + 0.063], [0.039 + 0.000, 0.296 + 0.063]]; }
                    'b' => { uvs = vec![[0.039 + 0.039, 0.242 + 0.063], [0.008 + 0.039, 0.242 + 0.063], [0.008 + 0.039, 0.296 + 0.063], [0.039 + 0.039, 0.242 + 0.063], [0.008 + 0.039, 0.296 + 0.063], [0.039 + 0.039, 0.296 + 0.063]]; }
                    'c' => { uvs = vec![[0.039 + 0.078, 0.242 + 0.063], [0.008 + 0.078, 0.242 + 0.063], [0.008 + 0.078, 0.296 + 0.063], [0.039 + 0.078, 0.242 + 0.063], [0.008 + 0.078, 0.296 + 0.063], [0.039 + 0.078, 0.296 + 0.063]]; }
                    'd' => { uvs = vec![[0.039 + 0.117, 0.242 + 0.063], [0.008 + 0.117, 0.242 + 0.063], [0.008 + 0.117, 0.296 + 0.063], [0.039 + 0.117, 0.242 + 0.063], [0.008 + 0.117, 0.296 + 0.063], [0.039 + 0.117, 0.296 + 0.063]]; }
                    'e' => { uvs = vec![[0.039 + 0.156, 0.242 + 0.063], [0.008 + 0.156, 0.242 + 0.063], [0.008 + 0.156, 0.296 + 0.063], [0.039 + 0.156, 0.242 + 0.063], [0.008 + 0.156, 0.296 + 0.063], [0.039 + 0.156, 0.296 + 0.063]]; }
                    'f' => { uvs = vec![[0.039 + 0.195, 0.242 + 0.063], [0.008 + 0.195, 0.242 + 0.063], [0.008 + 0.195, 0.296 + 0.063], [0.039 + 0.195, 0.242 + 0.063], [0.008 + 0.195, 0.296 + 0.063], [0.039 + 0.195, 0.296 + 0.063]]; }
                    'g' => { uvs = vec![[0.039 + 0.234, 0.242 + 0.063], [0.008 + 0.234, 0.242 + 0.063], [0.008 + 0.234, 0.296 + 0.063], [0.039 + 0.234, 0.242 + 0.063], [0.008 + 0.234, 0.296 + 0.063], [0.039 + 0.234, 0.296 + 0.063]]; }
                    'h' => { uvs = vec![[0.039 + 0.273, 0.242 + 0.063], [0.008 + 0.273, 0.242 + 0.063], [0.008 + 0.273, 0.296 + 0.063], [0.039 + 0.273, 0.242 + 0.063], [0.008 + 0.273, 0.296 + 0.063], [0.039 + 0.273, 0.296 + 0.063]]; }
                    'i' => { uvs = vec![[0.039 + 0.312, 0.242 + 0.063], [0.015 + 0.312, 0.242 + 0.063], [0.015 + 0.312, 0.296 + 0.063], [0.039 + 0.312, 0.242 + 0.063], [0.015 + 0.312, 0.296 + 0.063], [0.039 + 0.312, 0.296 + 0.063]]; }
                    'j' => { uvs = vec![[0.039 + 0.351, 0.242 + 0.063], [0.008 + 0.351, 0.242 + 0.063], [0.008 + 0.351, 0.296 + 0.063], [0.039 + 0.351, 0.242 + 0.063], [0.008 + 0.351, 0.296 + 0.063], [0.039 + 0.351, 0.296 + 0.063]]; }
                    'k' => { uvs = vec![[0.039 + 0.000, 0.242 + 0.126], [0.008 + 0.000, 0.242 + 0.126], [0.008 + 0.000, 0.296 + 0.126], [0.039 + 0.000, 0.242 + 0.126], [0.008 + 0.000, 0.296 + 0.126], [0.039 + 0.000, 0.296 + 0.126]]; }
                    'l' => { uvs = vec![[0.039 + 0.039, 0.242 + 0.126], [0.008 + 0.039, 0.242 + 0.126], [0.008 + 0.039, 0.296 + 0.126], [0.039 + 0.039, 0.242 + 0.126], [0.008 + 0.039, 0.296 + 0.126], [0.039 + 0.039, 0.296 + 0.126]]; }
                    'm' => { uvs = vec![[0.046 + 0.078, 0.242 + 0.126], [0.008 + 0.078, 0.242 + 0.126], [0.008 + 0.078, 0.296 + 0.126], [0.046 + 0.078, 0.242 + 0.126], [0.008 + 0.078, 0.296 + 0.126], [0.046 + 0.078, 0.296 + 0.126]]; }
                    'n' => { uvs = vec![[0.039 + 0.156, 0.242 + 0.126], [0.008 + 0.156, 0.242 + 0.126], [0.008 + 0.156, 0.296 + 0.126], [0.039 + 0.156, 0.242 + 0.126], [0.008 + 0.156, 0.296 + 0.126], [0.039 + 0.156, 0.296 + 0.126]]; }
                    'o' => { uvs = vec![[0.039 + 0.195, 0.242 + 0.126], [0.008 + 0.195, 0.242 + 0.126], [0.008 + 0.195, 0.296 + 0.126], [0.039 + 0.195, 0.242 + 0.126], [0.008 + 0.195, 0.296 + 0.126], [0.039 + 0.195, 0.296 + 0.126]]; }
                    'p' => { uvs = vec![[0.039 + 0.234, 0.242 + 0.126], [0.008 + 0.234, 0.242 + 0.126], [0.008 + 0.234, 0.296 + 0.126], [0.039 + 0.234, 0.242 + 0.126], [0.008 + 0.234, 0.296 + 0.126], [0.039 + 0.234, 0.296 + 0.126]]; }
                    'q' => { uvs = vec![[0.039 + 0.273, 0.242 + 0.126], [0.008 + 0.273, 0.242 + 0.126], [0.008 + 0.273, 0.296 + 0.126], [0.039 + 0.273, 0.242 + 0.126], [0.008 + 0.273, 0.296 + 0.126], [0.039 + 0.273, 0.296 + 0.126]]; }
                    'r' => { uvs = vec![[0.039 + 0.312, 0.242 + 0.126], [0.008 + 0.312, 0.242 + 0.126], [0.008 + 0.312, 0.296 + 0.126], [0.039 + 0.312, 0.242 + 0.126], [0.008 + 0.312, 0.296 + 0.126], [0.039 + 0.312, 0.296 + 0.126]]; }
                    's' => { uvs = vec![[0.039 + 0.351, 0.242 + 0.126], [0.008 + 0.351, 0.242 + 0.126], [0.008 + 0.351, 0.296 + 0.126], [0.039 + 0.351, 0.242 + 0.126], [0.008 + 0.351, 0.296 + 0.126], [0.039 + 0.351, 0.296 + 0.126]]; }
                    't' => { uvs = vec![[0.039 + 0.000, 0.242 + 0.189], [0.015 + 0.000, 0.242 + 0.189], [0.015 + 0.000, 0.296 + 0.189], [0.039 + 0.000, 0.242 + 0.189], [0.015 + 0.000, 0.296 + 0.189], [0.039 + 0.000, 0.296 + 0.189]]; }
                    'u' => { uvs = vec![[0.039 + 0.078, 0.242 + 0.189], [0.008 + 0.078, 0.242 + 0.189], [0.008 + 0.078, 0.296 + 0.189], [0.039 + 0.078, 0.242 + 0.189], [0.008 + 0.078, 0.296 + 0.189], [0.039 + 0.078, 0.296 + 0.189]]; }
                    'v' => { uvs = vec![[0.046 + 0.156, 0.242 + 0.189], [0.008 + 0.156, 0.242 + 0.189], [0.008 + 0.156, 0.296 + 0.189], [0.046 + 0.156, 0.242 + 0.189], [0.008 + 0.156, 0.296 + 0.189], [0.046 + 0.156, 0.296 + 0.189]]; }
                    'w' => { uvs = vec![[0.046 + 0.234, 0.242 + 0.189], [0.008 + 0.234, 0.242 + 0.189], [0.008 + 0.234, 0.296 + 0.189], [0.046 + 0.234, 0.242 + 0.189], [0.008 + 0.234, 0.296 + 0.189], [0.046 + 0.234, 0.296 + 0.189]]; }
                    'x' => { uvs = vec![[0.046 + 0.312, 0.242 + 0.189], [0.008 + 0.312, 0.242 + 0.189], [0.008 + 0.312, 0.296 + 0.189], [0.046 + 0.312, 0.242 + 0.189], [0.008 + 0.312, 0.296 + 0.189], [0.046 + 0.312, 0.296 + 0.189]]; }
                    'y' => { uvs = vec![[0.046 + 0.000, 0.242 + 0.252], [0.008 + 0.000, 0.242 + 0.252], [0.008 + 0.000, 0.296 + 0.252], [0.046 + 0.000, 0.242 + 0.252], [0.008 + 0.000, 0.296 + 0.252], [0.046 + 0.000, 0.296 + 0.252]]; }
                    'z' => { uvs = vec![[0.046 + 0.078, 0.242 + 0.252], [0.008 + 0.078, 0.242 + 0.252], [0.008 + 0.078, 0.296 + 0.252], [0.046 + 0.078, 0.242 + 0.252], [0.008 + 0.078, 0.296 + 0.252], [0.046 + 0.078, 0.296 + 0.252]]; }
                    _ => {}
                }
                let vertex_data = create_vertices(
                    vec![
                                    [(-1.0 - character_offset) * game_data.text_scale[i].0 as f64 - game_data.text_positions[i].0 as f64, 1.0 * game_data.text_scale[i].1 as f64 + game_data.text_positions[i].1 as f64, 1.0], 
                                    [(1.0 - character_offset) * game_data.text_scale[i].0 as f64 - game_data.text_positions[i].0 as f64, 1.0 * game_data.text_scale[i].1 as f64 + game_data.text_positions[i].1 as f64, 1.0], 
                                    [(1.0 - character_offset) * game_data.text_scale[i].0 as f64 - game_data.text_positions[i].0 as f64, -1.0 * game_data.text_scale[i].1 as f64 + game_data.text_positions[i].1 as f64, 1.0], 
                                    [(-1.0 - character_offset) * game_data.text_scale[i].0 as f64 - game_data.text_positions[i].0 as f64, 1.0 * game_data.text_scale[i].1 as f64 + game_data.text_positions[i].1 as f64, 1.0], 
                                    [(1.0 - character_offset) * game_data.text_scale[i].0 as f64 - game_data.text_positions[i].0 as f64, -1.0 * game_data.text_scale[i].1 as f64 + game_data.text_positions[i].1 as f64, 1.0], 
                                    [(-1.0 - character_offset) * game_data.text_scale[i].0 as f64 - game_data.text_positions[i].0 as f64, -1.0 * game_data.text_scale[i].1 as f64 + game_data.text_positions[i].1 as f64, 1.0]
                                ], 
                    vec![[0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1]], 
                    vec![[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]], 
                    uvs
                );
                text_characters.extend(vertex_data);
                character_offset += 2.5;
            }
        }
        init.queue.write_buffer(&text_vertex_buffer, 0, bytemuck::cast_slice(&text_characters));
        let text_num_vertices_ = text_characters.len() as u32;

        let (
            gui_item_block_uniform_bind_group, 
            gui_item_block_vertex_uniform_buffer, 
            gui_item_block_vertex_buffer
        ) = Self::create_object_gui_item_block(&init, light_data, &uniform_bind_group_layout);
        let mut gui_item_blocks: Vec<Vertex> = Vec::new();
        for i in 0..game_data.gui_item_block_objects.len() {
            gui_item_blocks.extend(&game_data.gui_item_block_objects[i]);
        }
        init.queue.write_buffer(&gui_item_block_vertex_buffer, 0, bytemuck::cast_slice(&gui_item_blocks));
        let gui_item_block_num_vertices_ = gui_item_blocks.len() as u32;

        let previous_frame_time = std::time::Instant::now();

        let frame = 0;

        let vertex_data = Vec::new();
        let vertex_offset = Vec::new();
        let model_matrices = Vec::new();
        let normal_matrices = Vec::new();

        let vertex_data_transparent = Vec::new();
        let vertex_offset_transparent = Vec::new();
        let model_matrices_transparent = Vec::new();
        let normal_matrices_transparent = Vec::new();

        let model_mat = transforms::create_transforms([
            0 as f32, 0 as f32, 0 as f32], 
            [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]
        );
        let normal_mat = (model_mat.invert().unwrap()).transpose();

        let model_ref:&[f32; 16] = model_mat.as_ref();
        let normal_ref:&[f32; 16] = normal_mat.as_ref();
        init.queue.write_buffer(&vertex_uniform_buffer, 0, bytemuck::cast_slice(model_ref));
        init.queue.write_buffer(&vertex_uniform_buffer, 128, bytemuck::cast_slice(normal_ref));
        
        let blocks;
        {
            let world_data_read = world_data.lock().unwrap();
            blocks = world_data_read.blocks.clone();
        }

        let chunks = HashMap::new();
        //let rng: rand::prelude::ThreadRng = rand::thread_rng();

        let render_ui = true;

        let chunk_world_vertices: Vec<Vertex> = Vec::new();
        let chunk_transparent_world_vertices: Vec<Vertex> = Vec::new();

        Self {
            init,
            pipeline,
            pipeline_transparent,
            gui_pipeline,
            text_pipeline,
            gui_item_block_pipeline,
            world_vertex_buffer,
            world_vertex_buffer_transparent,
            world_fragment_buffer,
            world_fragment_buffer_transparent,
            gui_vertex_buffer,
            text_vertex_buffer,
            gui_item_block_vertex_buffer,
            world_uniform_bind_group,
            world_uniform_bind_group_transparent,
            gui_uniform_bind_group,
            text_uniform_bind_group,
            gui_item_block_uniform_bind_group,
            vertex_uniform_buffer,
            gui_vertex_uniform_buffer,
            text_vertex_uniform_buffer,
            gui_item_block_vertex_uniform_buffer,
            project_mat,
            world_num_vertices,
            world_num_vertices_transparent,
            gui_num_vertices_,
            text_num_vertices_,
            gui_item_block_num_vertices_,
            game_data,
            previous_frame_time,
            frame,
            vertex_data,
            vertex_offset,
            vertex_data_transparent,
            vertex_offset_transparent,
            model_matrices,
            model_matrices_transparent,
            normal_matrices,
            normal_matrices_transparent,
            world_data,
            chunk_data_terrain,
            chunk_data_lighting,
            chunks,
            blocks,
            render_ui,
            //rng,
            chunk_world_vertices,
            chunk_transparent_world_vertices
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

    fn mouse_input(&mut self, button: i8, slot_selected: i8, inventory: Inventory) {
        if button == 0 {
            let vertex_data_chunk: Vec<Vertex>;
            let vertex_data_chunk_transparent: Vec<Vertex>;
            let buffer_index: i32;
            {
                (vertex_data_chunk, buffer_index, vertex_data_chunk_transparent) = interact::break_block(&mut self.game_data, &self.chunk_data_terrain, &self.chunk_data_lighting, &self.world_data);
            }
            if buffer_index != -1 {
                self.vertex_data[buffer_index as usize] = vertex_data_chunk;
                self.vertex_data_transparent[buffer_index as usize] = vertex_data_chunk_transparent;

                self.init.queue.write_buffer(&self.world_vertex_buffer, self.vertex_offset[buffer_index as usize] * std::mem::size_of::<Vertex>() as u64, bytemuck::cast_slice(&self.vertex_data[buffer_index as usize]));
                self.init.queue.write_buffer(&self.world_vertex_buffer_transparent, self.vertex_offset_transparent[buffer_index as usize] * std::mem::size_of::<Vertex>() as u64, bytemuck::cast_slice(&self.vertex_data_transparent[buffer_index as usize]));

                {   
                    let mut world_data = self.world_data.lock().unwrap();
                    world_data.sound_queue.push((1, 0.1));
                }
            }
        } else if button == 1 {
            let vertex_data_chunk: Vec<Vertex>;
            let vertex_data_chunk_transparent: Vec<Vertex>;
            let buffer_index: i32;
            {
                (vertex_data_chunk, buffer_index, vertex_data_chunk_transparent) = interact::place_block(&mut self.game_data, &self.chunk_data_terrain, &self.chunk_data_lighting, &self.world_data, slot_selected, inventory);
            }
            if buffer_index != -1 {
                self.vertex_data[buffer_index as usize] = vertex_data_chunk;
                self.vertex_data_transparent[buffer_index as usize] = vertex_data_chunk_transparent;

                self.init.queue.write_buffer(&self.world_vertex_buffer, self.vertex_offset[buffer_index as usize] * std::mem::size_of::<Vertex>() as u64, bytemuck::cast_slice(&self.vertex_data[buffer_index as usize]));
                self.init.queue.write_buffer(&self.world_vertex_buffer_transparent, self.vertex_offset_transparent[buffer_index as usize] * std::mem::size_of::<Vertex>() as u64, bytemuck::cast_slice(&self.vertex_data_transparent[buffer_index as usize]));

                {
                    let mut world_data = self.world_data.lock().unwrap();
                    world_data.sound_queue.push((2, 0.1));
                }
            }
        }
    }

    fn update(&mut self, dt: std::time::Duration, keys_down: &HashMap<&str, bool>, mouse_movement: &Vec<f64>, slot_selected: i8, render_ui: bool) {
        self.frame += 1;
        let current_time = std::time::Instant::now();
        let frame_time = current_time.duration_since(self.previous_frame_time).as_secs_f32() * 20.0;
        self.previous_frame_time = current_time;
        self.render_ui = render_ui;

        if let Some(is_pressed) = keys_down.get("right") {
            if is_pressed == &true {
                self.game_data.camera_rotation[1] += frame_time * 0.1;
            }
        }
        if let Some(is_pressed) = keys_down.get("left") {
            if is_pressed == &true {
                self.game_data.camera_rotation[1] -= frame_time * 0.1;
            }
        }

        //println!("{}", frame_time);
        self.game_data.camera_rotation[1] -= mouse_movement[0] as f32 * (frame_time * 0.1);
        self.game_data.camera_rotation[0] += mouse_movement[1] as f32 * (frame_time * 0.1);
        self.game_data.camera_rotation[0] = self.game_data.camera_rotation[0].clamp(-std::f32::consts::FRAC_PI_2 / 1.01, std::f32::consts::FRAC_PI_2 / 1.01);

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

        self.game_data.camera_acceleration_walking = (0.0, self.game_data.camera_acceleration_walking[1], 0.0).into();
        if let Some(is_pressed) = keys_down.get("w") {
            if is_pressed == &true {
                self.game_data.camera_acceleration_walking[0] += frame_time * forward[0];
                //self.game_data.camera_acceleration_walking[1] += frame_time * forward[1];
                self.game_data.camera_acceleration_walking[2] += frame_time * forward[2];
            }
        }
        if let Some(is_pressed) = keys_down.get("s") {
            if is_pressed == &true {
                self.game_data.camera_acceleration_walking[0] -= frame_time * forward[0];
                //self.game_data.camera_acceleration_walking[1] -= frame_time * forward[1];
                self.game_data.camera_acceleration_walking[2] -= frame_time * forward[2];
            }
        }
        if let Some(is_pressed) = keys_down.get("a") {
            if is_pressed == &true {
                self.game_data.camera_acceleration_walking[0] += frame_time * right[0];
                //self.game_data.camera_acceleration_walking[1] += frame_time * right[1];
                self.game_data.camera_acceleration_walking[2] += frame_time * right[2];
            }
        }
        if let Some(is_pressed) = keys_down.get("d") {
            if is_pressed == &true {
                self.game_data.camera_acceleration_walking[0] -= frame_time * right[0];
                //self.game_data.camera_acceleration_walking[1] -= frame_time * right[1];
                self.game_data.camera_acceleration_walking[2] -= frame_time * right[2];
            }
        }
        if let Some(is_pressed) = keys_down.get("space") {
            if is_pressed == &true && !self.game_data.jumping && self.game_data.grounded {
                self.game_data.camera_acceleration_walking[1] = 0.5;
                self.game_data.jumping = true;
            }
        }

        physics::update(&mut self.game_data, &self.chunks, &self.blocks, frame_time);

        let chunk_position_x = (self.game_data.camera_position.x / 64.0).floor();
        let chunk_position_y = (self.game_data.camera_position.y / 64.0).floor();
        let chunk_position_z = (self.game_data.camera_position.z / 64.0).floor();
        
        if self.frame % 30 == 0 {
            {
                let mut world_data = self.world_data.lock().unwrap();
                for active in world_data.active_chunks.clone() {
                    self.game_data.active[active] = false;
                }
                world_data.active_chunks.clear();
                self.chunks.clear();

                for radius_reversed in 0..4 {
                    let radius = 3 - radius_reversed;
                    for x in -radius..radius+1 {
                        for y in -2..2 {
                            for z in -radius..radius+1 {
                                if z != radius && z != -radius && x != radius && x != -radius {
                                    continue;
                                }
                                if ((x as f32).powi(2) + (y as f32).powi(2) + (z as f32).powi(2)).sqrt() > 4.0 {
                                    continue;
                                }

                                let chunk_position_x_with_offset = chunk_position_x as i64 + x;
                                let chunk_position_y_with_offset = chunk_position_y as i64 + y;
                                let chunk_position_z_with_offset = chunk_position_z as i64 + z;
                                if let Some(chunk_index) = world_data.chunk_buffer_index.get(&(chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset)) {
                                    let chunk_index = *chunk_index as usize;
                                    self.game_data.active[chunk_index] = true;
                                    world_data.active_chunks.push(chunk_index);

                                    self.chunks.insert(
                                        (chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset), 
                                        self.chunk_data_terrain.lock().unwrap()[&(chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset)].clone()
                                    );
                                } else {
                                    if !world_data.chunk_queue.contains(&(chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset)) && 
                                        !world_data.created_chunk_queue.contains(&(chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset)) {
                                        //println!("chunk queue new: {}", world_data.chunk_queue.len());
                                        world_data.chunk_queue.insert((chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset));
                                    }
                                }
                            }
                        }
                    }
                }

                /*let chance = self.rng.gen_range(0..10);
                if chance == 0 {
                    world_data.sound_queue.push((4, 0.1));
                }*/
            }

            let eye_position:&[f32; 3] = &Point3::new(self.game_data.camera_position.x, self.game_data.camera_position.y, self.game_data.camera_position.z).into();
            self.init.queue.write_buffer(&self.world_fragment_buffer, 16, bytemuck::cast_slice(eye_position));
            self.init.queue.write_buffer(&self.world_fragment_buffer_transparent, 16, bytemuck::cast_slice(eye_position));
        }
        if self.frame % 30 == 10 || self.frame % 30 == 15 {
            let updated_chunk;
            let updated_chunk_transparent;
            let updated_chunk_data_len;
            {
                let world_data_check = self.world_data.lock().unwrap();
                updated_chunk = world_data_check.updated_chunk_data.clone();
                updated_chunk_transparent = world_data_check.updated_chunk_data_transparent.clone();
                updated_chunk_data_len = updated_chunk.len();
            }
            for i in 0..10 {
                if updated_chunk_data_len > i {
                    let chunk_data = &updated_chunk[i];
                    let chunk_data_transparent = &updated_chunk_transparent[i];
                    self.vertex_data[chunk_data.0] = chunk_data.1.clone();
                    self.vertex_data_transparent[chunk_data_transparent.0] = chunk_data_transparent.1.clone();
    
                    self.init.queue.write_buffer(&self.world_vertex_buffer, self.world_num_vertices as u64 * std::mem::size_of::<Vertex>() as u64, bytemuck::cast_slice(&self.vertex_data[chunk_data.0]));
                    self.init.queue.write_buffer(&self.world_vertex_buffer_transparent, self.world_num_vertices_transparent as u64 * std::mem::size_of::<Vertex>() as u64, bytemuck::cast_slice(&self.vertex_data_transparent[chunk_data_transparent.0]));
    
                    self.world_num_vertices += self.vertex_data[chunk_data.0].len() as u32;
                    self.world_num_vertices_transparent += self.vertex_data_transparent[chunk_data_transparent.0].len() as u32;
    
                    {
                        let mut world_data_setting = self.world_data.lock().unwrap();
                        world_data_setting.updated_chunks.push(chunk_data.0);
                        world_data_setting.updated_chunks_transparent.push(chunk_data_transparent.0);
                        world_data_setting.updated_chunk_data.remove(0);
                        world_data_setting.updated_chunk_data_transparent.remove(0);
                    }
                }
            }
        }
        if self.frame % 30 == 20 || self.frame % 30 == 25 {
            let mut chunks_added = 0;
            let mut created_chunk_data_len;
            {
                let mut world_data_check = self.world_data.lock().unwrap();
                created_chunk_data_len = world_data_check.created_chunk_data.len();
                for i in 0..3 {
                    if created_chunk_data_len > i+1 {
                        chunks_added += 1;
                        let updated_chunk = &world_data_check.created_chunk_data;
                        let updated_chunk_transparent = &world_data_check.created_chunk_data_transparent;
                        let chunk_data = &updated_chunk[i].clone();
                        let chunk_data_transparent = &updated_chunk_transparent[i];
                        self.vertex_data.push(chunk_data.0.clone());
                        self.vertex_data_transparent.push(chunk_data_transparent.0.clone());

                        self.model_matrices.push(chunk_data.4);
                        self.model_matrices_transparent.push(chunk_data_transparent.4);
                        self.normal_matrices.push(chunk_data.5);
                        self.normal_matrices_transparent.push(chunk_data_transparent.5);
                        self.game_data.add_object(chunk_data.0.clone(), chunk_data_transparent.0.clone(), (chunk_data.1, chunk_data.2, chunk_data.3), true);
                        self.vertex_offset.push(0);
                        self.vertex_offset_transparent.push(0);
                        
                        world_data_check.active_chunks.push(self.vertex_data.len() - 1);
                        world_data_check.updated_chunks.push(self.vertex_data.len() - 1);
                        world_data_check.chunk_update_queue.push(self.vertex_data.len() - 1);
                        world_data_check.created_chunk_data.remove(i);
                        world_data_check.created_chunk_data_transparent.remove(i);
                        world_data_check.add_object((chunk_data.1, chunk_data.2, chunk_data.3));

                        created_chunk_data_len = world_data_check.created_chunk_data.len();
                    }
                }
            }
            for i in 1..chunks_added+1 {
                self.init.queue.write_buffer(&self.world_vertex_buffer, self.world_num_vertices as u64 * std::mem::size_of::<Vertex>() as u64, bytemuck::cast_slice(&self.vertex_data[self.vertex_data.len() - i]));
                self.init.queue.write_buffer(&self.world_vertex_buffer_transparent, self.world_num_vertices_transparent as u64 * std::mem::size_of::<Vertex>() as u64, bytemuck::cast_slice(&self.vertex_data_transparent[self.vertex_data_transparent.len() - i]));
                self.world_num_vertices += self.vertex_data[self.vertex_data.len() - i].len() as u32;
                self.world_num_vertices_transparent += self.vertex_data_transparent[self.vertex_data_transparent.len() - i].len() as u32;
            }
        }

        // this will update the distances for solid chunks in a few different frames
        if self.frame % 300 == 0 {
            self.chunk_world_vertices.clear();

            let active_chunk_data;
            {
                active_chunk_data = self.world_data.lock().unwrap();
            }

            let total_chunks = active_chunk_data.active_chunks.len();
            let start = 0;
            let end = total_chunks / 5;
            for i in &active_chunk_data.active_chunks[start..end] {
                self.vertex_offset[*i] = self.chunk_world_vertices.len() as u64;
                self.chunk_world_vertices.extend(&self.vertex_data[*i]);
            }
        }
        if self.frame % 300 == 2 {
            let active_chunk_data;
            {
                active_chunk_data = self.world_data.lock().unwrap();
            }

            let total_chunks = active_chunk_data.active_chunks.len();
            let start = total_chunks / 5;
            let end = 2 * total_chunks / 5;
            for i in &active_chunk_data.active_chunks[start..end] {
                self.vertex_offset[*i] = self.chunk_world_vertices.len() as u64;
                self.chunk_world_vertices.extend(&self.vertex_data[*i]);
            }
        }
        if self.frame % 300 == 4 {
            let active_chunk_data;
            {
                active_chunk_data = self.world_data.lock().unwrap();
            }

            let total_chunks = active_chunk_data.active_chunks.len();
            let start = 2 * total_chunks / 5;
            let end = 3 * total_chunks / 5;
            for i in &active_chunk_data.active_chunks[start..end] {
                self.vertex_offset[*i] = self.chunk_world_vertices.len() as u64;
                self.chunk_world_vertices.extend(&self.vertex_data[*i]);
            }
        }
        if self.frame % 300 == 6 {
            let active_chunk_data;
            {
                active_chunk_data = self.world_data.lock().unwrap();
            }

            let total_chunks = active_chunk_data.active_chunks.len();
            let start = 3 * total_chunks / 5;
            let end = 4 * total_chunks / 5;
            for i in &active_chunk_data.active_chunks[start..end] {
                self.vertex_offset[*i] = self.chunk_world_vertices.len() as u64;
                self.chunk_world_vertices.extend(&self.vertex_data[*i]);
            }
        }
        if self.frame % 300 == 8 {
            let active_chunk_data;
            {
                active_chunk_data = self.world_data.lock().unwrap();
            }

            let total_chunks = active_chunk_data.active_chunks.len();
            let start = 4 * total_chunks / 5;
            let end = total_chunks;
            for i in &active_chunk_data.active_chunks[start..end] {
                self.vertex_offset[*i] = self.chunk_world_vertices.len() as u64;
                self.chunk_world_vertices.extend(&self.vertex_data[*i]);
            }
        }
        if self.frame % 300 == 10 {
            self.init.queue.write_buffer(&self.world_vertex_buffer, 0, bytemuck::cast_slice(&self.chunk_world_vertices));
            self.world_num_vertices = self.chunk_world_vertices.len() as u32;
        }

        // this will update the distances for transparent chunks in a few different frames
        if self.frame % 300 == 150 {
            self.chunk_transparent_world_vertices.clear();

            let active_chunk_data;
            {
                active_chunk_data = self.world_data.lock().unwrap();
            }

            let total_chunks = active_chunk_data.active_chunks.len();
            let start = 0;
            let end = total_chunks / 5;
            for i in &active_chunk_data.active_chunks[start..end] {
                self.vertex_offset_transparent[*i] = self.chunk_transparent_world_vertices.len() as u64;
                self.chunk_transparent_world_vertices.extend(&self.vertex_data_transparent[*i]);
            }
        }
        if self.frame % 300 == 152 {
            let active_chunk_data;
            {
                active_chunk_data = self.world_data.lock().unwrap();
            }

            let total_chunks = active_chunk_data.active_chunks.len();
            let start = total_chunks / 5;
            let end = 2 * total_chunks / 5;
            for i in &active_chunk_data.active_chunks[start..end] {
                self.vertex_offset_transparent[*i] = self.chunk_transparent_world_vertices.len() as u64;
                self.chunk_transparent_world_vertices.extend(&self.vertex_data_transparent[*i]);
            }
        }
        if self.frame % 300 == 154 {
            let active_chunk_data;
            {
                active_chunk_data = self.world_data.lock().unwrap();
            }

            let total_chunks = active_chunk_data.active_chunks.len();
            let start = 2 * total_chunks / 5;
            let end = 3 * total_chunks / 5;
            for i in &active_chunk_data.active_chunks[start..end] {
                self.vertex_offset_transparent[*i] = self.chunk_transparent_world_vertices.len() as u64;
                self.chunk_transparent_world_vertices.extend(&self.vertex_data_transparent[*i]);
            }
        }
        if self.frame % 300 == 156 {
            let active_chunk_data;
            {
                active_chunk_data = self.world_data.lock().unwrap();
            }

            let total_chunks = active_chunk_data.active_chunks.len();
            let start = 3 * total_chunks / 5;
            let end = 4 * total_chunks / 5;
            for i in &active_chunk_data.active_chunks[start..end] {
                self.vertex_offset_transparent[*i] = self.chunk_transparent_world_vertices.len() as u64;
                self.chunk_transparent_world_vertices.extend(&self.vertex_data_transparent[*i]);
            }
        }
        if self.frame % 300 == 158 {
            let active_chunk_data;
            {
                active_chunk_data = self.world_data.lock().unwrap();
            }

            let total_chunks = active_chunk_data.active_chunks.len();
            let start = 4 * total_chunks / 5;
            let end = total_chunks;
            for i in &active_chunk_data.active_chunks[start..end] {
                self.vertex_offset_transparent[*i] = self.chunk_transparent_world_vertices.len() as u64;
                self.chunk_transparent_world_vertices.extend(&self.vertex_data_transparent[*i]);
            }
        }
        if self.frame % 300 == 160 {
            self.init.queue.write_buffer(&self.world_vertex_buffer_transparent, 0, bytemuck::cast_slice(&self.chunk_transparent_world_vertices));
            self.world_num_vertices_transparent = self.chunk_transparent_world_vertices.len() as u32;
        }

        let up_direction = cgmath::Vector3::unit_y();
        let (view_mat, project_mat, _) = transforms::create_view_rotation(
            self.game_data.camera_position, self.game_data.camera_rotation[1], self.game_data.camera_rotation[0], 
            up_direction, self.init.config.width as f32 / self.init.config.height as f32, IS_PERSPECTIVE);

        // update uniform buffer
        let _dt = ANIMATION_SPEED * dt.as_secs_f32(); 
        let view_project_mat = project_mat * view_mat;
        let view_projection_ref:&[f32; 16] = view_project_mat.as_ref();
        
        self.init.queue.write_buffer(&self.vertex_uniform_buffer, 64, bytemuck::cast_slice(view_projection_ref));

        if render_ui {
            if let Some(is_pressed) = keys_down.get("number") {
                if is_pressed == &true {
                    update_frame(&mut self.game_data, (-0.11 * (slot_selected as f64 - 4.0), -0.6, 0.0), (0.04, 0.04, 0.04), [0.5, 0.5, 0.5], vec![[0.007, 0.054], [0.07, 0.054], [0.07, 0.117], [0.007, 0.054], [0.07, 0.117], [0.007, 0.117]], 2);
                    let mut gui_element: Vec<Vertex> = Vec::new();
                    for i in 0..self.game_data.gui_objects.len() {
                        gui_element.extend(&self.game_data.gui_objects[i]);
                    }
                    self.init.queue.write_buffer(&self.gui_vertex_buffer, 0, bytemuck::cast_slice(&gui_element));
                    self.gui_num_vertices_ = gui_element.len() as u32;
                }
            }

            let rotation_x = -self.game_data.camera_rotation.x;
            let rotation_y = -self.game_data.camera_rotation.y + std::f32::consts::FRAC_PI_2;
            let rotation_z = -self.game_data.camera_rotation.z;

            let position_x = forward.x + self.game_data.camera_position.x;
            let position_y = forward.y + self.game_data.camera_position.y;
            let position_z = forward.z + self.game_data.camera_position.z;
            let model_mat = transforms::create_transforms(
                [position_x, position_y, position_z], 
                [rotation_x, rotation_y, rotation_z], 
                [1.0, 1.0, 1.0]);
            let normal_mat = (model_mat.invert().unwrap()).transpose();
            let model_ref:&[f32; 16] = model_mat.as_ref();
            let normal_ref:&[f32; 16] = normal_mat.as_ref();

            let mut combined_data = [0u8; 192];
            combined_data[..64].copy_from_slice(bytemuck::cast_slice(model_ref));
            combined_data[64..128].copy_from_slice(bytemuck::cast_slice(view_projection_ref));
            combined_data[128..192].copy_from_slice(bytemuck::cast_slice(normal_ref));

            self.init.queue.write_buffer(&self.gui_vertex_uniform_buffer, 0, &combined_data);
            self.init.queue.write_buffer(&self.gui_item_block_vertex_uniform_buffer, 0, &combined_data);
            self.init.queue.write_buffer(&self.text_vertex_uniform_buffer, 0, &combined_data);
        }

        //let current_time_updated = std::time::Instant::now();
        //let update_time = current_time_updated.duration_since(current_time).as_secs_f32();
        //println!("update time: {:.4}ms amount of vertices solid: {} transparent: {} fps: {:.2}", update_time * 1000.0, self.world_num_vertices, self.world_num_vertices_transparent, 1.0 / update_time);
        //println!("fps: {}", 1.0 / update_time);
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

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.world_vertex_buffer.slice(..));           
            render_pass.set_bind_group(0, &self.world_uniform_bind_group, &[]);
            render_pass.draw(0..self.world_num_vertices, 0..1);

            render_pass.set_pipeline(&self.pipeline_transparent);
            render_pass.set_vertex_buffer(0, self.world_vertex_buffer_transparent.slice(..));           
            render_pass.set_bind_group(0, &self.world_uniform_bind_group_transparent, &[]);
            render_pass.draw(0..self.world_num_vertices_transparent, 0..1);

            if self.render_ui {
                render_pass.set_pipeline(&self.gui_pipeline);
                render_pass.set_vertex_buffer(0, self.gui_vertex_buffer.slice(..));           
                render_pass.set_bind_group(0, &self.gui_uniform_bind_group, &[]);
                render_pass.draw(0..self.gui_num_vertices_, 0..1);

                render_pass.set_pipeline(&self.gui_item_block_pipeline);
                render_pass.set_vertex_buffer(0, self.gui_item_block_vertex_buffer.slice(..));           
                render_pass.set_bind_group(0, &self.gui_item_block_uniform_bind_group, &[]);
                render_pass.draw(0..self.gui_item_block_num_vertices_, 0..1);

                render_pass.set_pipeline(&self.text_pipeline);
                render_pass.set_vertex_buffer(0, self.text_vertex_buffer.slice(..));           
                render_pass.set_bind_group(0, &self.text_uniform_bind_group, &[]);
                render_pass.draw(0..self.text_num_vertices_, 0..1);
            }
        }

        self.init.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub fn load_texture_atlasses(world_data_thread: &Arc<Mutex<world::WorldData>>) {
    let texture_data = Assets::get("textures/blocks/atlas.png").expect("Failed to load embedded texture");
    let img = image::load_from_memory(&texture_data.data).expect("Failed to load texture");
    println!("loaded blocks/atlas");
    let rgba = img.to_rgba8();
    let (width, height) = img.dimensions();
    let texture_size = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    {
        let mut world_data = world_data_thread.lock().unwrap();
        world_data.textures.push((rgba, texture_size, width, height));
    }
}
fn handle_structure_data(world_data: &mut world::WorldData, json_content: &str) {
    let structure_data: StructureData = serde_json::from_str(json_content).expect("Failed to parse JSON");
    world_data.add_structure(
        structure_data.structure_name,
        structure_data.blocks
    );
}
fn handle_biome_data(world_data: &mut world::WorldData, json_content: &str) {
    let biome_data: BiomeData = serde_json::from_str(json_content).expect("Failed to parse JSON");
    world_data.add_biome(
        biome_data.biome_name,
        biome_data.temperature,
        biome_data.moisture,
        biome_data.height,
        biome_data.block_levels,
        biome_data.sea_level,
        biome_data.trees,
        biome_data.folliage,
        biome_data.buildings
    );
}
fn handle_shape_data(world_data: &mut world::WorldData, json_content: &str) {
    let shape_data: ShapeData = serde_json::from_str(json_content).expect("Failed to parse JSON");
    world_data.add_shape(
        shape_data.shape_name,
        shape_data.elements
    );
}
pub fn load_shape_files(world_data_thread: &Arc<Mutex<world::WorldData>>, modding_allowed: bool) {
    let mut world_data = world_data_thread.lock().unwrap();
    let mut json_files = Vec::new();
    let exe_dir: &Path = Path::new("");
    if modding_allowed {
        let exe_path = std::env::current_exe().expect("Failed to get current executable path");
        let exe_dir = exe_path.parent().expect("Failed to get executable directory");
        let models_dir = exe_dir.join("assets/models/shapes");

        if models_dir.exists() && models_dir.is_dir() {
            println!("Found the modded directory for shapes");
            for entry in fs::read_dir(&models_dir).expect("Failed to read models directory") {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "json") {
                        if let Some(file_name) = path.strip_prefix(&exe_dir).ok().and_then(|p| p.to_str()) {
                            println!("Found the modded shape file: {}", file_name);
                            json_files.push(file_name.to_string());
                        }
                    }
                }
            }
        }
    }
    json_files.extend(
        Assets::iter()
            .filter(|file| file.starts_with("models/shapes/") && file.ends_with(".json"))
            .map(|file| file.to_string())
    );
    json_files.sort();
    json_files.dedup();
    for file in json_files {
        println!("Found JSON file: {}", file);
        let file_path = exe_dir.join(&file);
        if file_path.exists() {
            let mut file_content = String::new();
            let mut file = fs::File::open(&file_path).expect("Failed to open file");
            file.read_to_string(&mut file_content).expect("Failed to read file");
            handle_shape_data(&mut world_data, &file_content);
        } else if let Some(asset) = Assets::get(&file) {
            let json_content = std::str::from_utf8(asset.data.as_ref()).expect("Invalid UTF-8");
            handle_shape_data(&mut world_data, json_content);
        }
    }
}
pub fn load_biome_files(world_data_thread: &Arc<Mutex<world::WorldData>>, modding_allowed: bool) {
    let mut world_data = world_data_thread.lock().unwrap();
    let mut json_files = Vec::new();
    let mut exe_dir: PathBuf = PathBuf::new();
    if modding_allowed {
        let exe_path = std::env::current_exe().expect("Failed to get current executable path");
        exe_dir = exe_path.parent().expect("Failed to get executable directory").to_path_buf();
        let models_dir = exe_dir.join("assets/biomes");
        if models_dir.exists() && models_dir.is_dir() {
            println!("Found the modded directory for biomes");
            for entry in fs::read_dir(&models_dir).expect("Failed to read models directory") {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "json") {
                        if let Some(file_name) = path.strip_prefix(&exe_dir).ok().and_then(|p| p.to_str()) {
                            println!("Found the modded biome file: {}", file_name);
                            json_files.push(file_name.to_string());
                        }
                    }
                }
            }
        }
    }
    json_files.extend(
        Assets::iter()
            .filter(|file| file.starts_with("biomes/") && file.ends_with(".json"))
            .map(|file| file.to_string())
    );
    json_files.sort();
    json_files.dedup();
    for file in json_files {
        println!("Found JSON file: {}", file);
        let file_path = exe_dir.join(&file);
        if file_path.exists() {
            let mut file_content = String::new();
            let mut file = fs::File::open(&file_path).expect("Failed to open file");
            file.read_to_string(&mut file_content).expect("Failed to read file");
            handle_biome_data(&mut world_data, &file_content);
        } else if let Some(asset) = Assets::get(&file) {
            let json_content = std::str::from_utf8(asset.data.as_ref()).expect("Invalid UTF-8");
            handle_biome_data(&mut world_data, json_content);
        }
    }
}
pub fn load_structure_files(world_data_thread: &Arc<Mutex<world::WorldData>>, modding_allowed: bool) {
    let mut world_data = world_data_thread.lock().unwrap();
    let mut json_files = Vec::new();
    let exe_dir: &Path = Path::new("");
    if modding_allowed {
        let exe_path = std::env::current_exe().expect("Failed to get current executable path");
        let exe_dir = exe_path.parent().expect("Failed to get executable directory");
        let models_dir = exe_dir.join("assets/structures");

        if models_dir.exists() && models_dir.is_dir() {
            println!("Found the modded directory for structures");
            for entry in fs::read_dir(&models_dir).expect("Failed to read models directory") {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "json") {
                        if let Some(file_name) = path.strip_prefix(&exe_dir).ok().and_then(|p| p.to_str()) {
                            println!("Found the modded structure file: {}", file_name);
                            json_files.push(file_name.to_string());
                        }
                    }
                }
            }
        }
    }
    json_files.extend(
        Assets::iter()
            .filter(|file| file.starts_with("structures/") && file.ends_with(".json"))
            .map(|file| file.to_string())
    );
    json_files.sort();
    json_files.dedup();
    for file in json_files {
        println!("Found JSON file: {}", file);
        let file_path = exe_dir.join(&file);
        if file_path.exists() {
            let mut file_content = String::new();
            let mut file = fs::File::open(&file_path).expect("Failed to open file");
            file.read_to_string(&mut file_content).expect("Failed to read file");
            handle_structure_data(&mut world_data, &file_content);
        } else if let Some(asset) = Assets::get(&file) {
            let json_content = std::str::from_utf8(asset.data.as_ref()).expect("Invalid UTF-8");
            handle_structure_data(&mut world_data, json_content);
        }
    }
}
fn handle_model_data(world_data: &mut world::WorldData, json_content: &str) {
    let model_data: ModelData = serde_json::from_str(json_content).expect("Failed to parse JSON");
    world_data.add_block(
        model_data.block_name,
        vec![
            model_data.textures.right(),
            model_data.textures.left(),
            model_data.textures.top(),
            model_data.textures.bottom(),
            model_data.textures.front(),
            model_data.textures.back(),
        ],
        model_data.creator,
        model_data.shape,
        model_data.sides,
        model_data.transparent,
        model_data.collide,
    );
}
pub fn load_block_model_files(world_data_thread: &Arc<Mutex<world::WorldData>>, modding_allowed: bool) {
    let mut json_files = Vec::new();
    let mut world_data = world_data_thread.lock().unwrap();
    let exe_dir: &Path = Path::new("");
    if modding_allowed {
        let exe_path = std::env::current_exe().expect("Failed to get current executable path");
        let exe_dir = exe_path.parent().expect("Failed to get executable directory");
        let models_dir = exe_dir.join("assets/models/blocks");
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
    }
    json_files.extend(
        Assets::iter()
            .filter(|file| file.starts_with("models/blocks/") && file.ends_with(".json"))
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

fn handle_audio_data_wav(world_data: &mut world::WorldData, file_content: &[u8]) {
    let cursor = std::io::Cursor::new(file_content);
    let mut reader = hound::WavReader::new(cursor).expect("Failed to read WAV data");

    match reader.spec().sample_format {
        hound::SampleFormat::Int => {
            let samples: Vec<i16> = reader
                .samples::<i16>()
                .map(|s| s.expect("Failed to read sample"))
                .collect();
            world_data.audio_files.push(vec![samples]);
        }
        hound::SampleFormat::Float => {
            let samples: Vec<i16> = reader
                .samples::<f32>()
                .map(|s| (s.expect("Failed to read sample") * i16::MAX as f32) as i16)
                .collect();
            world_data.audio_files.push(vec![samples]);
        }
    }
}
fn handle_audio_data_ogg(world_data: &mut world::WorldData, file_content: &[u8]) {
    let cursor = Cursor::new(file_content);
    let mut reader = OggStreamReader::new(cursor).expect("Failed to read OGG data");

    let mut all_samples: Vec<Vec<i16>> = Vec::new();
    while let Some(packet) = reader.read_dec_packet().expect("Failed to read packet") {
        for (channel_index, channel_samples) in packet.iter().enumerate() {
            if all_samples.len() <= channel_index {
                all_samples.push(vec![]);
            }
            all_samples[channel_index].extend(channel_samples);
        }
    }
    world_data.audio_files.push(all_samples);
}

pub fn load_audio_files(world_data_thread: &Arc<Mutex<world::WorldData>>, modding_allowed: bool) {
    let mut audio_files = Vec::new();
    let mut world_data = world_data_thread.lock().unwrap();
    let exe_dir: &Path = Path::new("");
    if modding_allowed {
        let exe_path = std::env::current_exe().expect("Failed to get current executable path");
        let exe_dir = exe_path.parent().expect("Failed to get executable directory");
        let sounds_dir = exe_dir.join("assets/sounds");
        if sounds_dir.exists() && sounds_dir.is_dir() {
            println!("Found the modded directory for audio");
            for entry in fs::read_dir(&sounds_dir).expect("Failed to read models directory") {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "wav") {
                        if let Some(file_name) = path.strip_prefix(&exe_dir).ok().and_then(|p| p.to_str()) {
                            println!("Found the modded audio file: {}", file_name);
                            audio_files.push((file_name.to_string(), false));
                        }
                    }
                    if path.extension().map_or(false, |ext| ext == "ogg") {
                        if let Some(file_name) = path.strip_prefix(&exe_dir).ok().and_then(|p| p.to_str()) {
                            println!("Found the modded audio file: {}", file_name);
                            audio_files.push((file_name.to_string(), true));
                        }
                    }
                }
            }
        }
    }
    audio_files.extend(
        Assets::iter()
            .filter(|file| file.starts_with("sounds/") && file.ends_with(".wav"))
            .map(|file| (file.to_string(), false))
    );
    audio_files.extend(
        Assets::iter()
            .filter(|file| file.starts_with("sounds/") && file.ends_with(".ogg"))
            .map(|file| (file.to_string(), true))
    );
    audio_files.sort();
    audio_files.dedup();
    for file in audio_files {
        println!("Found audio file: {}", file.0);
        let file_path = exe_dir.join(&file.0);
        if !file.1 {
            if file_path.exists() {
                let file_content = fs::read(file_path).expect("Failed to read audio file");
                handle_audio_data_wav(&mut world_data, &file_content);
            } else if let Some(asset) = Assets::get(&file.0) {
                let file_content = asset.data.as_ref();
                handle_audio_data_wav(&mut world_data, &file_content);
            }
        } else {
            if file_path.exists() {
                let file_content = fs::read(file_path).expect("Failed to read audio file");
                handle_audio_data_ogg(&mut world_data, &file_content);
            } else if let Some(asset) = Assets::get(&file.0) {
                let file_content = asset.data.as_ref();
                handle_audio_data_ogg(&mut world_data, &file_content);
            }
        }
    }
}

fn load_icon_from_bytes(data: &[u8]) -> Option<Icon> {
    let img = image::load(Cursor::new(data), image::ImageFormat::Png).ok()?;
    let (width, height) = img.dimensions();
    let rgba = img.into_rgba8().into_raw();

    Icon::from_rgba(rgba, width, height).ok()
}

pub fn run(game_data: GameData, world_data: Arc<Mutex<world::WorldData>>, inventory: Inventory, light_data: Light, title: &str, chunk_data_terrain: Arc<Mutex<HashMap<(i64, i64, i64), Vec<i8>>>>, chunk_data_lighting: Arc<Mutex<HashMap<(i64, i64, i64), Vec<i8>>>>) {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title(title);

    let logo_data = Assets::get("logo.png").expect("Logo file not found in assets");
    let icon = load_icon_from_bytes(logo_data.data.as_ref());

    window.set_window_icon(icon);
    
    if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::Confined) {
        eprintln!("Failed to lock the cursor: {:?}", err);
    }
    window.set_cursor_visible(false);

    let mut game_state = pollster::block_on(State::new(&window, game_data, light_data, world_data, chunk_data_terrain, chunk_data_lighting));    
    let render_start_time = std::time::Instant::now();

    let mut keys_down: HashMap<&str, bool> = HashMap::new();
    keys_down.insert("w", false);
    keys_down.insert("a", false);
    keys_down.insert("s", false);
    keys_down.insert("d", false);
    keys_down.insert("space", false);
    keys_down.insert("number", false);
    let mut slot_selected: i8 = 0;
    let mut mouse_movement: Vec<f64> = vec![0.0, 0.0];
    let mut mouse_locked = true;
    let mut render_ui = true;

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
                                        &VirtualKeyCode::U => { render_ui = !render_ui; }
                                        &VirtualKeyCode::Space => { keys_down.insert("space", true); }
                                        &VirtualKeyCode::Right => { keys_down.insert("right", true); }
                                        &VirtualKeyCode::Left => { keys_down.insert("left", true); }
                                        &VirtualKeyCode::Key1 => { slot_selected = 0; keys_down.insert("number", true); }
                                        &VirtualKeyCode::Key2 => { slot_selected = 1; keys_down.insert("number", true); }
                                        &VirtualKeyCode::Key3 => { slot_selected = 2; keys_down.insert("number", true); }
                                        &VirtualKeyCode::Key4 => { slot_selected = 3; keys_down.insert("number", true); }
                                        &VirtualKeyCode::Key5 => { slot_selected = 4; keys_down.insert("number", true); }
                                        &VirtualKeyCode::Key6 => { slot_selected = 5; keys_down.insert("number", true); }
                                        &VirtualKeyCode::Key7 => { slot_selected = 6; keys_down.insert("number", true); }
                                        &VirtualKeyCode::Key8 => { slot_selected = 7; keys_down.insert("number", true); }
                                        &VirtualKeyCode::Key9 => { slot_selected = 8; keys_down.insert("number", true); }
                                        //&VirtualKeyCode::Key0 => { slot_selected = 9; }
                                        &VirtualKeyCode::Escape | &VirtualKeyCode::LWin | &VirtualKeyCode::RWin => {
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
                        WindowEvent::Focused(focused) => {
                            if !focused {
                                mouse_locked = false;
                                if let Err(err) = window.set_cursor_grab(winit::window::CursorGrabMode::None) {
                                    eprintln!("Failed to unlock the cursor: {:?}", err);
                                }
                                window.set_cursor_visible(true);
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
                                            game_state.mouse_input(0, slot_selected, inventory.clone());
                                        }
                                        MouseButton::Right => {
                                            game_state.mouse_input(1, slot_selected, inventory.clone());
                                        }
                                        MouseButton::Middle => {
                                            game_state.mouse_input(2, slot_selected, inventory.clone());
                                        }
                                        _ => {}
                                    }
                                }
                                ElementState::Released => {
                                    return
                                }
                            }
                        }
                        WindowEvent::MouseWheel { delta, .. } => {
                            match delta {
                                MouseScrollDelta::LineDelta(_x, y) => {
                                    slot_selected = slot_selected - y.floor() as i8;
                                    if slot_selected < 0 { slot_selected = 8; }
                                    if slot_selected > 8 { slot_selected = 0; }
                                    keys_down.insert("number", true);
                                }
                                MouseScrollDelta::PixelDelta(_pos) => {}
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
                game_state.update(dt, &keys_down, &mouse_movement, slot_selected, render_ui);

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