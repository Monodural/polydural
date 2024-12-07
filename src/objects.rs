#[path="../src/common.rs"]
mod common;

pub fn vertex(p:[i8; 3], n: [i8; 3]) -> common::Vertex {
    return common::Vertex {
        position: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
        normal: [n[0] as f32, n[1] as f32, n[2] as f32, 1.0],
    }
}

pub fn create_vertices(vertices: Vec<[i8; 3]>, normals: Vec<[i8; 3]>) -> Vec<common::Vertex> {
    let mut data:Vec<common::Vertex> = Vec::with_capacity(vertices.len());
    for i in 0..vertices.len() {
        data.push(vertex(vertices[i], normals[i]));
    }
    return data.to_vec()
}