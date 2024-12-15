use crate::{chunk, common};
use cgmath::*;

pub fn break_block(game_data: &mut common::GameData) -> (Vec<common::Vertex>, i32) {
    let forward = cgmath::Vector3::new(
        game_data.camera_rotation[1].cos() * game_data.camera_rotation[0].cos(),
        game_data.camera_rotation[0].sin(),
        game_data.camera_rotation[1].sin() * game_data.camera_rotation[0].cos(),
    ).normalize();

    let mut found_block = false;
    for i in 0..16 {
        if found_block { continue; }
        let block_type = chunk::get_block_global(game_data, 
            (game_data.camera_position[0] + forward.x * i as f32) / 2.0, 
            (game_data.camera_position[1] + forward.y * i as f32) / 2.0, 
            (game_data.camera_position[2] + forward.z * i as f32) / 2.0
        );
        if block_type != 0 && block_type != -1 {
            found_block = true;

            let x = (game_data.camera_position[0] + forward.x * i as f32) / 2.0;
            let y = (game_data.camera_position[1] + forward.y * i as f32) / 2.0;
            let z = (game_data.camera_position[2] + forward.z * i as f32) / 2.0;

            let chunk_position_x: i64 = ((x + 0.5) / 16.0).floor() as i64;
            let chunk_position_y: i64 = ((y + 0.5) / 16.0).floor() as i64;
            let chunk_position_z: i64 = ((z + 0.5) / 16.0).floor() as i64;

            let mut local_position_x = ((x + 0.5) % 16.0).floor() as i8;
            let mut local_position_y = ((y + 0.5) % 16.0).floor() as i8;
            let mut local_position_z = ((z + 0.5) % 16.0).floor() as i8;
            if local_position_x < 0 { local_position_x = 16 + local_position_x; }
            if local_position_y < 0 { local_position_y = 16 + local_position_y; }
            if local_position_z < 0 { local_position_z = 16 + local_position_z; }

            if let Some(chunk) = game_data.chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z)) {
                let chunk_data = chunk::set_block(chunk.clone(), local_position_x, local_position_y, local_position_z, 0);
                game_data.set_chunk(chunk_position_x, chunk_position_y, chunk_position_z, chunk_data.clone());
                let (chunk_vertices, chunk_normals, chunk_colors, chunk_uvs) = chunk::render_chunk(&chunk_data, &game_data, 
                    chunk_position_x, chunk_position_y, chunk_position_z
                );
                let vertex_data_chunk = common::create_vertices(chunk_vertices, chunk_normals, chunk_colors, chunk_uvs);
        
                let mut buffer_index: usize = 0;
                if let Some(chunk_index) = game_data.chunk_buffer_index.get(&(chunk_position_x, chunk_position_y, chunk_position_z)) {
                    buffer_index = *chunk_index as usize;
                }
        
                return (vertex_data_chunk, buffer_index as i32);
            }
        }
    }
    return (Vec::new(), -1);
}

pub fn place_block(game_data: &mut common::GameData) -> (Vec<common::Vertex>, i32) {
    let forward = cgmath::Vector3::new(
        game_data.camera_rotation[1].cos() * game_data.camera_rotation[0].cos(),
        game_data.camera_rotation[0].sin(),
        game_data.camera_rotation[1].sin() * game_data.camera_rotation[0].cos(),
    ).normalize();

    let mut found_block = false;
    for mut i in 0..16 {
        if found_block { continue; }
        let block_type = chunk::get_block_global(game_data, 
            (game_data.camera_position[0] + forward.x * i as f32) / 2.0, 
            (game_data.camera_position[1] + forward.y * i as f32) / 2.0, 
            (game_data.camera_position[2] + forward.z * i as f32) / 2.0
        );
        if block_type != 0 && block_type != -1 {
            found_block = true;

            if i > 0 { i -= 1; }

            let x = (game_data.camera_position[0] + forward.x * i as f32) / 2.0;
            let y = (game_data.camera_position[1] + forward.y * i as f32) / 2.0;
            let z = (game_data.camera_position[2] + forward.z * i as f32) / 2.0;

            let chunk_position_x: i64 = ((x + 0.5) / 16.0).floor() as i64;
            let chunk_position_y: i64 = ((y + 0.5) / 16.0).floor() as i64;
            let chunk_position_z: i64 = ((z + 0.5) / 16.0).floor() as i64;

            let mut local_position_x = ((x + 0.5) % 16.0).floor() as i8;
            let mut local_position_y = ((y + 0.5) % 16.0).floor() as i8;
            let mut local_position_z = ((z + 0.5) % 16.0).floor() as i8;
            if local_position_x < 0 { local_position_x = 16 + local_position_x; }
            if local_position_y < 0 { local_position_y = 16 + local_position_y; }
            if local_position_z < 0 { local_position_z = 16 + local_position_z; }

            if let Some(chunk) = game_data.chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z)) {
                let x: i8 = local_position_x;
                let y: i8 = local_position_y;
                let z: i8 = local_position_z;

                let chunk_data = chunk::set_block(chunk.clone(), x, y, z, 6);
                game_data.set_chunk(chunk_position_x, chunk_position_y, chunk_position_z, chunk_data.clone());
                let (chunk_vertices, chunk_normals, chunk_colors, chunk_uvs) = chunk::render_chunk(&chunk_data, &game_data, 
                    chunk_position_x, chunk_position_y, chunk_position_z
                );
                let vertex_data_chunk = common::create_vertices(chunk_vertices, chunk_normals, chunk_colors, chunk_uvs);
        
                let mut buffer_index: usize = 0;
                if let Some(chunk_index) = game_data.chunk_buffer_index.get(&(chunk_position_x, chunk_position_y, chunk_position_z)) {
                    buffer_index = *chunk_index as usize;
                }
        
                return (vertex_data_chunk, buffer_index as i32);
            }
        }
    }
    return (Vec::new(), -1);
}