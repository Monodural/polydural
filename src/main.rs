//#![windows_subsystem = "windows"]

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

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
    //let mut game_data = common::GameData::new();
    let mut game_data = common::GameData::new();
    let world_data = Arc::new(Mutex::new(world::WorldData::new()));
    let randomness_functions = common::RandomnessFunctions::new();
    let mut _inventory = containers::Inventory::new();

    println!("loading model files");
    //common::load_texture_files(&mut game_data);
    let world_data_thread = Arc::clone(&world_data);
    common::load_block_model_files(world_data_thread);
    println!("loaded model files");

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

    thread::spawn(move || {
        let update_interval = Duration::from_millis(20);
        loop {
            let start_time = Instant::now();
            {
                let mut world_data = world_data_backend.lock().unwrap();
                //let random_value: f32 = data_backend.rng.gen(); // Example usage
                if world_data.chunk_queue.len() == 0 && world_data.chunk_update_queue.len() > 0 {
                    println!("updating chunk");
                    let chunk_position = world_data.chunk_buffer_coordinates[world_data.chunk_update_queue[0]];
                    let chunk_data = world_data.chunks[&(chunk_position.0, chunk_position.1, chunk_position.2)].clone();
                    let (chunk_vertices, chunk_normals, chunk_colors, chunk_uvs) = chunk::render_chunk(&chunk_data, &game_data_backend, &mut world_data, 
                        chunk_position.0, chunk_position.1, chunk_position.2
                    );
                    let vertex_data_chunk = create_vertices(chunk_vertices, chunk_normals, chunk_colors, chunk_uvs);
                    let mut buffer_index: usize = 0;
                    if let Some(chunk_index) = world_data.chunk_buffer_index.get(&(chunk_position.0, chunk_position.1, chunk_position.2)) {
                        buffer_index = *chunk_index as usize;
                    }
                    /*let (uniform_bind_group, vertex_uniform_buffer, vertex_buffer, num_vertices_) = common::create_object_from_chunk(&vertex_data_chunk, &self.init, self.light_data, &self.uniform_bind_group_layout);
                    self.vertex_buffers[buffer_index as usize] = vertex_buffer;
                    self.num_vertices[buffer_index as usize] = num_vertices_;
                    self.uniform_bind_groups[buffer_index as usize] = uniform_bind_group;
                    self.vertex_uniform_buffers[buffer_index as usize] = vertex_uniform_buffer;
                    world_data.updated_chunks.push(buffer_index as usize);*/
                    world_data.chunk_update_queue.remove(0);
                }
            }
            let elapsed = start_time.elapsed();
            if elapsed < update_interval {
                thread::sleep(update_interval - elapsed);
            }
        }
    });

    let light_data = common::light([1.0,1.0,1.0], [1.0, 1.0, 0.0], 0.05, 0.6, 0.3, 30.0);
    common::run(game_data, randomness_functions, world_data, light_data, "Polydural");
}