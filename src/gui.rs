use crate::common::{create_vertices, GameData};

pub fn create_frame(game_data: &mut GameData, position: (f64, f64, f64), scale: (f64, f64, f64), uvs: Vec<[f32; 2]>) {
    let vertex_data = create_vertices(
        vec![
            [-1.0 * scale.0 + position.0, 1.0 * scale.1 + position.1, 0.0 * scale.2 + position.2], 
            [1.0 * scale.0 + position.0, 1.0 * scale.1 + position.1, 0.0 * scale.2 + position.2], 
            [1.0 * scale.0 + position.0, -1.0 * scale.1 + position.1, 0.0 * scale.2 + position.2], 
            [-1.0 * scale.0 + position.0, 1.0 * scale.1 + position.1, 0.0 * scale.2 + position.2], 
            [1.0 * scale.0 + position.0, -1.0 * scale.1 + position.1, 0.0 * scale.2 + position.2], 
            [-1.0 * scale.0 + position.0, -1.0 * scale.1 + position.1, 0.0 * scale.2 + position.2]], 
        vec![[0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1]], 
        vec![[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]], 
        uvs
    );
    game_data.add_gui_object(vertex_data.clone(), (0.0, 0.0, 0.0), (1.0, 1.0, 1.0), true);
}

pub fn update_frame(game_data: &mut GameData, position: (f64, f64, f64), scale: (f64, f64, f64), uvs: Vec<[f32; 2]>, id: usize) {
    let vertex_data = create_vertices(
        vec![
            [-1.0 * scale.0 + position.0, 1.0 * scale.1 + position.1, 0.0 * scale.2 + position.2], 
            [1.0 * scale.0 + position.0, 1.0 * scale.1 + position.1, 0.0 * scale.2 + position.2], 
            [1.0 * scale.0 + position.0, -1.0 * scale.1 + position.1, 0.0 * scale.2 + position.2], 
            [-1.0 * scale.0 + position.0, 1.0 * scale.1 + position.1, 0.0 * scale.2 + position.2], 
            [1.0 * scale.0 + position.0, -1.0 * scale.1 + position.1, 0.0 * scale.2 + position.2], 
            [-1.0 * scale.0 + position.0, -1.0 * scale.1 + position.1, 0.0 * scale.2 + position.2]], 
        vec![[0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1]], 
        vec![[1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0], [1.0, 1.0, 1.0]], 
        uvs
    );
    game_data.gui_objects[id] = vertex_data.clone();
}