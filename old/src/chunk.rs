use std::collections::HashMap;

use crate::{common::{self, RandomnessFunctions}, world};
use noise::NoiseFn;
use rand::Rng;

pub fn generate_chunk(chunk_position_x: i64, chunk_position_y: i64, chunk_position_z: i64, randomness_functions: &RandomnessFunctions, rng: &mut rand::prelude::ThreadRng, world_data: &world::WorldData) -> (Vec<i8>, Vec<i8>) {
    let mut chunk: Vec<i8> = Vec::new();
    let mut light: Vec<i8> = Vec::new();

    for _ in 0..32*32*32 {
        chunk.push(0);
        light.push(127);
    }

    let chunk_position_x = 32 * chunk_position_x;
    let chunk_position_y = 32 * chunk_position_y;
    let chunk_position_z = 32 * chunk_position_z;

    for x in 0..32 {
        let position_x = (x + chunk_position_x) as f32;
        for z in 0..32 {
            let position_z = (z + chunk_position_z) as f32;

            let temperature = randomness_functions.noise.get([position_x as f64 / 2000.0, position_z as f64 / 2000.0]) as f32 * 50.0 + 30.0;
            let moisture = randomness_functions.noise.get([position_x as f64 / 1000.0, position_z as f64 / 1000.0]) as f32 * 50.0 + 50.0;

            let mut closest = (10000.0, &"".to_string());
            let biomes = &world_data.biomes;

            for biome_ in biomes.keys() {
                let biome = &biomes[biome_];
                let distance_from_params = ((biome.0 as f32 - temperature).powf(2.0) + (biome.1 as f32 - moisture).powf(2.0)).sqrt();
                if distance_from_params < closest.0 {
                    closest.0 = distance_from_params;
                    closest.1 = biome_;
                }
            }

            //let river_noise = randomness_functions.noise.get([position_x as f64 / 1000.0, position_z as f64 / 1000.0]);
            //let river_noise_branching = randomness_functions.noise.get([position_x as f64 / 50.0, position_z as f64 / 50.0]);

            for y in 0..32 {
                let position_y = (y + chunk_position_y) as f32;

                let block_layers = &world_data.biomes[closest.1].3;
                let trees = &world_data.biomes[closest.1].5;

                let terrain_max_height: f32 = ((
                    (16.0 + randomness_functions.noise.get([position_x as f64 / 200.0, position_z as f64 / 200.0]) as f32 * 64.0) +
                    (16.0 + randomness_functions.noise.get([position_x as f64 / 100.0, position_z as f64 / 100.0]) as f32 * 32.0) +
                    (16.0 + randomness_functions.noise.get([position_x as f64 / 50.0, position_z as f64 / 50.0]) as f32 * 16.0) +
                    (16.0 + randomness_functions.noise.get([position_x as f64 / 25.0, position_z as f64 / 25.0]) as f32 * 8.0) +
                    (16.0 + randomness_functions.noise.get([position_x as f64 / 12.5, position_z as f64 / 12.5]) as f32 * 4.0)
                ) / 5.0).floor();

                let cave_noise = randomness_functions.noise.get([position_x as f64 / 25.0, position_y as f64 / 25.0, position_z as f64 / 25.0]);

                if cave_noise < 0.7 {
                    /*if position_y > terrain_max_height - (0.012 - river_noise.abs()) as f32 * 400.0 && river_noise.abs() < 0.012 {
                        continue;
                    } else if position_y > terrain_max_height - (0.006 - river_noise.abs() * river_noise_branching.abs()) as f32 * 600.0 && river_noise.abs() * river_noise_branching.abs() < 0.006 {
                        continue;
                    }*/
                    let mut found_layer = false;
                    for layer_ in 0..block_layers.len() {
                        let layer = &block_layers[block_layers.len() - layer_ - 1];
                        if found_layer { continue; }
                        if position_y <= terrain_max_height - layer.1 as f32 {
                            let mut block_index = 0;
                            if layer.0.len() > 1 {
                                block_index = rng.gen_range(0..layer.0.len());
                            }
                            chunk[(x * 32 * 32 + y * 32 + z) as usize] = world_data.block_index[&layer.0[block_index]] as i8;
                            found_layer = true;
                        }
                    }
                    if position_y == (terrain_max_height + 1.0).floor() && y > 0 && (
                        chunk[(x * 32 * 32 + (y - 1) * 32 + z) as usize] == world_data.block_index["grass_1"] as i8 || 
                        chunk[(x * 32 * 32 + (y - 1) * 32 + z) as usize] == world_data.block_index["grass_2"] as i8 || 
                        chunk[(x * 32 * 32 + (y - 1) * 32 + z) as usize] == world_data.block_index["dirt"] as i8 || 
                        chunk[(x * 32 * 32 + (y - 1) * 32 + z) as usize] == world_data.block_index["snow"] as i8 || 
                        chunk[(x * 32 * 32 + (y - 1) * 32 + z) as usize] == world_data.block_index["sand_1"] as i8 || 
                        chunk[(x * 32 * 32 + (y - 1) * 32 + z) as usize] == world_data.block_index["sand_2"] as i8
                    ) {
                        let tree_chosen = rng.gen_range(0..trees.len());
                        let tree = &trees[tree_chosen];
                        let folliage_number: f32 = rng.gen();
                        if folliage_number < tree.1 {
                            for block in &world_data.structures[&tree.0] {
                                let block_position_x = block.position[0] as i64 + x;
                                let block_position_y = block.position[1] as i64 + y;
                                let block_position_z = block.position[2] as i64 + z;
                                if block_position_x > 31 || block_position_x < 0 || block_position_y > 31 || block_position_y < 0 || block_position_z > 31 || block_position_z < 0 {
                                    continue;
                                }
                                chunk[(block_position_x * 32 * 32 + block_position_y * 32 + block_position_z) as usize] = world_data.block_index[&block.block] as i8;
                            }
                        }
                    }
                }
            }
        }
    }

    for x in 0..32 {
        for z in 0..32 {
            let mut light_level = 127;

            for y in 0..32 {
                let actual_y = 31 - y;

                if chunk[(x * 32 * 32 + actual_y * 32 + z) as usize] > 0 {
                    let is_transparent = &world_data.blocks[(chunk[(x * 32 * 32 + actual_y * 32 + z) as usize] - 1) as usize].5;
                    if *is_transparent && light_level >= 127 / 12 {
                        light_level -= 127 / 12;
                    } else {
                        //light_level = 0;
                    }
                }

                light[(x * 32 * 32 + actual_y * 32 + z) as usize] = light_level;
            }
        }
    }
    // do a second loop for horizontal
    /*for x in 0..32 {
        for z in 0..32 {
            for y in 0..32 {
                light[(x * 32 * 32 + actual_y * 32 + z) as usize] = light_level;
            }
        }
    }*/

    return (chunk, light);
}

