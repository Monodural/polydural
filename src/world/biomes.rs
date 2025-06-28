pub struct Biome {
    name: String,
    temperature: f32,
    moisture: f32,
    height: u32,
    sea_level: u32
}
impl Biome {

}

pub struct Biomes {
    biomes: Vec<Biome>
}
impl Biomes {
    pub fn new() -> Self {
        Self {
            biomes: Vec::new()
        }
    }
}