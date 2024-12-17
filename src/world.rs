use std::collections::HashMap;
use std::collections::HashSet;
use cgmath::*;

use crate::common;

#[derive(Clone)]
pub struct WorldData {
    pub active_chunks: Vec<usize>,
    pub updated_chunks: Vec<usize>,
    pub chunk_update_queue: Vec<usize>,
    pub chunk_queue: HashSet<(i64, i64, i64)>,
    pub created_chunk_queue: HashSet<(i64, i64, i64)>,
    pub blocks: Vec<(String, Vec<i8>, String)>,
    pub block_index: HashMap<String, usize>,
    pub chunks: HashMap<(i64, i64, i64), Vec<i8>>,
    pub chunk_buffer_index: HashMap<(i64, i64, i64), i64>,
    pub chunk_buffer_coordinates: Vec<(i64, i64, i64)>,
    pub updated_chunk_data: Vec<(usize, Vec<common::Vertex>)>,
    pub created_chunk_data: Vec<(Vec<common::Vertex>, i64, i64, i64, Matrix4<f32>, Matrix4<f32>)>
}
impl WorldData {
    pub fn new() -> Self {
        WorldData {
            active_chunks: Vec::new(),
            updated_chunks: Vec::new(),
            chunk_update_queue: Vec::new(),
            chunk_queue: HashSet::new(),
            created_chunk_queue: HashSet::new(),
            blocks: Vec::new(),
            block_index: HashMap::new(),
            chunks: HashMap::new(),
            chunk_buffer_index: HashMap::new(),
            chunk_buffer_coordinates: Vec::new(),
            updated_chunk_data: Vec::new(),
            created_chunk_data: Vec::new()
        }
    }

    pub fn set_chunk(&mut self, x: i64, y: i64, z: i64, chunk_data: Vec<i8>) {
        self.chunks.insert((x, y, z), chunk_data);
    }

    pub fn add_block(&mut self, block_name: String, sides: Vec<i8>, owner: String) {
        if !self.block_index.contains_key(&block_name) {
            self.blocks.push((block_name.clone(), sides, owner));
            self.block_index.insert(block_name, self.blocks.len());
        }
    }

    pub fn add_object(&mut self, position: (i64, i64, i64)) {
        self.chunk_buffer_index.insert(position, self.chunk_buffer_coordinates.len() as i64);
        self.chunk_buffer_coordinates.push(position);
    }
}