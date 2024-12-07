pub fn cube_positions() -> Vec<[i8; 3]> {
    [
        [-1, -1,  1], [1, -1,  1], [-1,  1,  1], [-1,  1,  1], [ 1, -1,  1], [ 1,  1,  1],
        [ 1, -1,  1], [1, -1, -1], [ 1,  1,  1], [ 1,  1,  1], [ 1, -1, -1], [ 1,  1, -1],
        [ 1, -1, -1], [-1, -1, -1], [1,  1, -1], [ 1,  1, -1], [-1, -1, -1], [-1,  1, -1],
        [-1, -1, -1], [-1, -1,  1], [-1,  1, -1], [-1,  1, -1], [-1, -1,  1], [-1,  1,  1],
        [-1,  1,  1], [ 1,  1,  1], [-1,  1, -1], [-1,  1, -1], [ 1,  1,  1], [ 1,  1, -1],
        [-1, -1, -1], [ 1, -1, -1], [-1, -1,  1], [-1, -1,  1], [ 1, -1, -1], [ 1, -1,  1],
    ].to_vec()
}

/*pub fn cube_colors() -> Vec<[i8; 3]> {
    [
        [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1],
        [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0],      
        [1, 1, 0], [1, 1, 0], [1, 1, 0], [1, 1, 0], [1, 1, 0], [1, 1, 0],
        [0, 1, 1], [0, 1, 1], [0, 1, 1], [0, 1, 1], [0, 1, 1], [0, 1, 1],
        [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0],
        [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1], [1, 0, 1],
    ].to_vec()
}*/

pub fn cube_normals() -> Vec<[i8; 3]> {
    [
        [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1], [0, 0, 1],
        [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0], [1, 0, 0],        
        [0, 0, -1], [0, 0, -1], [0, 0, -1], [0, 0, -1], [0, 0, -1], [0, 0, -1],
        [-1, 0, 0], [-1, 0, 0], [-1, 0, 0], [-1, 0, 0], [-1, 0, 0], [-1, 0, 0],
        [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0], [0, 1, 0],
        [0, -1, 0], [0, -1, 0], [0, -1, 0], [0, -1, 0], [0, -1, 0], [0, -1, 0],
    ].to_vec()
}