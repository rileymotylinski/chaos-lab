mod math;
mod integrators;
mod logistic_map;
mod rng;
mod tests;

use rand::RngCore;

fn main() {
    let data = logistic_map::logistic_data(0.7, 50, 3.7);
  
    let mut g = rng::rng_generator(12345);
    println!("{}",g.next_u32());
    println!("{}",data[0][0]);
}


