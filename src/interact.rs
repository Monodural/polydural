use crate::{chunk, common, containers, world};
use std::{collections::HashMap, sync::Arc};
use cgmath::*;

pub fn break_block(game_data: &mut common::GameData, chunks_thread: &Arc<std::sync::Mutex<HashMap<(i64, i64, i64), Vec<i8>>>>, lighting_chunks_thread: &Arc<std::sync::Mutex<HashMap<(i64, i64, i64), Vec<i8>>>>, world_data: &Arc<std::sync::Mutex<world::WorldData>>) -> (Vec<common::Vertex>, i32, Vec<common::Vertex>) {
    let world_data_read = world_data.lock().unwrap().clone();
    let chunks = chunks_thread.lock().unwrap().clone();
    let lighting_chunks = lighting_chunks_thread.lock().unwrap().clone();
    
    let forward = cgmath::Vector3::new(
        game_data.camera_rotation[1].cos() * game_data.camera_rotation[0].cos(),
        game_data.camera_rotation[0].sin(),
        game_data.camera_rotation[1].sin() * game_data.camera_rotation[0].cos(),
    ).normalize();

    let mut found_block = false;
    for i in 0..16 {
        if found_block { continue; }
        let block_type = chunk::get_block_global(game_data, &chunks, &world_data_read, 
            (game_data.camera_position[0] + forward.x * i as f32) / 2.0, 
            (game_data.camera_position[1] + forward.y * i as f32) / 2.0, 
            (game_data.camera_position[2] + forward.z * i as f32) / 2.0
        );
        if block_type != 0 && block_type != -1 {
            found_block = true;

            let x = (game_data.camera_position[0] + forward.x * i as f32) / 2.0;
            let y = (game_data.camera_position[1] + forward.y * i as f32) / 2.0;
            let z = (game_data.camera_position[2] + forward.z * i as f32) / 2.0;

            let chunk_position_x: i64 = ((x + 0.5) / 32.0).floor() as i64;
            let chunk_position_y: i64 = ((y + 0.5) / 32.0).floor() as i64;
            let chunk_position_z: i64 = ((z + 0.5) / 32.0).floor() as i64;

            let mut local_position_x = ((x + 0.5) % 32.0).floor() as i8;
            let mut local_position_y = ((y + 0.5) % 32.0).floor() as i8;
            let mut local_position_z = ((z + 0.5) % 32.0).floor() as i8;
            if local_position_x < 0 { local_position_x = 32 + local_position_x; }
            if local_position_y < 0 { local_position_y = 32 + local_position_y; }
            if local_position_z < 0 { local_position_z = 32 + local_position_z; }

            if let Some(chunk) = chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z)) {
                let x: i8 = local_position_x;
                let y: i8 = local_position_y;
                let z: i8 = local_position_z;

                let light_data = &lighting_chunks[&(chunk_position_x, chunk_position_y, chunk_position_z)];
                let chunk_data = chunk::set_block(chunk.clone(), local_position_x, local_position_y, local_position_z, world_data_read.block_index["air"] as i8);
                let (chunk_vertices, chunk_normals, chunk_colors, chunk_uvs, 
                    chunk_vertices_transparent, chunk_normals_transparent, chunk_colors_transparent, chunk_uvs_transparent
                    ) = chunk::render_chunk(&chunk_data, &light_data, &game_data, &chunks, &world_data_read, 
                    chunk_position_x, chunk_position_y, chunk_position_z
                );

                {
                    let mut world_data_reading = world_data.lock().unwrap();

                    world_data_reading.set_chunk(chunk_position_x, chunk_position_y, chunk_position_z, chunk_data, light_data.clone());
                    if x == 0 || x == 31 || y == 0 || y == 31 || z == 0 || z == 31 {
                        let chunk_buffer_index_read = &world_data_reading.chunk_buffer_index.clone();
                        let chunk_update_queue = &mut world_data_reading.chunk_update_queue;

                        if x == 0 {
                            chunk_update_queue.push(chunk_buffer_index_read[&(chunk_position_x - 1, chunk_position_y, chunk_position_z)] as usize);
                        } else if x == 31 {
                            chunk_update_queue.push(chunk_buffer_index_read[&(chunk_position_x + 1, chunk_position_y, chunk_position_z)] as usize);
                        }
                        if y == 0 {
                            chunk_update_queue.push(chunk_buffer_index_read[&(chunk_position_x, chunk_position_y - 1, chunk_position_z)] as usize);
                        } else if y == 31 {
                            chunk_update_queue.push(chunk_buffer_index_read[&(chunk_position_x, chunk_position_y + 1, chunk_position_z)] as usize);
                        }
                        if z == 0 {
                            chunk_update_queue.push(chunk_buffer_index_read[&(chunk_position_x, chunk_position_y, chunk_position_z - 1)] as usize);
                        } else if z == 31 {
                            chunk_update_queue.push(chunk_buffer_index_read[&(chunk_position_x, chunk_position_y, chunk_position_z + 1)] as usize);
                        }
                    }
                }

                let vertex_data_chunk = common::create_vertices(chunk_vertices, chunk_normals, chunk_colors, chunk_uvs);
                let vertex_data_chunk_transparent = common::create_vertices(chunk_vertices_transparent, chunk_normals_transparent, chunk_colors_transparent, chunk_uvs_transparent);
        
                let mut buffer_index: usize = 0;
                if let Some(chunk_index) = world_data_read.chunk_buffer_index.get(&(chunk_position_x, chunk_position_y, chunk_position_z)) {
                    buffer_index = *chunk_index as usize;
                }
        
                return (vertex_data_chunk, buffer_index as i32, vertex_data_chunk_transparent);
            }
        }
    }
    return (Vec::new(), -1, Vec::new());
}

