pub fn generate_chunk() -> Vec<i8> {
    let mut chunk: Vec<i8> = Vec::new();

    for x in 0..32 {
        for y in 0..32 {
            for z in 0..32 {
                let position_x = x as f32;
                let position_y = y as f32;
                let position_z = z as f32;

                if position_y < 16.0 + ((position_x + position_z) / 10.0).sin() * 5.0 {
                    chunk.push(1);
                } else {
                    chunk.push(0);
                }
            }
        }
    }

    return chunk;
}

pub fn render_chunk(chunk: &Vec<i8>) -> (Vec<[i8; 3]>, Vec<[i8; 3]>, Vec<[f32; 3]>, Vec<[i8; 2]>) {
    let mut vertices: Vec<[i8; 3]> = Vec::new();
    let mut normals: Vec<[i8; 3]> = Vec::new();
    let mut colors: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[i8; 2]> = Vec::new();

    for x in 0..32 {
        for y in 0..32 {
            for z in 0..32 {
                if get_block(&chunk, x, y, z) == 0 { continue; }

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

                    uvs.push([0, 0]);
                    uvs.push([1, 0]);
                    uvs.push([0, 1]);
                    uvs.push([0, 1]);
                    uvs.push([1, 0]);
                    uvs.push([1, 1]);

                    normals.push([1, 0, 0]);
                    normals.push([1, 0, 0]);
                    normals.push([1, 0, 0]);
                    normals.push([1, 0, 0]);
                    normals.push([1, 0, 0]);
                    normals.push([1, 0, 0]);

                    colors.push([0.16, 0.1, 0.07]);
                    colors.push([0.16, 0.1, 0.07]);
                    colors.push([0.35, 0.13, 0.1]);
                    colors.push([0.35, 0.13, 0.1]);
                    colors.push([0.16, 0.1, 0.07]);
                    colors.push([0.35, 0.13, 0.1]);
                }
                if !directions[1] {
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);

                    uvs.push([0, 0]);
                    uvs.push([1, 0]);
                    uvs.push([0, 1]);
                    uvs.push([0, 1]);
                    uvs.push([1, 0]);
                    uvs.push([1, 1]);

                    normals.push([-1, 0, 0]);
                    normals.push([-1, 0, 0]);
                    normals.push([-1, 0, 0]);
                    normals.push([-1, 0, 0]);
                    normals.push([-1, 0, 0]);
                    normals.push([-1, 0, 0]);

                    colors.push([0.16, 0.1, 0.07]);
                    colors.push([0.16, 0.1, 0.07]);
                    colors.push([0.35, 0.13, 0.1]);
                    colors.push([0.35, 0.13, 0.1]);
                    colors.push([0.16, 0.1, 0.07]);
                    colors.push([0.35, 0.13, 0.1]);
                }
                if !directions[2] {
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);

                    uvs.push([0, 0]);
                    uvs.push([1, 0]);
                    uvs.push([0, 1]);
                    uvs.push([0, 1]);
                    uvs.push([1, 0]);
                    uvs.push([1, 1]);

                    normals.push([0, 1, 0]);
                    normals.push([0, 1, 0]);
                    normals.push([0, 1, 0]);
                    normals.push([0, 1, 0]);
                    normals.push([0, 1, 0]);
                    normals.push([0, 1, 0]);

                    colors.push([0.0, 1.0, 0.0]);
                    colors.push([0.0, 1.0, 0.0]);
                    colors.push([0.0, 1.0, 0.0]);
                    colors.push([0.0, 1.0, 0.0]);
                    colors.push([0.0, 1.0, 0.0]);
                    colors.push([0.0, 1.0, 0.0]);
                }
                if !directions[3] {
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);

                    uvs.push([0, 0]);
                    uvs.push([1, 0]);
                    uvs.push([0, 1]);
                    uvs.push([0, 1]);
                    uvs.push([1, 0]);
                    uvs.push([1, 1]);

                    normals.push([0, -1, 0]);
                    normals.push([0, -1, 0]);
                    normals.push([0, -1, 0]);
                    normals.push([0, -1, 0]);
                    normals.push([0, -1, 0]);
                    normals.push([0, -1, 0]);

                    colors.push([0.16, 0.1, 0.07]);
                    colors.push([0.16, 0.1, 0.07]);
                    colors.push([0.35, 0.13, 0.1]);
                    colors.push([0.35, 0.13, 0.1]);
                    colors.push([0.16, 0.1, 0.07]);
                    colors.push([0.35, 0.13, 0.1]);
                }
                if !directions[4] {
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2, -1 + block_position_y * 2,  1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2,  1 + block_position_y * 2,  1 + block_position_z * 2]);

                    uvs.push([0, 0]);
                    uvs.push([1, 0]);
                    uvs.push([0, 1]);
                    uvs.push([0, 1]);
                    uvs.push([1, 0]);
                    uvs.push([1, 1]);

                    normals.push([0, 0, 1]);
                    normals.push([0, 0, 1]);
                    normals.push([0, 0, 1]);
                    normals.push([0, 0, 1]);
                    normals.push([0, 0, 1]);
                    normals.push([0, 0, 1]);

                    colors.push([0.16, 0.1, 0.07]);
                    colors.push([0.16, 0.1, 0.07]);
                    colors.push([0.35, 0.13, 0.1]);
                    colors.push([0.35, 0.13, 0.1]);
                    colors.push([0.16, 0.1, 0.07]);
                    colors.push([0.35, 0.13, 0.1]);
                }
                if !directions[5] {
                    vertices.push([ 1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([ 1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2, -1 + block_position_y * 2, -1 + block_position_z * 2]);
                    vertices.push([-1 + block_position_x * 2,  1 + block_position_y * 2, -1 + block_position_z * 2]);

                    uvs.push([0, 0]);
                    uvs.push([1, 0]);
                    uvs.push([0, 1]);
                    uvs.push([0, 1]);
                    uvs.push([1, 0]);
                    uvs.push([1, 1]);

                    normals.push([0, 0, -1]);
                    normals.push([0, 0, -1]);
                    normals.push([0, 0, -1]);
                    normals.push([0, 0, -1]);
                    normals.push([0, 0, -1]);
                    normals.push([0, 0, -1]);

                    colors.push([0.16, 0.1, 0.07]);
                    colors.push([0.16, 0.1, 0.07]);
                    colors.push([0.35, 0.13, 0.1]);
                    colors.push([0.35, 0.13, 0.1]);
                    colors.push([0.16, 0.1, 0.07]);
                    colors.push([0.35, 0.13, 0.1]);
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