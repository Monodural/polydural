use crate::world;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Inventory {
    pub hotbar_slots: Vec<(String, i8)>
}
impl Inventory {
    pub fn new() -> Self {
        Inventory {
            hotbar_slots: vec![
                ("dirt".to_string(), 9), 
                ("grass_1".to_string(), 9), 
                ("grass_2".to_string(), 9), 
                ("oak_leaves".to_string(), 9), 
                ("oak_log".to_string(), 9), 
                ("stone".to_string(), 9), 
                ("sand_1".to_string(), 9), 
                ("sand_2".to_string(), 9), 
                ("cactus".to_string(), 9)
            ]
        }
    }

    pub fn render_item_block(world_data_thread: Arc<Mutex<world::WorldData>>, block_name: String) -> (Vec<[f64; 3]>, Vec<[f32; 2]>, Vec<[i8; 3]>, Vec<[f32; 3]>) {
        let world_data = world_data_thread.lock().unwrap();
        let mut vertices: Vec<[f64; 3]> = Vec::new();
        let mut normals: Vec<[i8; 3]> = Vec::new();
        let mut colors: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();

        let atlas_width = 8 as f32;
        let atlas_height = 8 as f32;

        if block_name == "air".to_string() { return (vertices, uvs, normals, colors); }

        let block_id = world_data.block_index[&block_name];

        vertices.push([ 1.0, -1.0,  1.0]);
        vertices.push([ 1.0, -1.0, -1.0]);
        vertices.push([ 1.0,  1.0,  1.0]);
        vertices.push([ 1.0,  1.0,  1.0]);
        vertices.push([ 1.0, -1.0, -1.0]);
        vertices.push([ 1.0,  1.0, -1.0]);

        let uv_x = (world_data.blocks[(block_id - 1) as usize].1[0] as f32 % atlas_width).floor();
        let uv_y = (world_data.blocks[(block_id - 1) as usize].1[0] as f32 / atlas_height).floor();
        uvs.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);

        normals.push([1, 0, 0]);
        normals.push([1, 0, 0]);
        normals.push([1, 0, 0]);
        normals.push([1, 0, 0]);
        normals.push([1, 0, 0]);
        normals.push([1, 0, 0]);

        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);

        vertices.push([-1.0, -1.0, -1.0]);
        vertices.push([-1.0, -1.0,  1.0]);
        vertices.push([-1.0,  1.0, -1.0]);
        vertices.push([-1.0,  1.0, -1.0]);
        vertices.push([-1.0, -1.0,  1.0]);
        vertices.push([-1.0,  1.0,  1.0]);

        let uv_x = (world_data.blocks[(block_id - 1) as usize].1[1] as f32 % atlas_width).floor();
        let uv_y = (world_data.blocks[(block_id - 1) as usize].1[1] as f32 / atlas_height).floor();
        uvs.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);

        normals.push([-1, 0, 0]);
        normals.push([-1, 0, 0]);
        normals.push([-1, 0, 0]);
        normals.push([-1, 0, 0]);
        normals.push([-1, 0, 0]);
        normals.push([-1, 0, 0]);

        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);

        vertices.push([-1.0,  1.0,  1.0]);
        vertices.push([ 1.0,  1.0,  1.0]);
        vertices.push([-1.0,  1.0, -1.0]);
        vertices.push([-1.0,  1.0, -1.0]);
        vertices.push([ 1.0,  1.0,  1.0]);
        vertices.push([ 1.0,  1.0, -1.0]);

        let uv_x = (world_data.blocks[(block_id - 1) as usize].1[2] as f32 % atlas_width).floor();
        let uv_y = (world_data.blocks[(block_id - 1) as usize].1[2] as f32 / atlas_height).floor();
        uvs.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);

        normals.push([0, 1, 0]);
        normals.push([0, 1, 0]);
        normals.push([0, 1, 0]);
        normals.push([0, 1, 0]);
        normals.push([0, 1, 0]);
        normals.push([0, 1, 0]);

        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);

        vertices.push([-1.0, -1.0, -1.0]);
        vertices.push([ 1.0, -1.0, -1.0]);
        vertices.push([-1.0, -1.0,  1.0]);
        vertices.push([-1.0, -1.0,  1.0]);
        vertices.push([ 1.0, -1.0, -1.0]);
        vertices.push([ 1.0, -1.0,  1.0]);

        let uv_x = (world_data.blocks[(block_id - 1) as usize].1[3] as f32 % atlas_width).floor();
        let uv_y = (world_data.blocks[(block_id - 1) as usize].1[3] as f32 / atlas_height).floor();
        uvs.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);

        normals.push([0, -1, 0]);
        normals.push([0, -1, 0]);
        normals.push([0, -1, 0]);
        normals.push([0, -1, 0]);
        normals.push([0, -1, 0]);
        normals.push([0, -1, 0]);

        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);

        vertices.push([-1.0, -1.0,  1.0]);
        vertices.push([ 1.0, -1.0,  1.0]);
        vertices.push([-1.0,  1.0,  1.0]);
        vertices.push([-1.0,  1.0,  1.0]);
        vertices.push([ 1.0, -1.0,  1.0]);
        vertices.push([ 1.0,  1.0,  1.0]);

        let uv_x = (world_data.blocks[(block_id - 1) as usize].1[4] as f32 % atlas_width).floor();
        let uv_y = (world_data.blocks[(block_id - 1) as usize].1[4] as f32 / atlas_height).floor();
        uvs.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);

        normals.push([0, 0, 1]);
        normals.push([0, 0, 1]);
        normals.push([0, 0, 1]);
        normals.push([0, 0, 1]);
        normals.push([0, 0, 1]);
        normals.push([0, 0, 1]);

        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);

        vertices.push([ 1.0, -1.0, -1.0]);
        vertices.push([-1.0, -1.0, -1.0]);
        vertices.push([ 1.0,  1.0, -1.0]);
        vertices.push([ 1.0,  1.0, -1.0]);
        vertices.push([-1.0, -1.0, -1.0]);
        vertices.push([-1.0,  1.0, -1.0]);

        let uv_x = (world_data.blocks[(block_id - 1) as usize].1[5] as f32 % atlas_width).floor();
        let uv_y = (world_data.blocks[(block_id - 1) as usize].1[5] as f32 / atlas_height).floor();
        uvs.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
        uvs.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);

        normals.push([0, 0, -1]);
        normals.push([0, 0, -1]);
        normals.push([0, 0, -1]);
        normals.push([0, 0, -1]);
        normals.push([0, 0, -1]);
        normals.push([0, 0, -1]);

        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);
        colors.push([1.0, 1.0, 1.0]);

        return (vertices, uvs, normals, colors);
    }
}