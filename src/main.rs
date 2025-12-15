use crate::logistic_map::iterative_logistic_map;

mod math;
mod integrators;
mod logistic_map;
mod rng;
mod tests;

fn main() {
    let simulation = std::env::args().nth(1).expect("No Simulation was given");

    println!("Simulation selected: {:?}", simulation);

    let _data = iterative_logistic_map(1.0, 50, 3.5, 0.01,50);


}


