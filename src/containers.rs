pub struct Inventory {
    pub _slots: Vec<(i32, i8)>
}
impl Inventory {
    pub fn new() -> Self {
        Inventory {
            _slots: vec![(1, 99), (-1, -1), (-1, -1), (-1, -1), (-1, -1), (-1, -1), (-1, -1), (-1, -1), (-1, -1)]
        }
    }
}