use std::collections::HashMap;
use crate::common;
use super::GameData;

pub fn update(game_data: &mut GameData, chunks: &HashMap<(i64, i64, i64), Vec<i8>>, blocks: &Vec<(String, Vec<i8>, String, String, bool, bool, bool)>, frame_time: f32) {
let block_type = get_block_global(game_data, &chunks, &blocks, 
        game_data.camera_position.x as f32 / 2.0, 
        game_data.camera_position.y as f32 / 2.0 - 2.0, 
        game_data.camera_position.z as f32 / 2.0
    );
    let grounded = block_type;
    game_data.grounded = grounded;

    if game_data.camera_acceleration_walking.x != 0.0 || game_data.camera_acceleration_walking.z != 0.0 {
        let block_type = get_block_global(game_data, &chunks, &blocks, 
            (game_data.camera_position.x + game_data.camera_acceleration_walking.x * 1.5) as f32 / 2.0, 
            game_data.camera_position.y as f32 / 2.0 - 1.5, 
            (game_data.camera_position.z + game_data.camera_acceleration_walking.z * 1.5) as f32 / 2.0
        );
        let can_walk = !block_type;

        // update the walked direction
        if can_walk {
            game_data.camera_position.x += game_data.camera_acceleration_walking.x;
            game_data.camera_position.z += game_data.camera_acceleration_walking.z;
        }
    }
    if game_data.camera_acceleration_walking.y > 0.0 {
        game_data.camera_position.y += game_data.camera_acceleration_walking.y;
        game_data.camera_acceleration_walking.y /= 1.2;
    }

    if !grounded {
        game_data.camera_position.y -= game_data.camera_acceleration.y;
        game_data.camera_acceleration.y += 0.01 * frame_time;
    } else {
        game_data.camera_acceleration.y = 0.0;
        game_data.jumping = false;

        // the distance into the block is the float distance from the full block
        let distance_in_block = ((game_data.camera_position.y as f32 / 2.0 - 2.0).floor() - 0.5) - (game_data.camera_position.y as f32 / 2.0 - 2.0) + 1.0;
        if distance_in_block > 0.0 {
            game_data.camera_position.y += distance_in_block;
        }
    }
}

fn get_block(chunk: &Vec<i8>, x: i64, y: i64, z: i64, game_data: &common::GameData, chunks: &HashMap<(i64, i64, i64), Vec<i8>>, blocks: &Vec<(String, Vec<i8>, String, String, bool, bool, bool)>, chunk_position_x: i64, chunk_position_y: i64, chunk_position_z: i64) -> bool {
    //let world_data_read = world_data.lock().unwrap();
    
    if x < 0 || y < 0 || z < 0 || x > 31 || y > 31 || z > 31 {
        if x < 0 && y >= 0 && z >= 0 && y < 32 && z < 32 {
            let chunk_position_x: i64 = chunk_position_x - 1;
            let chunk_position_y: i64 = chunk_position_y;
            let chunk_position_z: i64 = chunk_position_z;

            let maybe_chunk = chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block(chunk, 31, y, z, game_data, chunks, blocks, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        if y < 0 && x >= 0 && z >= 0 && x < 32 && z < 32 {
            let chunk_position_x: i64 = chunk_position_x;
            let chunk_position_y: i64 = chunk_position_y - 1;
            let chunk_position_z: i64 = chunk_position_z;

            let maybe_chunk = chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block(chunk, x, 31, z, game_data, chunks, blocks, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        if z < 0 && x >= 0 && y >= 0 && x < 32 && y < 32 {
            let chunk_position_x: i64 = chunk_position_x;
            let chunk_position_y: i64 = chunk_position_y;
            let chunk_position_z: i64 = chunk_position_z - 1;

            let maybe_chunk = chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block(chunk, x, y, 31, game_data, chunks, blocks, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        if x > 31 && y >= 0 && z >= 0 && y < 32 && z < 32 {
            let chunk_position_x: i64 = chunk_position_x + 1;
            let chunk_position_y: i64 = chunk_position_y;
            let chunk_position_z: i64 = chunk_position_z;

            let maybe_chunk = chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block(chunk, 0, y, z, game_data, chunks, blocks, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        if y > 31 && x >= 0 && z >= 0 && x < 32 && z < 32 {
            let chunk_position_x: i64 = chunk_position_x;
            let chunk_position_y: i64 = chunk_position_y + 1;
            let chunk_position_z: i64 = chunk_position_z;

            let maybe_chunk = chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block(chunk, x, 0, z, game_data, chunks, blocks, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        if z > 31 && x >= 0 && y >= 0 && x < 32 && y < 32 {
            let chunk_position_x: i64 = chunk_position_x;
            let chunk_position_y: i64 = chunk_position_y;
            let chunk_position_z: i64 = chunk_position_z + 1;

            let maybe_chunk = chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block(chunk, x, y, 0, game_data, chunks, blocks, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        return true;
    }
    let block_id = chunk[(x * 32 * 32 + y * 32 + z) as usize] as usize;
    if block_id <= 0 {
        return false;
    }

    let can_collide = blocks[block_id - 1].6;
    return can_collide;
}
fn get_block_global(game_data: &common::GameData, chunks: &HashMap<(i64, i64, i64), Vec<i8>>, blocks: &Vec<(String, Vec<i8>, String, String, bool, bool, bool)>, x: f32, y: f32, z: f32) -> bool {
    let chunk_position_x: i64 = ((x + 0.5) / 32.0).floor() as i64;
    let chunk_position_y: i64 = ((y + 0.5) / 32.0).floor() as i64;
    let chunk_position_z: i64 = ((z + 0.5) / 32.0).floor() as i64;

    let mut local_position_x = ((x + 0.5) % 32.0).floor() as i64;
    let mut local_position_y = ((y + 0.5) % 32.0).floor() as i64;
    let mut local_position_z = ((z + 0.5) % 32.0).floor() as i64;
    if local_position_x < 0 { local_position_x = 32 + local_position_x; }
    if local_position_y < 0 { local_position_y = 32 + local_position_y; }
    if local_position_z < 0 { local_position_z = 32 + local_position_z; }

    if let Some(chunk) = chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z)) {
        return get_block(chunk, local_position_x, local_position_y, local_position_z, game_data, chunks, blocks, chunk_position_x, chunk_position_y, chunk_position_z);
    }
    return true;
}