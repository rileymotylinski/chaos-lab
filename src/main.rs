use crate::logistic_map::{iterative_logistic_map, write_logistic_data};
use std::process;

mod math;
mod integrators;
mod logistic_map;
mod rng;
mod tests;
mod lorenz;
mod double_pendulum;

fn main() {
    //let simulation = std::env::args().nth(1).expect("No Simulation was given");

    //println!("Simulation selected: {:?}", simulation);

    // logistic map stuff
    let path = "./test.csv";
    // generating data
    let data = iterative_logistic_map(0.3,10,3.6, 0.0001,2000);

    if let Err(err) = crate::double_pendulum::double_pendulum() {
        println!("{}", err);
        process::exit(1);
    }

    // lorenz attractor stuff
    // writing data
    if let Err(err) = crate::lorenz::lorenz() {
        println!("{}", err);
        process::exit(1);
    }
}