pub fn _generate_light(_chunk: &Vec<i8>) -> Vec<i8> {
    let mut light: Vec<i8> = Vec::new();

    for _ in 0..32*32*32 {
        light.push(127);
    }

    return light;
}

pub fn set_block(chunk: Vec<i8>, x: i8, y: i8, z: i8, block_type: i8) -> Vec<i8> {
    if x > 31 || y > 31 || z > 31 { return chunk; }
    if x < 0 || y < 0 || z < 0 { return chunk; }
    let mut new_chunk = chunk;
    new_chunk[x as usize * 32 * 32 + y as usize * 32 + z as usize] = block_type;
    return new_chunk;
}

pub fn render_chunk(chunk: &Vec<i8>, light_data: &Vec<i8>, game_data: &common::GameData, chunks: &HashMap<(i64, i64, i64), Vec<i8>>, world_data: &world::WorldData, chunk_position_x: i64, chunk_position_y: i64, chunk_position_z: i64) -> (Vec<[f64; 3]>, Vec<[i8; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<[f64; 3]>, Vec<[i8; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>) {
    let mut vertices: Vec<[f64; 3]> = Vec::new();
    let mut normals: Vec<[i8; 3]> = Vec::new();
    let mut colors: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();

    let mut vertices_transparent: Vec<[f64; 3]> = Vec::new();
    let mut normals_transparent: Vec<[i8; 3]> = Vec::new();
    let mut colors_transparent: Vec<[f32; 3]> = Vec::new();
    let mut uvs_transparent: Vec<[f32; 2]> = Vec::new();

    let atlas_width = 8.0;
    let atlas_height = 8.0;

    let world_data_clone = &world_data;

    for x in 0..32 {
        for z in 0..32 {
            for y in 0..32 {
                let block_id = get_block(&chunk, x, y, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z);
                if block_id == 0 { continue; }

                let light_level = light_data[(x * 32 * 32 + y * 32 + z) as usize] as f32 / 127.0;

                let shape_index = world_data.shape_index[&world_data.blocks[(block_id - 1) as usize].3];
                let elements = &world_data.shapes[shape_index - 1].1;
                let is_transparent = &world_data.blocks[(block_id - 1) as usize].5;

                if !is_transparent {
                    for element in elements {
                        let vertices_from = element.from;
                        let vertices_to = element.to;

                        let mut directions = vec![
                            false, false, false, false, false, false,
                            false, false, false, false, false, false,
                            false, false, false, false, false, false
                        ];

                        if get_block_transparent(&chunk, x + 1, y, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[0] = true;
                        }

                        if get_block_transparent(&chunk, x - 1, y, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[1] = true;
                        }

                        if get_block_transparent(&chunk, x, y + 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[2] = true;
                        }

                        if get_block_transparent(&chunk, x, y - 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[3] = true;
                        }

                        if get_block_transparent(&chunk, x, y, z + 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[4] = true;
                        }

                        if get_block_transparent(&chunk, x, y, z - 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[5] = true;
                        }

                        if get_block(&chunk, x + 1, y + 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // right top (6)
                            directions[6] = true;
                        }
                        if get_block(&chunk, x + 1, y - 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // right bottom (7)
                            directions[7] = true;
                        }
                        if get_block(&chunk, x - 1, y + 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // left top (8)
                            directions[8] = true;
                        }
                        if get_block(&chunk, x - 1, y - 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // left bottom (9)
                            directions[9] = true;
                        }

                        if get_block(&chunk, x, y + 1, z + 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // front top (10)
                            directions[10] = true;
                        }
                        if get_block(&chunk, x, y - 1, z + 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // front bottom (11)
                            directions[11] = true;
                        }
                        if get_block(&chunk, x, y + 1, z - 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // back top (12)
                            directions[12] = true;
                        }
                        if get_block(&chunk, x, y - 1, z - 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // back bottom (13)
                            directions[13] = true;
                        }

                        if get_block(&chunk, x + 1, y, z + 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // right front (14)
                            directions[14] = true;
                        }
                        if get_block(&chunk, x + 1, y, z - 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // right back (15)
                            directions[15] = true;
                        }
                        if get_block(&chunk, x - 1, y , z + 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // left front (16)
                            directions[16] = true;
                        }
                        if get_block(&chunk, x - 1, y, z - 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // left back (17)
                            directions[17] = true;
                        }

                        let block_position_x = x as f64;
                        let block_position_y = y as f64;
                        let block_position_z = z as f64;

                        if !directions[0] {
                            vertices.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

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
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            if !directions[7] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                        }
                        if !directions[1] {
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

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
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            if !directions[9] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                        }
                        if !directions[2] {
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

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
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[6] && ! directions[10] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[8] && ! directions[12] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[6] && ! directions[10] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[6] && ! directions[12] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                        }
                        if !directions[3] {
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

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
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[7] && !directions[13] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[9] && !directions[11] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[7] && !directions[13] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[7] && !directions[11] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                        }
                        if !directions[4] {
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

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
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            if !directions[11] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                        }
                        if !directions[5] {
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

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
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            if !directions[13] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                        }
                    }
                } else {
                    for element in elements {
                        let vertices_from = element.from;
                        let vertices_to = element.to;

                        let mut directions = vec![
                            false, false, false, false, false, false,
                            false, false, false, false, false, false,
                            false, false, false, false, false, false
                        ];

                        if get_block_transparent(&chunk, x + 1, y, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[0] = true;
                        }

                        if get_block_transparent(&chunk, x - 1, y, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[1] = true;
                        }

                        if get_block_transparent(&chunk, x, y + 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[2] = true;
                        }

                        if get_block_transparent(&chunk, x, y - 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[3] = true;
                        }

                        if get_block_transparent(&chunk, x, y, z + 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[4] = true;
                        }

                        if get_block_transparent(&chunk, x, y, z - 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[5] = true;
                        }

                        if get_block(&chunk, x + 1, y + 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // right top (6)
                            directions[6] = true;
                        }
                        if get_block(&chunk, x + 1, y - 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // right bottom (7)
                            directions[7] = true;
                        }
                        if get_block(&chunk, x - 1, y + 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // left top (8)
                            directions[8] = true;
                        }
                        if get_block(&chunk, x - 1, y - 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // left bottom (9)
                            directions[9] = true;
                        }

                        if get_block(&chunk, x, y + 1, z + 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // front top (10)
                            directions[10] = true;
                        }
                        if get_block(&chunk, x, y - 1, z + 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // front bottom (11)
                            directions[11] = true;
                        }
                        if get_block(&chunk, x, y + 1, z - 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // back top (12)
                            directions[12] = true;
                        }
                        if get_block(&chunk, x, y - 1, z - 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // back bottom (13)
                            directions[13] = true;
                        }

                        if get_block(&chunk, x + 1, y, z + 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // right front (14)
                            directions[14] = true;
                        }
                        if get_block(&chunk, x + 1, y, z - 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // right back (15)
                            directions[15] = true;
                        }
                        if get_block(&chunk, x - 1, y , z + 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // left front (16)
                            directions[16] = true;
                        }
                        if get_block(&chunk, x - 1, y, z - 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // left back (17)
                            directions[17] = true;
                        }

                        let block_position_x = x as f64;
                        let block_position_y = y as f64;
                        let block_position_z = z as f64;

                        if !directions[0] && vertices_from[1] != vertices_to[1] && vertices_from[2] != vertices_to[2] {
                            vertices_transparent.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

                            let uv_x = (world_data.blocks[(block_id - 1) as usize].1[0] as f32 % atlas_width).floor();
                            let uv_y = (world_data.blocks[(block_id - 1) as usize].1[0] as f32 / atlas_height).floor();
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);

                            normals_transparent.push([1, 0, 0]);
                            normals_transparent.push([1, 0, 0]);
                            normals_transparent.push([1, 0, 0]);
                            normals_transparent.push([1, 0, 0]);
                            normals_transparent.push([1, 0, 0]);
                            normals_transparent.push([1, 0, 0]);

                            if !directions[7] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            if !directions[7] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                        }
                        if !directions[1] && vertices_from[1] != vertices_to[1] && vertices_from[2] != vertices_to[2] {
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

                            let uv_x = (world_data.blocks[(block_id - 1) as usize].1[1] as f32 % atlas_width).floor();
                            let uv_y = (world_data.blocks[(block_id - 1) as usize].1[1] as f32 / atlas_height).floor();
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);

                            normals_transparent.push([-1, 0, 0]);
                            normals_transparent.push([-1, 0, 0]);
                            normals_transparent.push([-1, 0, 0]);
                            normals_transparent.push([-1, 0, 0]);
                            normals_transparent.push([-1, 0, 0]);
                            normals_transparent.push([-1, 0, 0]);

                            if !directions[9] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            if !directions[9] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                        }
                        if !directions[2] && vertices_from[0] != vertices_to[0] && vertices_from[2] != vertices_to[2] {
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

                            let uv_x = (world_data.blocks[(block_id - 1) as usize].1[2] as f32 % atlas_width).floor();
                            let uv_y = (world_data.blocks[(block_id - 1) as usize].1[2] as f32 / atlas_height).floor();
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);

                            normals_transparent.push([0, 1, 0]);
                            normals_transparent.push([0, 1, 0]);
                            normals_transparent.push([0, 1, 0]);
                            normals_transparent.push([0, 1, 0]);
                            normals_transparent.push([0, 1, 0]);
                            normals_transparent.push([0, 1, 0]);

                            if !directions[8] && ! directions[10] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[6] && ! directions[10] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[8] && ! directions[12] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[6] && ! directions[10] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[6] && ! directions[12] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                        }
                        if !directions[3] && vertices_from[0] != vertices_to[0] && vertices_from[2] != vertices_to[2] {
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

                            let uv_x = (world_data.blocks[(block_id - 1) as usize].1[3] as f32 % atlas_width).floor();
                            let uv_y = (world_data.blocks[(block_id - 1) as usize].1[3] as f32 / atlas_height).floor();
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);

                            normals_transparent.push([0, -1, 0]);
                            normals_transparent.push([0, -1, 0]);
                            normals_transparent.push([0, -1, 0]);
                            normals_transparent.push([0, -1, 0]);
                            normals_transparent.push([0, -1, 0]);
                            normals_transparent.push([0, -1, 0]);

                            if !directions[9] && !directions[13] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[7] && !directions[13] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[9] && !directions[11] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[7] && !directions[13] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[7] && !directions[11] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                        }
                        if !directions[4] && vertices_from[0] != vertices_to[0] && vertices_from[1] != vertices_to[1] {
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

                            let uv_x = (world_data.blocks[(block_id - 1) as usize].1[4] as f32 % atlas_width).floor();
                            let uv_y = (world_data.blocks[(block_id - 1) as usize].1[4] as f32 / atlas_height).floor();
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);

                            normals_transparent.push([0, 0, 1]);
                            normals_transparent.push([0, 0, 1]);
                            normals_transparent.push([0, 0, 1]);
                            normals_transparent.push([0, 0, 1]);
                            normals_transparent.push([0, 0, 1]);
                            normals_transparent.push([0, 0, 1]);

                            if !directions[11] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            if !directions[11] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                        }
                        if !directions[5] && vertices_from[0] != vertices_to[0] && vertices_from[1] != vertices_to[1] {
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

                            let uv_x = (world_data.blocks[(block_id - 1) as usize].1[5] as f32 % atlas_width).floor();
                            let uv_y = (world_data.blocks[(block_id - 1) as usize].1[5] as f32 / atlas_height).floor();
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);

                            normals_transparent.push([0, 0, -1]);
                            normals_transparent.push([0, 0, -1]);
                            normals_transparent.push([0, 0, -1]);
                            normals_transparent.push([0, 0, -1]);
                            normals_transparent.push([0, 0, -1]);
                            normals_transparent.push([0, 0, -1]);

                            if !directions[13] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            if !directions[13] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                        }
                    }
                }
            }
        }
    }

    return (vertices, normals, colors, uvs, vertices_transparent, normals_transparent, colors_transparent, uvs_transparent);
}

