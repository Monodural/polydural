use noise::Perlin;
use rand::Rng;

#[derive(Clone)]
pub struct RandomnessFunctions {
    //pub rng: ThreadRng,
    pub noise: Perlin
}
impl RandomnessFunctions {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let seed: u32 = rng.gen_range(0..1000000);
        println!("seed: {}", seed);

        RandomnessFunctions {
            //rng: rand::thread_rng(),
            noise: Perlin::new(seed)
        }
    }
}