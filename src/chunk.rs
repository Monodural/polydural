use crate::common;

pub fn generate_chunk(chunk_position_x: i64, chunk_position_y: i64, chunk_position_z: i64) -> Vec<i8> {
    let mut chunk: Vec<i8> = Vec::new();

    for x in 0..32 {
        for y in 0..32 {
            for z in 0..32 {
                let position_x = (x + 32 * chunk_position_x) as f32;
                let position_y = (y + 32 * chunk_position_y) as f32;
                let position_z = (z + 32 * chunk_position_z) as f32;

                let terrain_max_height = (16.0 + ((position_x + position_z) / 10.0).sin() * 5.0).floor();

                if (position_x.powf(2.0) + (position_y - 16.0).powf(2.0) + position_z.powf(2.0)).sqrt() < 10.0 {
                    chunk.push(0);
                } else {
                    if position_y > terrain_max_height - 4.0 && position_y < terrain_max_height {
                        chunk.push(1);
                    } else if position_y < terrain_max_height {
                        chunk.push(3);
                    } else if position_y == terrain_max_height {
                        chunk.push(2);
                    } else {
                        chunk.push(0);
                    }
                }
            }
        }
    }

    return chunk;
}

pub fn set_block(chunk: Vec<i8>, x: i8, y: i8, z: i8, block_type: i8) -> Vec<i8> {
    if x > 31 || y > 31 || z > 31 { return chunk; }
    if x < 0 || y < 0 || z < 0 { return chunk; }
    let mut new_chunk = chunk;
    new_chunk[x as usize * 32 * 32 + y as usize * 32 + z as usize] = block_type;
    return new_chunk;
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

                if get_block(&chunk, x + 1, y + 1, z) > 0 { // right top (6)
                    directions.push(true)
                } else {
                    directions.push(false);
                }
                if y > 0 && get_block(&chunk, x + 1, y - 1, z) > 0 { // right bottom (7)
                    directions.push(true)
                } else {
                    directions.push(false);
                }
                if x > 0 && get_block(&chunk, x - 1, y + 1, z) > 0 { // left top (8)
                    directions.push(true)
                } else {
                    directions.push(false);
                }
                if x > 0 && y > 0 && get_block(&chunk, x - 1, y - 1, z) > 0 { // left bottom (9)
                    directions.push(true)
                } else {
                    directions.push(false);
                }

                if get_block(&chunk, x, y + 1, z + 1) > 0 { // front top (10)
                    directions.push(true)
                } else {
                    directions.push(false);
                }
                if y > 0 && get_block(&chunk, x, y - 1, z + 1) > 0 { // front bottom (11)
                    directions.push(true)
                } else {
                    directions.push(false);
                }
                if z > 0 && get_block(&chunk, x, y + 1, z - 1) > 0 { // back top (12)
                    directions.push(true)
                } else {
                    directions.push(false);
                }
                if z > 0 && y > 0 && get_block(&chunk, x, y - 1, z - 1) > 0 { // back bottom (13)
                    directions.push(true)
                } else {
                    directions.push(false);
                }

                if get_block(&chunk, x + 1, y, z + 1) > 0 { // right front (14)
                    directions.push(true)
                } else {
                    directions.push(false);
                }
                if z > 0 && get_block(&chunk, x + 1, y, z - 1) > 0 { // right back (15)
                    directions.push(true)
                } else {
                    directions.push(false);
                }
                if x > 0 && get_block(&chunk, x - 1, y , z + 1) > 0 { // left front (16)
                    directions.push(true)
                } else {
                    directions.push(false);
                }
                if z > 0 && x > 0 && get_block(&chunk, x - 1, y, z - 1) > 0 { // left back (17)
                    directions.push(true)
                } else {
                    directions.push(false);
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

                    if !directions[7] {
                        colors.push([1.0, 1.0, 1.0]);
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    if !directions[7] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
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

                    if !directions[9] {
                        colors.push([1.0, 1.0, 1.0]);
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    if !directions[9] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
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

                    if !directions[8] && ! directions[10] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    if !directions[6] && ! directions[10] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    if !directions[8] && ! directions[12] {
                        colors.push([1.0, 1.0, 1.0]);
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    if !directions[6] && ! directions[10] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    if !directions[6] && ! directions[12] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
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

                    if !directions[9] && !directions[13] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    if !directions[7] && !directions[13] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    if !directions[9] && !directions[11] {
                        colors.push([1.0, 1.0, 1.0]);
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    if !directions[7] && !directions[13] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    if !directions[7] && !directions[11] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
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

                    if !directions[11] {
                        colors.push([1.0, 1.0, 1.0]);
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    if !directions[11] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
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

                    if !directions[13] {
                        colors.push([1.0, 1.0, 1.0]);
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    colors.push([1.0, 1.0, 1.0]);
                    colors.push([1.0, 1.0, 1.0]);
                    if !directions[13] {
                        colors.push([1.0, 1.0, 1.0]);
                    } else  {
                        colors.push([0.5, 0.5, 0.5]);
                    }
                    colors.push([1.0, 1.0, 1.0]);
                }
            }
        }
    }

    return (vertices, normals, colors, uvs);
}

pub fn get_block(chunk: &Vec<i8>, x: usize, y: usize, z: usize) -> i8 {
    if x > 31 || y > 31 || z > 31 { return -1; } // return block tipe -1 to signal a chunk border
    return chunk[x * 32 * 32 + y * 32 + z];
}