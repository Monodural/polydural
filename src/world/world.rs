use crate::world::{object::Object};
use crate::world::generation::randomness;
use crate::world::biomes;

pub struct World {
    random_functions: randomness::RandomnessFunctions,
    biomes: biomes::Biomes,
    objects: Vec<Object>
}
impl World {
    pub fn new() -> Self{
        Self {
            random_functions: randomness::RandomnessFunctions::new(),
            biomes: biomes::Biomes::new(),
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

pub fn create_world() -> World {
    let world: World = World::new();
    world
}