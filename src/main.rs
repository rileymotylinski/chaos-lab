use crate::logistic_map::{iterative_logistic_map, write_logistic_data};
use std::process;

mod math;
mod integrators;
mod logistic_map;
mod rng;
mod tests;

fn main() {
    //let simulation = std::env::args().nth(1).expect("No Simulation was given");

    //println!("Simulation selected: {:?}", simulation);

    let path = "./test.csv";
    // generating data
    let data = iterative_logistic_map(0.2,5,3.7, 0.001,5);

    // writing data
    if let Err(err) = write_logistic_data(data, path) {
        println!("{}", err);
        process::exit(1);
    }
}


