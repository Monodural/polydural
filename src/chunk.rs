use crate::{common::{self, RandomnessFunctions}, world};
use noise::NoiseFn;
use rand::Rng;

pub fn generate_chunk(chunk_position_x: i64, chunk_position_y: i64, chunk_position_z: i64, _game_data: &mut common::GameData, randomness_functions: &mut RandomnessFunctions, world_data: &mut world::WorldData) -> Vec<i8> {
    let mut chunk: Vec<i8> = Vec::new();

    for _ in 0..16*16*16 {
        chunk.push(0);
    }

    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                let position_x = (x + 16 * chunk_position_x) as f32;
                let position_y = (y + 16 * chunk_position_y) as f32;
                let position_z = (z + 16 * chunk_position_z) as f32;

                let terrain_max_height: f32 = ((
                    (16.0 + randomness_functions.noise.get([position_x as f64 / 50.0, position_z as f64 / 50.0]) as f32 * 16.0) +
                    (16.0 + randomness_functions.noise.get([position_x as f64 / 25.0, position_z as f64 / 25.0]) as f32 * 8.0) +
                    (16.0 + randomness_functions.noise.get([position_x as f64 / 12.5, position_z as f64 / 12.5]) as f32 * 4.0)
                ) / 3.0).floor();

                if (position_x.powf(2.0) + (position_y - 16.0).powf(2.0) + position_z.powf(2.0)).sqrt() > 10.0 {
                    if position_y > terrain_max_height - 4.0 && position_y < terrain_max_height {
                        chunk[(x * 16 * 16 + y * 16 + z) as usize] = world_data.block_index["dirt"] as i8;
                    } else if position_y < terrain_max_height {
                        chunk[(x * 16 * 16 + y * 16 + z) as usize] = world_data.block_index["stone"] as i8;
                    } else if position_y == terrain_max_height {
                        let folliage_number: bool = randomness_functions.rng.gen();
                        if folliage_number == true {
                            chunk[(x * 16 * 16 + y * 16 + z) as usize] = world_data.block_index["grass_1"] as i8;
                        } else {
                            chunk[(x * 16 * 16 + y * 16 + z) as usize] = world_data.block_index["grass_2"] as i8;
                        }
                    } else if position_y == (terrain_max_height + 1.0).floor() {
                        let folliage_number: f32 = randomness_functions.rng.gen();
                        if folliage_number < 0.01 {
                            for i in 0..5 {
                                if y + i > 15 { continue; }
                                chunk[(x * 16 * 16 + (y + i) * 16 + z) as usize] = world_data.block_index["oak_log"] as i8;
                            }
                        }
                    }
                }
            }
        }
    }

    return chunk;
}

pub fn set_block(chunk: Vec<i8>, x: i8, y: i8, z: i8, block_type: i8) -> Vec<i8> {
    if x > 15 || y > 15 || z > 15 { return chunk; }
    if x < 0 || y < 0 || z < 0 { return chunk; }
    let mut new_chunk = chunk;
    new_chunk[x as usize * 16 * 16 + y as usize * 16 + z as usize] = block_type;
    return new_chunk;
}

