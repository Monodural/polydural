pub struct Inventory {
    pub slots: Vec<(i32, i8)>
}
impl Inventory {
    pub fn new() -> Self {
        Inventory {
            slots: vec![(1, 99), (-1, -1), (-1, -1), (-1, -1), (-1, -1), (-1, -1), (-1, -1), (-1, -1), (-1, -1)]
        }
    }
}