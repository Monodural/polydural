//#![windows_subsystem = "windows"]

mod common;
mod transforms;
mod world;
mod chunk;
mod interact;

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

    println!("loading model files");
    common::load_block_model_files(&mut game_data);

    // add first 4 chunks
    let chunk_data = chunk::generate_chunk(0, 0, 0, &mut game_data);
    let (chunk_vertices, chunk_normals, chunk_colors, chunk_uvs) = chunk::render_chunk(&chunk_data, &game_data);
    let vertex_data_chunk = create_vertices(chunk_vertices, chunk_normals, chunk_colors, chunk_uvs);
    game_data.set_chunk(0, 0, 0, chunk_data);
    game_data.add_object(vertex_data_chunk.clone(), (0, 0, 0), true);

    let chunk_data = chunk::generate_chunk(-1, 0, 0, &mut game_data);
    let (chunk_vertices, chunk_normals, chunk_colors, chunk_uvs) = chunk::render_chunk(&chunk_data, &game_data);
    let vertex_data_chunk = create_vertices(chunk_vertices, chunk_normals, chunk_colors, chunk_uvs);
    game_data.set_chunk(-1, 0, 0, chunk_data);
    game_data.add_object(vertex_data_chunk.clone(), (-1, 0, 0), true);

    let chunk_data = chunk::generate_chunk(0, 0, -1, &mut game_data);
    let (chunk_vertices, chunk_normals, chunk_colors, chunk_uvs) = chunk::render_chunk(&chunk_data, &game_data);
    let vertex_data_chunk = create_vertices(chunk_vertices, chunk_normals, chunk_colors, chunk_uvs);
    game_data.set_chunk(0, 0, -1, chunk_data);
    game_data.add_object(vertex_data_chunk.clone(), (0, 0, -1), true);

    let chunk_data = chunk::generate_chunk(-1, 0, -1, &mut game_data);
    let (chunk_vertices, chunk_normals, chunk_colors, chunk_uvs) = chunk::render_chunk(&chunk_data, &game_data);
    let vertex_data_chunk = create_vertices(chunk_vertices, chunk_normals, chunk_colors, chunk_uvs);
    game_data.set_chunk(-1, 0, -1, chunk_data);
    game_data.add_object(vertex_data_chunk.clone(), (-1, 0, -1), true);

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

    let light_data = common::light([1.0,1.0,1.0], [1.0, 1.0, 0.0], 0.05, 0.6, 0.3, 30.0);
    common::run(game_data, light_data, "Polydural");
}