mod math;
mod integrators;
mod logistic_map;
mod rng;
mod tests;

use rand::RngCore;

use crate::integrators::euler_step;

fn main() {
    let data = logistic_map::logistic_data(0.7, 50, 3.7);
  
    let mut g = rng::rng_generator(12345);
    println!("{}",g.next_u32());
    println!("{}",data[0][0]);


    let mut state = [1.0]; // inital x (vertical axis) value
    let t= 0.0; // initial t (horizontal axis) value 
    let dt = 0.1; // t increment

    // x is like the offset
    let f = |x: f64, s: &[f64]| -> Vec<f64> {
        s.iter().map(|n: &f64| -> f64 {x-n}).collect()
    };

    euler_step(&mut state, t, dt, f);

    print!("{}", state[0]);
}