pub fn render_chunk(chunk: &Vec<i8>, game_data: &common::GameData, world_data: &mut world::WorldData, chunk_position_x: i64, chunk_position_y: i64, chunk_position_z: i64) -> (Vec<[i8; 3]>, Vec<[i8; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>) {
    let mut vertices: Vec<[i8; 3]> = Vec::new();
    let mut normals: Vec<[i8; 3]> = Vec::new();
    let mut colors: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();

    let atlas_width = 8.0;
    let atlas_height = 8.0;

    let world_data_clone = &world_data;

    for x in 0..16 {
        for y in 0..16 {
            for z in 0..16 {
                let block_id = get_block(&chunk, x, y, z, &game_data, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z);
                if block_id == 0 { continue; }

                let mut directions = vec![
                    false, false, false, false, false, false,
                    false, false, false, false, false, false,
                    false, false, false, false, false, false
                ];

                if get_block(&chunk, x + 1, y, z, &game_data, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) != 0 {
                    directions[0] = true;
                }

                if get_block(&chunk, x - 1, y, z, &game_data, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) != 0 {
                    directions[1] = true;
                }

                if get_block(&chunk, x, y + 1, z, &game_data, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) != 0 {
                    directions[2] = true;
                }

                if get_block(&chunk, x, y - 1, z, &game_data, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) != 0 {
                    directions[3] = true;
                }

                if get_block(&chunk, x, y, z + 1, &game_data, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) != 0 {
                    directions[4] = true;
                }

                if get_block(&chunk, x, y, z - 1, &game_data, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) != 0 {
                    directions[5] = true;
                }

                if get_block(&chunk, x + 1, y + 1, z, &game_data, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // right top (6)
                    directions[6] = true;
                }
                if get_block(&chunk, x + 1, y - 1, z, &game_data, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // right bottom (7)
                    directions[7] = true;
                }
                if get_block(&chunk, x - 1, y + 1, z, &game_data, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // left top (8)
                    directions[8] = true;
                }
                if get_block(&chunk, x - 1, y - 1, z, &game_data, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // left bottom (9)
                    directions[9] = true;
                }

                if get_block(&chunk, x, y + 1, z + 1, &game_data, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // front top (10)
                    directions[10] = true;
                }
                if get_block(&chunk, x, y - 1, z + 1, &game_data, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // front bottom (11)
                    directions[11] = true;
                }
                if get_block(&chunk, x, y + 1, z - 1, &game_data, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // back top (12)
                    directions[12] = true;
                }
                if get_block(&chunk, x, y - 1, z - 1, &game_data, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // back bottom (13)
                    directions[13] = true;
                }

                if get_block(&chunk, x + 1, y, z + 1, &game_data, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // right front (14)
                    directions[14] = true;
                }
                if get_block(&chunk, x + 1, y, z - 1, &game_data, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // right back (15)
                    directions[15] = true;
                }
                if get_block(&chunk, x - 1, y , z + 1, &game_data, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // left front (16)
                    directions[16] = true;
                }
                if get_block(&chunk, x - 1, y, z - 1, &game_data, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // left back (17)
                    directions[17] = true;
                }

                let block_position_x = x as i8;
                let block_position_y = y as i8;
                let block_position_z = z as i8;

                if !directions[0] {
                    vertices.push([ 1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);

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

                    if !directions[7] {
                        colors.push([1.0, 1.0, 1.0]);
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    if !directions[7] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    colors.push([1.0, 1.0, 1.0]);
                }
                if !directions[1] {
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);

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

                    if !directions[9] {
                        colors.push([1.0, 1.0, 1.0]);
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    if !directions[9] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    colors.push([1.0, 1.0, 1.0]);
                }
                if !directions[2] {
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);

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

                    if !directions[8] && ! directions[10] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    if !directions[6] && ! directions[10] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    if !directions[8] && ! directions[12] {
                        colors.push([1.0, 1.0, 1.0]);
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    if !directions[6] && ! directions[10] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    if !directions[6] && ! directions[12] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
                }
                if !directions[3] {
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);

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

                    if !directions[9] && !directions[13] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    if !directions[7] && !directions[13] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    if !directions[9] && !directions[11] {
                        colors.push([1.0, 1.0, 1.0]);
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    if !directions[7] && !directions[13] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    if !directions[7] && !directions[11] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
                }
                if !directions[4] {
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);

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

                    if !directions[11] {
                        colors.push([1.0, 1.0, 1.0]);
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    if !directions[11] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    colors.push([1.0, 1.0, 1.0]);
                }
                if !directions[5] {
                    vertices.push([ 1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);

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

                    if !directions[13] {
                        colors.push([1.0, 1.0, 1.0]);
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    if !directions[13] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    colors.push([1.0, 1.0, 1.0]);
                }
            }
        }
    }

    return (vertices, normals, colors, uvs);
}

pub fn get_block(chunk: &Vec<i8>, x: i64, y: i64, z: i64, game_data: &common::GameData, world_data: &world::WorldData, chunk_position_x: i64, chunk_position_y: i64, chunk_position_z: i64) -> i8 {
    if x < 0 || y < 0 || z < 0 || x > 15 || y > 15 || z > 15 {
        if x < 0 && y >= 0 && z >= 0 && y < 16 && z < 16 {
            let chunk_position_x: i64 = chunk_position_x - 1;
            let chunk_position_y: i64 = chunk_position_y;
            let chunk_position_z: i64 = chunk_position_z;

            let maybe_chunk = world_data.chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block(chunk, 15, y, z, game_data, &world_data, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        if y < 0 && x >= 0 && z >= 0 && x < 16 && z < 16 {
            let chunk_position_x: i64 = chunk_position_x;
            let chunk_position_y: i64 = chunk_position_y - 1;
            let chunk_position_z: i64 = chunk_position_z;

            let maybe_chunk = world_data.chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block(chunk, x, 15, z, game_data, &world_data, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        if z < 0 && x >= 0 && y >= 0 && x < 16 && y < 16 {
            let chunk_position_x: i64 = chunk_position_x;
            let chunk_position_y: i64 = chunk_position_y;
            let chunk_position_z: i64 = chunk_position_z - 1;

            let maybe_chunk = world_data.chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block(chunk, x, y, 15, game_data, &world_data, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        if x > 15 && y >= 0 && z >= 0 && y < 16 && z < 16 {
            let chunk_position_x: i64 = chunk_position_x + 1;
            let chunk_position_y: i64 = chunk_position_y;
            let chunk_position_z: i64 = chunk_position_z;

            let maybe_chunk = world_data.chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block(chunk, 0, y, z, game_data, &world_data, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        if y > 15 && x >= 0 && z >= 0 && x < 16 && z < 16 {
            let chunk_position_x: i64 = chunk_position_x;
            let chunk_position_y: i64 = chunk_position_y + 1;
            let chunk_position_z: i64 = chunk_position_z;

            let maybe_chunk = world_data.chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block(chunk, x, 0, z, game_data, &world_data, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        if z > 15 && x >= 0 && y >= 0 && x < 16 && y < 16 {
            let chunk_position_x: i64 = chunk_position_x;
            let chunk_position_y: i64 = chunk_position_y;
            let chunk_position_z: i64 = chunk_position_z + 1;

            let maybe_chunk = world_data.chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block(chunk, x, y, 0, game_data, &world_data, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        return -1;
    }
    return chunk[(x * 16 * 16 + y * 16 + z) as usize];
}
pub fn get_block_global(game_data: &common::GameData, world_data: world::WorldData, x: f32, y: f32, z: f32) -> i8 {
    let chunk_position_x: i64 = ((x + 0.5) / 16.0).floor() as i64;
    let chunk_position_y: i64 = ((y + 0.5) / 16.0).floor() as i64;
    let chunk_position_z: i64 = ((z + 0.5) / 16.0).floor() as i64;

    let mut local_position_x = ((x + 0.5) % 16.0).floor() as i64;
    let mut local_position_y = ((y + 0.5) % 16.0).floor() as i64;
    let mut local_position_z = ((z + 0.5) % 16.0).floor() as i64;
    if local_position_x < 0 { local_position_x = 16 + local_position_x; }
    if local_position_y < 0 { local_position_y = 16 + local_position_y; }
    if local_position_z < 0 { local_position_z = 16 + local_position_z; }

    if let Some(chunk) = world_data.chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z)) {
        return get_block(chunk, local_position_x, local_position_y, local_position_z, game_data, &world_data, chunk_position_x, chunk_position_y, chunk_position_z);
    } else {
        return -1;
    }
}