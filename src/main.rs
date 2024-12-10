#![windows_subsystem = "windows"]

mod common;
mod vertex_data;
mod transforms;
mod world;
mod chunk;

fn vertex(p:[i8; 3], n: [i8; 3], c: [f32; 3], u: [i8; 2]) -> common::Vertex {
    return common::Vertex {
        position: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
        normal: [n[0] as f32, n[1] as f32, n[2] as f32, 1.0],
        color: [c[0] as f32, c[1] as f32, c[2] as f32, 1.0],
        uv: [u[0] as f32, u[1] as f32, 0.0, 0.0],
    }
}

fn create_vertices(vertices: Vec<[i8; 3]>, normals: Vec<[i8; 3]>, colors: Vec<[f32; 3]>, uvs: Vec<[i8; 2]>) -> Vec<common::Vertex> {
    let mut data:Vec<common::Vertex> = Vec::with_capacity(vertices.len());
    for i in 0..vertices.len() {
        data.push(vertex(vertices[i], normals[i], colors[i], uvs[i]));
    }
    return data.to_vec()
}

fn main(){
    let vertex_data_cube = create_vertices(
        vertex_data::cube_positions(), vertex_data::cube_normals(), 
        vertex_data::cube_colors(), vertex_data::cube_uvs()
    );

    let mut game_data = common::GameData::new();

    let chunk_data = chunk::generate_chunk();
    let (chunk_vertices, chunk_normals, chunk_colors, chunk_uvs) = chunk::render_chunk(&chunk_data);
    let vertex_data_chunk = create_vertices(chunk_vertices, chunk_normals, chunk_colors, chunk_uvs);
    game_data.add_object(vertex_data_chunk.clone(), (-1.0, 2.0, 1.0));

    game_data.add_object(vertex_data_cube.clone(), (-1.0, 2.0, 1.0));

    let light_data = common::light([1.0,1.0,1.0], [1.0, 1.0, 0.0], 0.05, 0.6, 0.3, 30.0);
    common::run(/*&vertex_data_cube_1, */game_data, light_data, "Polydural");
}