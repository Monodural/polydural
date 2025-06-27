pub fn render_chunk(chunk: &Vec<i8>, light_data: &Vec<i8>, game_data: &common::GameData, chunks: &HashMap<(i64, i64, i64), Vec<i8>>, world_data: &world::WorldData, chunk_position_x: i64, chunk_position_y: i64, chunk_position_z: i64) -> (Vec<[f64; 3]>, Vec<[i8; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<[f64; 3]>, Vec<[i8; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>) {
    let mut vertices: Vec<[f64; 3]> = Vec::new();
    let mut normals: Vec<[i8; 3]> = Vec::new();
    let mut colors: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();

    let mut vertices_transparent: Vec<[f64; 3]> = Vec::new();
    let mut normals_transparent: Vec<[i8; 3]> = Vec::new();
    let mut colors_transparent: Vec<[f32; 3]> = Vec::new();
    let mut uvs_transparent: Vec<[f32; 2]> = Vec::new();

    let atlas_width = 8.0;
    let atlas_height = 8.0;

    let world_data_clone = &world_data;

    for x in 0..32 {
        for z in 0..32 {
            for y in 0..32 {
                let block_id = get_block(&chunk, x, y, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z);
                if block_id == 0 { continue; }

                let light_level = light_data[(x * 32 * 32 + y * 32 + z) as usize] as f32 / 127.0;

                let shape_index = world_data.shape_index[&world_data.blocks[(block_id - 1) as usize].3];
                let elements = &world_data.shapes[shape_index - 1].1;
                let is_transparent = &world_data.blocks[(block_id - 1) as usize].5;

                if !is_transparent {
                    for element in elements {
                        let vertices_from = element.from;
                        let vertices_to = element.to;

                        let mut directions = vec![
                            false, false, false, false, false, false,
                            false, false, false, false, false, false,
                            false, false, false, false, false, false
                        ];

                        if get_block_transparent(&chunk, x + 1, y, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[0] = true;
                        }

                        if get_block_transparent(&chunk, x - 1, y, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[1] = true;
                        }

                        if get_block_transparent(&chunk, x, y + 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[2] = true;
                        }

                        if get_block_transparent(&chunk, x, y - 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[3] = true;
                        }

                        if get_block_transparent(&chunk, x, y, z + 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[4] = true;
                        }

                        if get_block_transparent(&chunk, x, y, z - 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[5] = true;
                        }

                        if get_block(&chunk, x + 1, y + 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // right top (6)
                            directions[6] = true;
                        }
                        if get_block(&chunk, x + 1, y - 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // right bottom (7)
                            directions[7] = true;
                        }
                        if get_block(&chunk, x - 1, y + 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // left top (8)
                            directions[8] = true;
                        }
                        if get_block(&chunk, x - 1, y - 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // left bottom (9)
                            directions[9] = true;
                        }

                        if get_block(&chunk, x, y + 1, z + 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // front top (10)
                            directions[10] = true;
                        }
                        if get_block(&chunk, x, y - 1, z + 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // front bottom (11)
                            directions[11] = true;
                        }
                        if get_block(&chunk, x, y + 1, z - 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // back top (12)
                            directions[12] = true;
                        }
                        if get_block(&chunk, x, y - 1, z - 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // back bottom (13)
                            directions[13] = true;
                        }

                        if get_block(&chunk, x + 1, y, z + 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // right front (14)
                            directions[14] = true;
                        }
                        if get_block(&chunk, x + 1, y, z - 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // right back (15)
                            directions[15] = true;
                        }
                        if get_block(&chunk, x - 1, y , z + 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // left front (16)
                            directions[16] = true;
                        }
                        if get_block(&chunk, x - 1, y, z - 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // left back (17)
                            directions[17] = true;
                        }

                        let block_position_x = x as f64;
                        let block_position_y = y as f64;
                        let block_position_z = z as f64;

                        if !directions[0] {
                            vertices.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

                            let uv_x = (world_data.blocks[(block_id - 1) as usize].1[0] as f32 % atlas_width).floor();
                            let uv_y = (world_data.blocks[(block_id - 1) as usize].1[0] as f32 / atlas_height).floor();
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
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            if !directions[7] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                        }
                        if !directions[1] {
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

                            let uv_x = (world_data.blocks[(block_id - 1) as usize].1[1] as f32 % atlas_width).floor();
                            let uv_y = (world_data.blocks[(block_id - 1) as usize].1[1] as f32 / atlas_height).floor();
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
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            if !directions[9] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                        }
                        if !directions[2] {
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

                            let uv_x = (world_data.blocks[(block_id - 1) as usize].1[2] as f32 % atlas_width).floor();
                            let uv_y = (world_data.blocks[(block_id - 1) as usize].1[2] as f32 / atlas_height).floor();
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
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[6] && ! directions[10] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[8] && ! directions[12] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[6] && ! directions[10] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[6] && ! directions[12] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                        }
                        if !directions[3] {
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

                            let uv_x = (world_data.blocks[(block_id - 1) as usize].1[3] as f32 % atlas_width).floor();
                            let uv_y = (world_data.blocks[(block_id - 1) as usize].1[3] as f32 / atlas_height).floor();
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
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[7] && !directions[13] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[9] && !directions[11] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[7] && !directions[13] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[7] && !directions[11] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                        }
                        if !directions[4] {
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

                            let uv_x = (world_data.blocks[(block_id - 1) as usize].1[4] as f32 % atlas_width).floor();
                            let uv_y = (world_data.blocks[(block_id - 1) as usize].1[4] as f32 / atlas_height).floor();
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
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            if !directions[11] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                        }
                        if !directions[5] {
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

                            let uv_x = (world_data.blocks[(block_id - 1) as usize].1[5] as f32 % atlas_width).floor();
                            let uv_y = (world_data.blocks[(block_id - 1) as usize].1[5] as f32 / atlas_height).floor();
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
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            if !directions[13] {
                                colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                        }
                    }
                } else {
                    for element in elements {
                        let vertices_from = element.from;
                        let vertices_to = element.to;

                        let mut directions = vec![
                            false, false, false, false, false, false,
                            false, false, false, false, false, false,
                            false, false, false, false, false, false
                        ];

                        if get_block_transparent(&chunk, x + 1, y, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[0] = true;
                        }

                        if get_block_transparent(&chunk, x - 1, y, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[1] = true;
                        }

                        if get_block_transparent(&chunk, x, y + 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[2] = true;
                        }

                        if get_block_transparent(&chunk, x, y - 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[3] = true;
                        }

                        if get_block_transparent(&chunk, x, y, z + 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[4] = true;
                        }

                        if get_block_transparent(&chunk, x, y, z - 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) == false {
                            directions[5] = true;
                        }

                        if get_block(&chunk, x + 1, y + 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // right top (6)
                            directions[6] = true;
                        }
                        if get_block(&chunk, x + 1, y - 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // right bottom (7)
                            directions[7] = true;
                        }
                        if get_block(&chunk, x - 1, y + 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // left top (8)
                            directions[8] = true;
                        }
                        if get_block(&chunk, x - 1, y - 1, z, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // left bottom (9)
                            directions[9] = true;
                        }

                        if get_block(&chunk, x, y + 1, z + 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // front top (10)
                            directions[10] = true;
                        }
                        if get_block(&chunk, x, y - 1, z + 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // front bottom (11)
                            directions[11] = true;
                        }
                        if get_block(&chunk, x, y + 1, z - 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // back top (12)
                            directions[12] = true;
                        }
                        if get_block(&chunk, x, y - 1, z - 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // back bottom (13)
                            directions[13] = true;
                        }

                        if get_block(&chunk, x + 1, y, z + 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // right front (14)
                            directions[14] = true;
                        }
                        if get_block(&chunk, x + 1, y, z - 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // right back (15)
                            directions[15] = true;
                        }
                        if get_block(&chunk, x - 1, y , z + 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // left front (16)
                            directions[16] = true;
                        }
                        if get_block(&chunk, x - 1, y, z - 1, &game_data, chunks, &world_data_clone, chunk_position_x, chunk_position_y, chunk_position_z) > 0 { // left back (17)
                            directions[17] = true;
                        }

                        let block_position_x = x as f64;
                        let block_position_y = y as f64;
                        let block_position_z = z as f64;

                        if !directions[0] && vertices_from[1] != vertices_to[1] && vertices_from[2] != vertices_to[2] {
                            vertices_transparent.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([ ( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

                            let uv_x = (world_data.blocks[(block_id - 1) as usize].1[0] as f32 % atlas_width).floor();
                            let uv_y = (world_data.blocks[(block_id - 1) as usize].1[0] as f32 / atlas_height).floor();
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);

                            normals_transparent.push([1, 0, 0]);
                            normals_transparent.push([1, 0, 0]);
                            normals_transparent.push([1, 0, 0]);
                            normals_transparent.push([1, 0, 0]);
                            normals_transparent.push([1, 0, 0]);
                            normals_transparent.push([1, 0, 0]);

                            if !directions[7] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            if !directions[7] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                        }
                        if !directions[1] && vertices_from[1] != vertices_to[1] && vertices_from[2] != vertices_to[2] {
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

                            let uv_x = (world_data.blocks[(block_id - 1) as usize].1[1] as f32 % atlas_width).floor();
                            let uv_y = (world_data.blocks[(block_id - 1) as usize].1[1] as f32 / atlas_height).floor();
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);

                            normals_transparent.push([-1, 0, 0]);
                            normals_transparent.push([-1, 0, 0]);
                            normals_transparent.push([-1, 0, 0]);
                            normals_transparent.push([-1, 0, 0]);
                            normals_transparent.push([-1, 0, 0]);
                            normals_transparent.push([-1, 0, 0]);

                            if !directions[9] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            if !directions[9] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                        }
                        if !directions[2] && vertices_from[0] != vertices_to[0] && vertices_from[2] != vertices_to[2] {
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

                            let uv_x = (world_data.blocks[(block_id - 1) as usize].1[2] as f32 % atlas_width).floor();
                            let uv_y = (world_data.blocks[(block_id - 1) as usize].1[2] as f32 / atlas_height).floor();
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);

                            normals_transparent.push([0, 1, 0]);
                            normals_transparent.push([0, 1, 0]);
                            normals_transparent.push([0, 1, 0]);
                            normals_transparent.push([0, 1, 0]);
                            normals_transparent.push([0, 1, 0]);
                            normals_transparent.push([0, 1, 0]);

                            if !directions[8] && ! directions[10] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[6] && ! directions[10] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[8] && ! directions[12] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[6] && ! directions[10] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[6] && ! directions[12] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                        }
                        if !directions[3] && vertices_from[0] != vertices_to[0] && vertices_from[2] != vertices_to[2] {
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

                            let uv_x = (world_data.blocks[(block_id - 1) as usize].1[3] as f32 % atlas_width).floor();
                            let uv_y = (world_data.blocks[(block_id - 1) as usize].1[3] as f32 / atlas_height).floor();
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);

                            normals_transparent.push([0, -1, 0]);
                            normals_transparent.push([0, -1, 0]);
                            normals_transparent.push([0, -1, 0]);
                            normals_transparent.push([0, -1, 0]);
                            normals_transparent.push([0, -1, 0]);
                            normals_transparent.push([0, -1, 0]);

                            if !directions[9] && !directions[13] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[7] && !directions[13] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[9] && !directions[11] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[7] && !directions[13] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            if !directions[7] && !directions[11] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                        }
                        if !directions[4] && vertices_from[0] != vertices_to[0] && vertices_from[1] != vertices_to[1] {
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0, (1.0 - (2.0 - vertices_to[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

                            let uv_x = (world_data.blocks[(block_id - 1) as usize].1[4] as f32 % atlas_width).floor();
                            let uv_y = (world_data.blocks[(block_id - 1) as usize].1[4] as f32 / atlas_height).floor();
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);

                            normals_transparent.push([0, 0, 1]);
                            normals_transparent.push([0, 0, 1]);
                            normals_transparent.push([0, 0, 1]);
                            normals_transparent.push([0, 0, 1]);
                            normals_transparent.push([0, 0, 1]);
                            normals_transparent.push([0, 0, 1]);

                            if !directions[11] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            if !directions[11] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                        }
                        if !directions[5] && vertices_from[0] != vertices_to[0] && vertices_from[1] != vertices_to[1] {
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([( 1.0 - (2.0 - vertices_to[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0,(-1.0 + (vertices_from[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);
                            vertices_transparent.push([(-1.0 + (vertices_from[0] / 8.0)) + block_position_x * 2.0 + chunk_position_x as f64 * 64.0, (1.0 - (2.0 - vertices_to[1] / 8.0)) + block_position_y * 2.0 + chunk_position_y as f64 * 64.0,(-1.0 + (vertices_from[2] / 8.0)) + block_position_z * 2.0 + chunk_position_z as f64 * 64.0]);

                            let uv_x = (world_data.blocks[(block_id - 1) as usize].1[5] as f32 % atlas_width).floor();
                            let uv_y = (world_data.blocks[(block_id - 1) as usize].1[5] as f32 / atlas_height).floor();
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
                            uvs_transparent.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);

                            normals_transparent.push([0, 0, -1]);
                            normals_transparent.push([0, 0, -1]);
                            normals_transparent.push([0, 0, -1]);
                            normals_transparent.push([0, 0, -1]);
                            normals_transparent.push([0, 0, -1]);
                            normals_transparent.push([0, 0, -1]);

                            if !directions[13] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            if !directions[13] {
                                colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                            } else  {
                                colors_transparent.push([0.5 * light_level, 0.5 * light_level, 0.5 * light_level]);
                            }
                            colors_transparent.push([1.0 * light_level, 1.0 * light_level, 1.0 * light_level]);
                        }
                    }
                }
            }
        }
    }

    return (vertices, normals, colors, uvs, vertices_transparent, normals_transparent, colors_transparent, uvs_transparent);
}