use noise::{NoiseFn, Perlin, Seedable};
use rand::Rng;

enum Biome{
    Forest,
}

impl Biome {
    fn value(&self) -> &str {
        match *self {
            Biome::Forest => "forest",
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();
    let perlin = Perlin::new(1);
    let val = perlin.get([42.4, 37.7, 2.8]);
    let random_biome: Biome = rand::random(); 
}