pub fn get_block_transparent(chunk: &Vec<i8>, x: i64, y: i64, z: i64, game_data: &common::GameData, chunks: &HashMap<(i64, i64, i64), Vec<i8>>, world_data: &world::WorldData, chunk_position_x: i64, chunk_position_y: i64, chunk_position_z: i64) -> bool {
    if x < 0 || y < 0 || z < 0 || x > 31 || y > 31 || z > 31 {
        if x < 0 && y >= 0 && z >= 0 && y < 32 && z < 32 {
            let chunk_position_x: i64 = chunk_position_x - 1;
            let chunk_position_y: i64 = chunk_position_y;
            let chunk_position_z: i64 = chunk_position_z;

            let maybe_chunk = chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block_transparent(chunk, 31, y, z, game_data, chunks, &world_data, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        if y < 0 && x >= 0 && z >= 0 && x < 32 && z < 32 {
            let chunk_position_x: i64 = chunk_position_x;
            let chunk_position_y: i64 = chunk_position_y - 1;
            let chunk_position_z: i64 = chunk_position_z;

            let maybe_chunk = chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block_transparent(chunk, x, 31, z, game_data, chunks, &world_data, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        if z < 0 && x >= 0 && y >= 0 && x < 32 && y < 32 {
            let chunk_position_x: i64 = chunk_position_x;
            let chunk_position_y: i64 = chunk_position_y;
            let chunk_position_z: i64 = chunk_position_z - 1;

            let maybe_chunk = chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block_transparent(chunk, x, y, 31, game_data, chunks, &world_data, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        if x > 31 && y >= 0 && z >= 0 && y < 32 && z < 32 {
            let chunk_position_x: i64 = chunk_position_x + 1;
            let chunk_position_y: i64 = chunk_position_y;
            let chunk_position_z: i64 = chunk_position_z;

            let maybe_chunk = chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block_transparent(chunk, 0, y, z, game_data, chunks, &world_data, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        if y > 31 && x >= 0 && z >= 0 && x < 32 && z < 32 {
            let chunk_position_x: i64 = chunk_position_x;
            let chunk_position_y: i64 = chunk_position_y + 1;
            let chunk_position_z: i64 = chunk_position_z;

            let maybe_chunk = chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block_transparent(chunk, x, 0, z, game_data, chunks, &world_data, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        if z > 31 && x >= 0 && y >= 0 && x < 32 && y < 32 {
            let chunk_position_x: i64 = chunk_position_x;
            let chunk_position_y: i64 = chunk_position_y;
            let chunk_position_z: i64 = chunk_position_z + 1;

            let maybe_chunk = chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block_transparent(chunk, x, y, 0, game_data, chunks, &world_data, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        return false;
    }
    let block_found = chunk[(x * 32 * 32 + y * 32 + z) as usize];
    if block_found == 0 {
        return true;
    }
    let is_transparent = world_data.blocks[block_found as usize - 1].4;
    return is_transparent;
}
pub fn get_block(chunk: &Vec<i8>, x: i64, y: i64, z: i64, game_data: &common::GameData, chunks: &HashMap<(i64, i64, i64), Vec<i8>>, world_data: &world::WorldData, chunk_position_x: i64, chunk_position_y: i64, chunk_position_z: i64) -> i8 {
    if x < 0 || y < 0 || z < 0 || x > 31 || y > 31 || z > 31 {
        if x < 0 && y >= 0 && z >= 0 && y < 32 && z < 32 {
            let chunk_position_x: i64 = chunk_position_x - 1;
            let chunk_position_y: i64 = chunk_position_y;
            let chunk_position_z: i64 = chunk_position_z;

            let maybe_chunk = chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block(chunk, 31, y, z, game_data, chunks, &world_data, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        if y < 0 && x >= 0 && z >= 0 && x < 32 && z < 32 {
            let chunk_position_x: i64 = chunk_position_x;
            let chunk_position_y: i64 = chunk_position_y - 1;
            let chunk_position_z: i64 = chunk_position_z;

            let maybe_chunk = chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block(chunk, x, 31, z, game_data, chunks, &world_data, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        if z < 0 && x >= 0 && y >= 0 && x < 32 && y < 32 {
            let chunk_position_x: i64 = chunk_position_x;
            let chunk_position_y: i64 = chunk_position_y;
            let chunk_position_z: i64 = chunk_position_z - 1;

            let maybe_chunk = chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block(chunk, x, y, 31, game_data, chunks, &world_data, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        if x > 31 && y >= 0 && z >= 0 && y < 32 && z < 32 {
            let chunk_position_x: i64 = chunk_position_x + 1;
            let chunk_position_y: i64 = chunk_position_y;
            let chunk_position_z: i64 = chunk_position_z;

            let maybe_chunk = chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block(chunk, 0, y, z, game_data, chunks, &world_data, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        if y > 31 && x >= 0 && z >= 0 && x < 32 && z < 32 {
            let chunk_position_x: i64 = chunk_position_x;
            let chunk_position_y: i64 = chunk_position_y + 1;
            let chunk_position_z: i64 = chunk_position_z;

            let maybe_chunk = chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block(chunk, x, 0, z, game_data, chunks, &world_data, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        if z > 31 && x >= 0 && y >= 0 && x < 32 && y < 32 {
            let chunk_position_x: i64 = chunk_position_x;
            let chunk_position_y: i64 = chunk_position_y;
            let chunk_position_z: i64 = chunk_position_z + 1;

            let maybe_chunk = chunks.get(&(chunk_position_x, chunk_position_y, chunk_position_z));

            if let Some(chunk) = maybe_chunk {
                return get_block(chunk, x, y, 0, game_data, chunks, &world_data, chunk_position_x, chunk_position_y, chunk_position_z);
            }
        }
        return -1;
    }
    return chunk[(x * 32 * 32 + y * 32 + z) as usize];
}
pub fn get_block_global(game_data: &common::GameData, chunks: &HashMap<(i64, i64, i64), Vec<i8>>, world_data: &world::WorldData, x: f32, y: f32, z: f32) -> i8 {
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
        return get_block(chunk, local_position_x, local_position_y, local_position_z, game_data, chunks, &world_data, chunk_position_x, chunk_position_y, chunk_position_z);
    } else {
        return -1;
    }
}