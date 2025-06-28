use crate::world::assets::load_biomes;
use crate::world::{object::Object};
use crate::world::generation::randomness;
use crate::world::biomes;

pub struct World {
    randomness_functions: randomness::RandomnessFunctions,

    biomes: biomes::Biomes,

    objects: Vec<Object>
}
impl World {
    pub fn new() -> Self{
        let biomes: biomes::Biomes = biomes::Biomes::new();
        load_biomes(&biomes);

        Self {
            randomness_functions: randomness::RandomnessFunctions::new(),
            biomes: biomes,
            objects: Vec::new()
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn get_objects(&self) -> &Vec<Object> {
        &self.objects
    }
}
