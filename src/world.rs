use std::collections::HashMap;
use std::collections::HashSet;
use cgmath::*;

use crate::common;

#[derive(Clone)]
pub struct WorldData {
    pub active_chunks: Vec<usize>,
    pub updated_chunks: Vec<usize>,
    pub updated_chunks_transparent: Vec<usize>,
    pub chunk_update_queue: Vec<usize>,
    pub chunk_queue: HashSet<(i64, i64, i64)>,
    pub created_chunk_queue: HashSet<(i64, i64, i64)>,
    pub shapes: Vec<(String, Vec<common::Element>)>,
    pub shape_index: HashMap<String, usize>,
    pub blocks: Vec<(String, Vec<i8>, String, String, bool, bool, bool)>,
    pub block_index: HashMap<String, usize>,
    pub chunks: HashMap<(i64, i64, i64), Vec<i8>>,
    pub chunk_buffer_index: HashMap<(i64, i64, i64), i64>,
    pub chunk_buffer_coordinates: Vec<(i64, i64, i64)>,
    pub updated_chunk_data: Vec<(usize, Vec<common::Vertex>)>,
    pub updated_chunk_data_transparent: Vec<(usize, Vec<common::Vertex>)>,
    pub created_chunk_data: Vec<(Vec<common::Vertex>, i64, i64, i64, Matrix4<f32>, Matrix4<f32>)>,
    pub created_chunk_data_transparent: Vec<(Vec<common::Vertex>, i64, i64, i64, Matrix4<f32>, Matrix4<f32>)>,
    pub textures: Vec<(image::ImageBuffer<image::Rgba<u8>, Vec<u8>>, wgpu::Extent3d, u32, u32)>,
    pub biomes: HashMap<String, (i8, i8, i8, Vec<(Vec<String>, i64)>, i64, Vec<(String, f32)>, Vec<(String, f32)>, Vec<(String, f32)>)>,
    pub structures: HashMap<String, Vec<common::Block>>
}
impl WorldData {
    pub fn new() -> Self {
        WorldData {
            active_chunks: Vec::new(),
            updated_chunks: Vec::new(),
            updated_chunks_transparent: Vec::new(),
            chunk_update_queue: Vec::new(),
            chunk_queue: HashSet::new(),
            created_chunk_queue: HashSet::new(),
            shapes: Vec::new(),
            shape_index: HashMap::new(),
            blocks: Vec::new(),
            block_index: HashMap::new(),
            chunks: HashMap::new(),
            chunk_buffer_index: HashMap::new(),
            chunk_buffer_coordinates: Vec::new(),
            updated_chunk_data: Vec::new(),
            updated_chunk_data_transparent: Vec::new(),
            created_chunk_data: Vec::new(),
            created_chunk_data_transparent: Vec::new(),
            textures: Vec::new(),
            biomes: HashMap::new(),
            structures: HashMap::new()
        }
    }

    pub fn set_chunk(&mut self, x: i64, y: i64, z: i64, chunk_data: Vec<i8>) {
        self.chunks.insert((x, y, z), chunk_data);
    }

    pub fn add_shape(&mut self, shape_name: String, elements: Vec<common::Element>) {
        if !self.shape_index.contains_key(&shape_name) {
            self.shapes.push((shape_name.clone(), elements));
            self.shape_index.insert(shape_name, self.shapes.len());
        }
    }
    pub fn add_block(&mut self, block_name: String, sides: Vec<i8>, owner: String, shape: String, render_sides: bool, transparent: bool, collide: bool) {
        if !self.block_index.contains_key(&block_name) {
            self.blocks.push((block_name.clone(), sides, owner, shape, render_sides, transparent, collide));
            self.block_index.insert(block_name, self.blocks.len());
        }
    }
    pub fn add_structure(&mut self, structure_name: String, blocks: Vec<common::Block>) {
        if !self.structures.contains_key(&structure_name) {
            self.structures.insert(structure_name, blocks);
        }
    }
    pub fn add_biome(&mut self, biome_name: String, temperature: i8, moisture: i8, height: i8, 
                    block_levels: Vec<(Vec<String>, i64)>, sea_level: i64, trees: Vec<(String, f32)>, 
                    folliage: Vec<(String, f32)>, buildings: Vec<(String, f32)>) {
        if !self.biomes.contains_key(&biome_name) {
            self.biomes.insert(biome_name, (temperature, moisture, height, block_levels, sea_level, trees, folliage, buildings));
        }
    }

    pub fn add_object(&mut self, position: (i64, i64, i64)) {
        self.chunk_buffer_index.insert(position, self.chunk_buffer_coordinates.len() as i64);
        self.chunk_buffer_coordinates.push(position);
    }
}