pub fn place_block(game_data: &mut common::GameData, chunks_thread: &Arc<std::sync::Mutex<HashMap<(i64, i64, i64), Vec<i8>>>>, lighting_chunks_thread: &Arc<std::sync::Mutex<HashMap<(i64, i64, i64), Vec<i8>>>>, world_data: &Arc<std::sync::Mutex<world::WorldData>>, slot_selected: i8, inventory: containers::Inventory) -> (Vec<common::Vertex>, i32, Vec<common::Vertex>) {
    let world_data_read = world_data.lock().unwrap().clone();
    let chunks = chunks_thread.lock().unwrap().clone();
    let lighting_chunks = lighting_chunks_thread.lock().unwrap().clone();
    
    let forward = cgmath::Vector3::new(
        game_data.camera_rotation[1].cos() * game_data.camera_rotation[0].cos(),
        game_data.camera_rotation[0].sin(),
        game_data.camera_rotation[1].sin() * game_data.camera_rotation[0].cos(),
    ).normalize();

    let mut found_block = false;
    for mut i in 0..16 {
        if found_block { continue; }
        let block_type = chunk::get_block_global(game_data, &chunks, &world_data_read, 
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

            let chunk_position_x: i64 = ((x + 0.5) / 32.0).floor() as i64;
            let chunk_position_y: i64 = ((y + 0.5) / 32.0).floor() as i64;
            let chunk_position_z: i64 = ((z + 0.5) / 32.0).floor() as i64;

            let mut local_position_x = ((x + 0.5) % 32.0).floor() as i8;
            let mut local_position_y = ((y + 0.5) % 32.0).floor() as i8;
            let mut local_position_z = ((z + 0.5) % 32.0).floor() as i8;
            if local_position_x < 0 { local_position_x = 32 + local_position_x; }
            if local_position_y < 0 { local_position_y = 32 + local_position_y; }
            if local_position_z < 0 { local_position_z = 32 + local_position_z; }
            
            if let Some(chunk) = chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z)) {
                let light_data = &lighting_chunks[&(chunk_position_x, chunk_position_y, chunk_position_z)];
                let x: i8 = local_position_x;
                let y: i8 = local_position_y;
                let z: i8 = local_position_z;

                let selected_item = &inventory.hotbar_slots[slot_selected as usize].0;
                let chunk_data = chunk::set_block(chunk.clone(), x, y, z, world_data_read.block_index[selected_item] as i8);
                
                {
                    let mut world_data_reading = world_data.lock().unwrap();

                    world_data_reading.set_chunk(chunk_position_x, chunk_position_y, chunk_position_z, chunk_data.clone(), light_data.clone());
                    if x == 0 || x == 31 || y == 0 || y == 31 || z == 0 || z == 31 {
                        if x == 0 {
                            world_data_reading.chunk_update_queue.push(world_data_read.chunk_buffer_index[&(chunk_position_x - 1, chunk_position_y, chunk_position_z)] as usize);
                        } else if x == 31 {
                            world_data_reading.chunk_update_queue.push(world_data_read.chunk_buffer_index[&(chunk_position_x + 1, chunk_position_y, chunk_position_z)] as usize);
                        }
                        if y == 0 {
                            world_data_reading.chunk_update_queue.push(world_data_read.chunk_buffer_index[&(chunk_position_x, chunk_position_y - 1, chunk_position_z)] as usize);
                        } else if y == 31 {
                            world_data_reading.chunk_update_queue.push(world_data_read.chunk_buffer_index[&(chunk_position_x, chunk_position_y + 1, chunk_position_z)] as usize);
                        }
                        if z == 0 {
                            world_data_reading.chunk_update_queue.push(world_data_read.chunk_buffer_index[&(chunk_position_x, chunk_position_y, chunk_position_z - 1)] as usize);
                        } else if z == 31 {
                            world_data_reading.chunk_update_queue.push(world_data_read.chunk_buffer_index[&(chunk_position_x, chunk_position_y, chunk_position_z + 1)] as usize);
                        }
                    }
                }

                let (chunk_vertices, chunk_normals, chunk_colors, chunk_uvs,
                    chunk_vertices_transparent, chunk_normals_transparent, chunk_colors_transparent, chunk_uvs_transparent
                    ) = chunk::render_chunk(&chunk_data, &light_data, &game_data, &chunks, &world_data_read, 
                    chunk_position_x, chunk_position_y, chunk_position_z
                );
                let vertex_data_chunk = common::create_vertices(chunk_vertices, chunk_normals, chunk_colors, chunk_uvs);
                let vertex_data_chunk_transparent = common::create_vertices(chunk_vertices_transparent, chunk_normals_transparent, chunk_colors_transparent, chunk_uvs_transparent);
        
                let mut buffer_index: usize = 0;
                if let Some(chunk_index) = world_data_read.chunk_buffer_index.get(&(chunk_position_x, chunk_position_y, chunk_position_z)) {
                    buffer_index = *chunk_index as usize;
                }
        
                return (vertex_data_chunk, buffer_index as i32, vertex_data_chunk_transparent);
            }
        }
    }
    return (Vec::new(), -1, Vec::new());
}