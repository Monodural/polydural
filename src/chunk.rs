use crate::common;

pub fn generate_chunk() -> Vec<i8> {
    let mut chunk: Vec<i8> = Vec::new();

    for x in 0..32 {
        for y in 0..32 {
            for z in 0..32 {
                let position_x = x as f32;
                let position_y = y as f32;
                let position_z = z as f32;

                if position_y < 16.0 + ((position_x + position_z) / 10.0).sin() * 5.0 {
                    if (position_x + position_z).sin() > 0.5 {
                        chunk.push(1);
                    } else {
                        chunk.push(2);
                    }
                } else {
                    chunk.push(0);
                }
            }
        }
    }

    return chunk;
}

pub fn render_chunk(chunk: &Vec<i8>, game_data: &common::GameData) -> (Vec<[i8; 3]>, Vec<[i8; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>) {
    let mut vertices: Vec<[i8; 3]> = Vec::new();
    let mut normals: Vec<[i8; 3]> = Vec::new();
    let mut colors: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();

    let atlas_width = 8.0;
    let atlas_height = 8.0;

    for x in 0..32 {
        for y in 0..32 {
            for z in 0..32 {
                let block_id = get_block(&chunk, x, y, z);
                if block_id == 0 { continue; }

                let mut directions = Vec::new();
                if get_block(&chunk, x + 1, y, z) != 0 {
                    directions.push(true)
                } else {
                    directions.push(false)
                }
                if x > 0 && get_block(&chunk, x - 1, y, z) != 0 {
                    directions.push(true)
                } else {
                    if x == 0 { directions.push(true); }
                    else { directions.push(false); }
                }
                if get_block(&chunk, x, y + 1, z) != 0 {
                    directions.push(true)
                } else {
                    directions.push(false)
                }
                if y > 0 && get_block(&chunk, x, y - 1, z) != 0 {
                    directions.push(true)
                } else {
                    if y == 0 { directions.push(true); }
                    else { directions.push(false); }
                }
                if get_block(&chunk, x, y, z + 1) != 0 {
                    directions.push(true)
                } else {
                    directions.push(false)
                }
                if z > 0 && get_block(&chunk, x, y, z - 1) != 0 {
                    directions.push(true)
                } else {
                    if z == 0 { directions.push(true); }
                    else { directions.push(false); }
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

                    let uv_x = (game_data.blocks[(block_id - 1) as usize].1[0] as f32 % atlas_width).floor();
                    let uv_y = (game_data.blocks[(block_id - 1) as usize].1[0] as f32 / atlas_height).floor();
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

                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                }
                if !directions[1] {
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);

                    let uv_x = (game_data.blocks[(block_id - 1) as usize].1[1] as f32 % atlas_width).floor();
                    let uv_y = (game_data.blocks[(block_id - 1) as usize].1[1] as f32 / atlas_height).floor();
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

                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                }
                if !directions[2] {
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);

                    let uv_x = (game_data.blocks[(block_id - 1) as usize].1[2] as f32 % atlas_width).floor();
                    let uv_y = (game_data.blocks[(block_id - 1) as usize].1[2] as f32 / atlas_height).floor();
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

                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                }
                if !directions[3] {
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);

                    let uv_x = (game_data.blocks[(block_id - 1) as usize].1[3] as f32 % atlas_width).floor();
                    let uv_y = (game_data.blocks[(block_id - 1) as usize].1[3] as f32 / atlas_height).floor();
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

                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                }
                if !directions[4] {
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);

                    let uv_x = (game_data.blocks[(block_id - 1) as usize].1[4] as f32 % atlas_width).floor();
                    let uv_y = (game_data.blocks[(block_id - 1) as usize].1[4] as f32 / atlas_height).floor();
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

                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                }
                if !directions[5] {
                    vertices.push([ 1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);

                    let uv_x = (game_data.blocks[(block_id - 1) as usize].1[5] as f32 % atlas_width).floor();
                    let uv_y = (game_data.blocks[(block_id - 1) as usize].1[5] as f32 / atlas_height).floor();
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

                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                }
            }
        }
    }

    return (vertices, normals, colors, uvs);
}

pub fn get_block(chunk: &Vec<i8>, x: usize, y: usize, z: usize) -> i8 {
    if x > 31 || y > 31 || z > 31 { return 1; }
    return chunk[x * 32 * 32 + y * 32 + z];
}