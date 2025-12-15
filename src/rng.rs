use rand::prelude::*;

pub fn rng_generator(seed: u64) -> StdRng {
    StdRng::seed_from_u64(seed)
}