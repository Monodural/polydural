use crate::world::{object::Object};
use crate::world::generation::randomness;

pub struct World {
    random_functions: randomness::RandomnessFunctions,
    objects: Vec<Object>
}
impl World {
    pub fn new() -> Self{
        Self {
            random_functions: randomness::RandomnessFunctions::new(),
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