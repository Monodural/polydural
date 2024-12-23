//#![windows_subsystem = "windows"]

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

fn vertex(p:[i8; 3], n: [i8; 3], c: [f32; 3], u: [f32; 2]) -> common::Vertex {
    return common::Vertex {
        position: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
        normal: [n[0] as f32, n[1] as f32, n[2] as f32, 1.0],
        color: [c[0] as f32, c[1] as f32, c[2] as f32, 1.0],
        uv: [u[0] as f32, u[1] as f32, 0.0, 0.0],
    }
}

fn create_vertices(vertices: Vec<[i8; 3]>, normals: Vec<[i8; 3]>, colors: Vec<[f32; 3]>, uvs: Vec<[f32; 2]>) -> Vec<common::Vertex> {
    let mut data:Vec<common::Vertex> = Vec::with_capacity(vertices.len());
    for i in 0..vertices.len() {
        data.push(vertex(vertices[i], normals[i], colors[i], uvs[i]));
    }
    return data.to_vec()
}

fn main(){
    let mut game_data = common::GameData::new();
    let world_data = Arc::new(Mutex::new(world::WorldData::new()));
    let randomness_functions = common::RandomnessFunctions::new();
    let inventory = containers::Inventory::new();

    //common::load_texture_files(&mut game_data);
    {
        let world_data_thread = Arc::clone(&world_data);
        println!("loading texture atlasses");
        common::load_texture_atlasses(&world_data_thread);
        println!("loaded texture atlasses");
        println!("loading biome files");
        common::load_biome_files(&world_data_thread);
        println!("loaded biome files");
        println!("loading structure files");
        common::load_structure_files(&world_data_thread);
        println!("loaded structure files");
        println!("loading model files");
        common::load_block_model_files(world_data_thread);
        println!("loaded model files");
    }

    // add a test string
    game_data.add_text_object((-0.46, -0.62, 0.0), (0.01, 0.02, 0.02), true, " 0".to_string());
    game_data.add_text_object((-0.347, -0.62, 0.0), (0.01, 0.02, 0.02), true, " 0".to_string());
    game_data.add_text_object((-0.235, -0.62, 0.0), (0.01, 0.02, 0.02), true, " 0".to_string());
    game_data.add_text_object((-0.122, -0.62, 0.0), (0.01, 0.02, 0.02), true, " 0".to_string());
    game_data.add_text_object((-0.01, -0.62, 0.0), (0.01, 0.02, 0.02), true, " 0".to_string());
    game_data.add_text_object((0.10, -0.62, 0.0), (0.01, 0.02, 0.02), true, " 0".to_string());
    game_data.add_text_object((0.215, -0.62, 0.0), (0.01, 0.02, 0.02), true, " 0".to_string());
    game_data.add_text_object((0.325, -0.62, 0.0), (0.01, 0.02, 0.02), true, " 0".to_string());
    game_data.add_text_object((0.44, -0.62, 0.0), (0.01, 0.02, 0.02), true, " 0".to_string());

    for slot in 0..inventory.hotbar_slots.len() {
        let slot_data = inventory.hotbar_slots[slot].clone();
        let mut slot_number = format!("{}", slot_data.1);
        if slot_number.len() == 1 {
            slot_number = format!(" {}", slot_number);
        }
        game_data.text[slot] = slot_number;

        let world_data_thread = Arc::clone(&world_data);
        let vertex_data_items = containers::Inventory::render_item_block(world_data_thread, slot_data.0);
        let vertex_data = create_vertices(
            vertex_data_items.0, vertex_data_items.2, vertex_data_items.3, vertex_data_items.1
        );
        game_data.add_gui_item_block(vertex_data.clone(), (0.11 * (slot as f32 - 4.0), -0.6, 0.0), (0.02, 0.02, 0.02), (1.0, 1.0, 1.0), true);
    }

    // add gui elements
    let vertex_data = create_vertices(
        vec![[-1, 1, 1], [1, 1, 1], [1, -1, 1], [-1, 1, 1], [1, -1, 1], [-1, -1, 1]], 
        vec![[0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1]], 
        vec![[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]], 
        vec![[0.0, 0.0], [0.046, 0.0], [0.046, 0.046], [0.0, 0.0], [0.046, 0.046], [0.0, 0.046]]
    );
    game_data.add_gui_object(vertex_data.clone(), (0.0, 0.0, 0.0), (0.02, 0.02, 0.02), true);
    let vertex_data = create_vertices(
        vec![[-1, 1, 0], [1, 1, 0], [1, -1, 0], [-1, 1, 0], [1, -1, 0], [-1, -1, 0]], 
        vec![[0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1]], 
        vec![[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]], 
        vec![[0.007, 0.13], [0.79, 0.13], [0.79, 0.23], [0.007, 0.13], [0.79, 0.23], [0.007, 0.23]]
    );
    game_data.add_gui_object(vertex_data.clone(), (0.0, -0.6, 0.0), (0.5, 0.06, 0.06), true);
    let vertex_data = create_vertices(
        vec![[-1, 1, 0], [1, 1, 0], [1, -1, 0], [-1, 1, 0], [1, -1, 0], [-1, -1, 0]], 
        vec![[0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1]], 
        vec![[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]], 
        vec![[0.007, 0.054], [0.07, 0.054], [0.07, 0.117], [0.007, 0.054], [0.07, 0.117], [0.007, 0.117]]
    );
    game_data.add_gui_object(vertex_data.clone(), (0.0, -0.6, 0.0), (0.04, 0.04, 0.04), true);

    let world_data_backend = Arc::clone(&world_data);
    let game_data_backend = game_data.clone();
    let randomness_functions_backend = randomness_functions.clone();

    thread::spawn(move || {
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        let update_interval = Duration::from_millis(5);
        loop {
            let start_time = Instant::now();
            {   
                let mut world_data_read: world::WorldData;
                {
                    let world_data = world_data_backend.lock().unwrap();
                    world_data_read = world_data.clone();
                }

                if world_data_read.chunk_queue.len() == 0 && world_data_read.chunk_update_queue.len() > 0 {
                    let chunk_position = world_data_read.chunk_buffer_coordinates[world_data_read.chunk_update_queue[0]];
                    let chunk_data = world_data_read.chunks[&(chunk_position.0, chunk_position.1, chunk_position.2)].clone();
                    let (chunk_vertices, chunk_normals, chunk_colors, chunk_uvs) = chunk::render_chunk(&chunk_data, &game_data_backend, &mut world_data_read, 
                        chunk_position.0, chunk_position.1, chunk_position.2
                    );
                    let vertex_data_chunk = create_vertices(chunk_vertices, chunk_normals, chunk_colors, chunk_uvs);
                    let mut buffer_index: usize = 0;
                    if let Some(chunk_index) = world_data_read.chunk_buffer_index.get(&(chunk_position.0, chunk_position.1, chunk_position.2)) {
                        buffer_index = *chunk_index as usize;
                    }
                    {
                        let mut world_data_write = world_data_backend.lock().unwrap();
                        world_data_write.updated_chunk_data.push((buffer_index, vertex_data_chunk));
                        world_data_write.chunk_update_queue.remove(0);
                    }
                }
                if let Some(chunk_coordinates) = world_data_read.chunk_queue.iter().next() {
                    let chunk_position_x_with_offset = chunk_coordinates.0;
                    let chunk_position_y_with_offset = chunk_coordinates.1;
                    let chunk_position_z_with_offset = chunk_coordinates.2;
                    let chunk_data = chunk::generate_chunk(
                        chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset, game_data_backend.clone(), &randomness_functions_backend, &mut rng, &mut world_data_backend.lock().unwrap()
                    );
                    let (chunk_vertices, chunk_normals, chunk_colors, chunk_uvs) = chunk::render_chunk(&chunk_data, &game_data_backend, &mut world_data_backend.lock().unwrap(), 
                        chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset
                    );
                    let vertex_data_chunk = create_vertices(chunk_vertices, chunk_normals, chunk_colors, chunk_uvs);
                    let model_mat = transforms::create_transforms([
                        chunk_position_x_with_offset as f32 * 32.0, 
                        chunk_position_y_with_offset as f32 * 32.0, 
                        chunk_position_z_with_offset as f32 * 32.0], 
                        [0.0, 0.0, 0.0], [1.0, 1.0, 1.0]
                    );
                    {
                        let mut world_data_write = world_data_backend.lock().unwrap();
                        let normal_mat = (model_mat.invert().unwrap()).transpose();
                        world_data_write.set_chunk(chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset, chunk_data);
                        world_data_write.chunk_queue.remove(&(chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset));
                        world_data_write.created_chunk_data.push((vertex_data_chunk, chunk_position_x_with_offset, chunk_position_y_with_offset, chunk_position_z_with_offset, model_mat, normal_mat));
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

    let light_data = common::light([1.0, 1.0, 1.0], [1.0, 1.0, 0.0], 0.1, 0.6, 0.3, 30.0);
    common::run(game_data, world_data, inventory, light_data, "Polydural");
}