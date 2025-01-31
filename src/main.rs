#![windows_subsystem = "windows"]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use cgmath::*;

mod common;
mod transforms;
mod world;
mod chunk;
mod interact;
mod containers;
mod gui;
mod sounds;

fn vertex(p:[f64; 3], n: [i8; 3], c: [f32; 3], u: [f32; 2]) -> common::Vertex {
    return common::Vertex {
        position: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
        normal: [n[0] as f32, n[1] as f32, n[2] as f32, 1.0],
        color: [c[0] as f32, c[1] as f32, c[2] as f32, 1.0],
        uv: [u[0] as f32, u[1] as f32, 0.0, 0.0],
    }
}

fn create_vertices(vertices: Vec<[f64; 3]>, normals: Vec<[i8; 3]>, colors: Vec<[f32; 3]>, uvs: Vec<[f32; 2]>) -> Vec<common::Vertex> {
    let mut data:Vec<common::Vertex> = Vec::with_capacity(vertices.len());
    for i in 0..vertices.len() {
        data.push(vertex(vertices[i], normals[i], colors[i], uvs[i]));
    }
    return data.to_vec()
}

fn main(){
    let mut modding_allowed = true;
    if cfg!(target_os = "android") || cfg!(target_os = "ios") {
        modding_allowed = false;
    }
    let mut game_data = common::GameData::new();
    let chunk_data_terrain: Arc<Mutex<HashMap<(i64, i64, i64), Vec<i8>>>> = Arc::new(Mutex::new(HashMap::new()));
    let chunk_data_lighting: Arc<Mutex<HashMap<(i64, i64, i64), Vec<i8>>>> = Arc::new(Mutex::new(HashMap::new()));

    let world_data = Arc::new(Mutex::new(world::WorldData::new(Arc::clone(&chunk_data_terrain), Arc::clone(&chunk_data_lighting))));
    let randomness_functions = common::RandomnessFunctions::new();
    let inventory = containers::Inventory::new();

    //common::load_texture_files(&mut game_data);
    {
        let world_data_thread = Arc::clone(&world_data);
        println!("loading texture atlasses");
        common::load_texture_atlasses(&world_data_thread);
        println!("loaded texture atlasses");
        println!("loading biome files");
        common::load_biome_files(&world_data_thread, modding_allowed);
        println!("loaded biome files");
        println!("loading structure files");
        common::load_structure_files(&world_data_thread, modding_allowed);
        println!("loaded structure files");
        println!("loading shape files");
        common::load_shape_files(&world_data_thread, modding_allowed);
        println!("loaded shape files");
        println!("loading model files");
        common::load_block_model_files(&world_data_thread, modding_allowed);
        println!("loaded model files");
        println!("loading audio files");
        common::load_audio_files(&world_data_thread, modding_allowed);
        println!("loaded audio files");
    }

    // add text elements
    game_data.add_text_object((-0.44 * 2.0 - 0.04, -0.62 * 2.0 + 0.025, 0.0), (0.02, 0.04, 0.04), true, " 0".to_string());
    game_data.add_text_object((-0.33 * 2.0 - 0.04, -0.62 * 2.0 + 0.025, 0.0), (0.02, 0.04, 0.04), true, " 0".to_string());
    game_data.add_text_object((-0.22 * 2.0 - 0.04, -0.62 * 2.0 + 0.025, 0.0), (0.02, 0.04, 0.04), true, " 0".to_string());
    game_data.add_text_object((-0.11 * 2.0 - 0.04, -0.62 * 2.0 + 0.025, 0.0), (0.02, 0.04, 0.04), true, " 0".to_string());
    game_data.add_text_object((0.0 * 2.0 - 0.04, -0.62 * 2.0 + 0.025, 0.0), (0.02, 0.04, 0.04), true, " 0".to_string());
    game_data.add_text_object((0.11 * 2.0 - 0.04, -0.62 * 2.0 + 0.025, 0.0), (0.02, 0.04, 0.04), true, " 0".to_string());
    game_data.add_text_object((0.22 * 2.0 - 0.04, -0.62 * 2.0 + 0.025, 0.0), (0.02, 0.04, 0.04), true, " 0".to_string());
    game_data.add_text_object((0.33 * 2.0 - 0.04, -0.62 * 2.0 + 0.025, 0.0), (0.02, 0.04, 0.04), true, " 0".to_string());
    game_data.add_text_object((0.44 * 2.0 - 0.04, -0.62 * 2.0 + 0.025, 0.0), (0.02, 0.04, 0.04), true, " 0".to_string());

    // add items to hotbar
    for slot in 0..inventory.hotbar_slots.len() {
        let slot_data = inventory.hotbar_slots[slot].clone();
        let mut slot_number = format!("{}", slot_data.1);
        if slot_number == "1" {
            slot_number = format!("");
        }
        if slot_number.len() == 1 {
            slot_number = format!(" {}", slot_number);
        }
        game_data.text[slot] = slot_number;

        let world_data_thread = Arc::clone(&world_data);
        let vertex_data_items = containers::Inventory::render_item_block(
            world_data_thread, slot_data.0, (0.11 * (slot as f64 - 4.0), -0.6, 0.0), (0.02, 0.02, 0.02)
        );
        let vertex_data = create_vertices(
            vertex_data_items.0, vertex_data_items.2, vertex_data_items.3, vertex_data_items.1
        );
        game_data.add_gui_item_block(vertex_data.clone(), (0.11 * (slot as f32 - 4.0), -0.6, 0.0), (0.02, 0.02, 0.02), (1.0, 1.0, 1.0), true);
    }

    // add gui elements
    gui::create_frame(&mut game_data, (0.0, 0.0, 0.0), (0.02, 0.02, 0.02), [0.5, 0.5, 0.5], vec![[0.0, 0.0], [0.046, 0.0], [0.046, 0.046], [0.0, 0.0], [0.046, 0.046], [0.0, 0.046]]);
    gui::create_frame(&mut game_data, (0.0, -0.6, 0.0), (0.5, 0.06, 0.06), [0.5, 0.5, 0.5], vec![[0.007, 0.13], [0.79, 0.13], [0.79, 0.23], [0.007, 0.13], [0.79, 0.23], [0.007, 0.23]]);
    gui::create_frame(&mut game_data, (0.44, -0.6, 0.0), (0.04, 0.04, 0.04), [0.5, 0.5, 0.5], vec![[0.007, 0.054], [0.07, 0.054], [0.07, 0.117], [0.007, 0.054], [0.07, 0.117], [0.007, 0.117]]);

    let world_data_backend = Arc::clone(&world_data);
    let chunk_data_terrain_backend = Arc::clone(&chunk_data_terrain);
    let chunk_data_lighting_backend = Arc::clone(&chunk_data_lighting);
    let game_data_backend = game_data.clone();
    let randomness_functions_backend = randomness_functions.clone();

    let world_data_audio = world_data_backend.lock().unwrap().clone();
    let audio_tx = sounds::start_audio_thread(world_data_audio.audio_files.clone());
    let music_tx = sounds::start_music_thread(world_data_audio.audio_files);

    thread::spawn(move || {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        let update_interval = Duration::from_millis(1);
        loop {
            let start_time = Instant::now();
            {
                let world_data_read = world_data_backend.lock().unwrap().clone();

                for item in world_data_read.sound_queue.clone().into_iter() {
                    audio_tx.send(item).expect("Failed to send sound index");
                }
                for item in world_data_read.music_queue.clone().into_iter() {
                    music_tx.send(item).expect("Failed to send sound index");
                }
                {
                    let mut world_audio_read = world_data_backend.lock().unwrap();
                    world_audio_read.sound_queue.clear();
                    world_audio_read.music_queue.clear();
                }

                let chunk_data_terrain = chunk_data_terrain_backend.lock().unwrap().clone();

                if world_data_read.chunk_queue.len() == 0 && world_data_read.chunk_update_queue.len() > 0 {
                    let chunk_data_lighting = chunk_data_lighting_backend.lock().unwrap().clone();

                    let chunk_position = world_data_read.chunk_buffer_coordinates[world_data_read.chunk_update_queue[0]];
                    let chunk_data = chunk_data_terrain[&(chunk_position.0, chunk_position.1, chunk_position.2)].clone();
                    let chunk_data_light = chunk_data_lighting[&(chunk_position.0, chunk_position.1, chunk_position.2)].clone();
                    let (chunk_vertices, chunk_normals, chunk_colors, chunk_uvs,
                        chunk_vertices_transparent, chunk_normals_transparent, chunk_colors_transparent, chunk_uvs_transparent
                        ) = chunk::render_chunk(&chunk_data, &chunk_data_light, &game_data_backend, &chunk_data_terrain, &world_data_read, 
                        chunk_position.0, chunk_position.1, chunk_position.2
                    );
                    let vertex_data_chunk = create_vertices(chunk_vertices, chunk_normals, chunk_colors, chunk_uvs);
                    let vertex_data_chunk_transparent = create_vertices(chunk_vertices_transparent, chunk_normals_transparent, chunk_colors_transparent, chunk_uvs_transparent);
                    let mut buffer_index: usize = 0;
                    if let Some(chunk_index) = world_data_read.chunk_buffer_index.get(&(chunk_position.0, chunk_position.1, chunk_position.2)) {
                        buffer_index = *chunk_index as usize;
                    }
                    {
                        let mut world_data_write = world_data_backend.lock().unwrap();
                        world_data_write.updated_chunk_data.push((buffer_index, vertex_data_chunk));
                        world_data_write.updated_chunk_data_transparent.push((buffer_index, vertex_data_chunk_transparent));
                        world_data_write.chunk_update_queue.remove(0);
                    }
                }
                if let Some(chunk_coordinates) = world_data_read.chunk_queue.iter().next() {
                    let chunk_position_x_with_offset = chunk_coordinates.0;
                    let chunk_position_y_with_offset = chunk_coordinates.1;
                    let chunk_position_z_with_offset = chunk_coordinates.2;
                    let world_data_cloned = world_data_backend.lock().unwrap().clone();
                    let (chunk_data, light_map) = chunk::generate_chunk(
                        chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset, &randomness_functions_backend, &mut rng, &world_data_cloned
                    );
                    let (chunk_vertices, chunk_normals, chunk_colors, chunk_uvs,
                        chunk_vertices_transparent, chunk_normals_transparent, chunk_colors_transparent, chunk_uvs_transparent
                        ) = chunk::render_chunk(&chunk_data, &light_map, &game_data_backend, &chunk_data_terrain, &world_data_cloned, 
                        chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset
                    );
                    let vertex_data_chunk = create_vertices(chunk_vertices, chunk_normals, chunk_colors, chunk_uvs);
                    let vertex_data_chunk_transparent = create_vertices(chunk_vertices_transparent, chunk_normals_transparent, chunk_colors_transparent, chunk_uvs_transparent);
                    let model_mat = transforms::create_transforms([
                        chunk_position_x_with_offset as f32 * 32.0, 
                        chunk_position_y_with_offset as f32 * 32.0, 
                        chunk_position_z_with_offset as f32 * 32.0], 
                        [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]
                    );
                    {
                        let normal_mat = (model_mat.invert().unwrap()).transpose();
                        let mut world_data_write = world_data_backend.lock().unwrap();
                        world_data_write.set_chunk(chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset, chunk_data, light_map);
                        world_data_write.chunk_queue.remove(&(chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset));
                        world_data_write.created_chunk_data.push((vertex_data_chunk, chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset, model_mat, normal_mat));
                        world_data_write.created_chunk_data_transparent.push((vertex_data_chunk_transparent, chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset, model_mat, normal_mat));
                        world_data_write.created_chunk_queue.insert((chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset));
                    }
                }
            }
            let elapsed = start_time.elapsed();
            if elapsed < update_interval {
                thread::sleep(update_interval - elapsed);
            }
        }
    });

    let light_data = common::light([1.0, 1.0, 1.0], [1.0, 1.0, 0.0], 0.1, 0.8, 0.3, 30.0);
    common::run(game_data, world_data, inventory, light_data, "Polydural", chunk_data_terrain, chunk_data_lighting);
}