/*use crate::common::{self, RandomnessFunctions};
use noise::NoiseFn;
use rand::Rng;

use crate::config::{CHUNK_SIZE_X, CHUNK_SIZE_Y, CHUNK_SIZE_Z};

pub fn generate_chunk(chunk_position_x: i64, chunk_position_y: i64, chunk_position_z: i64, randomness_functions: &RandomnessFunctions, rng: &mut rand::prelude::ThreadRng, world_data: &world::WorldData) -> (Vec<i8>, Vec<i8>) {
    let mut chunk = [CHUNK_SIZE_X * CHUNK_SIZE_Y * CHUNK_SIZE_Z; 0];
    let mut light = [CHUNK_SIZE_X * CHUNK_SIZE_Y * CHUNK_SIZE_Z; 0];

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

    return (chunk, light);
}*/