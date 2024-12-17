pub struct Inventory {
    pub hotbar_slots: Vec<(i32, i8)>
}
impl Inventory {
    pub fn new() -> Self {
        Inventory {
            hotbar_slots: vec![(1, 99), (-1, 0), (-1, 0), (-1, 0), (-1, 0), (-1, 0), (-1, 0), (-1, 0), (-1, 0)]
        }
    }
}