// fixed point iteration - value mapped to itself by the function: f(x) = x 
// where the curve intersects the line y=x? Yes
// - repeatedly applying function over and over again 
// - converges to fixed point
// - how fast does it converge?
// - derivative of fixed point < 1 to converge fast
// - or slope > 45 degrees
// attractors vs. repellers
// repeatedly applying x_{n+1} = rx_n(1-x_n)

use core::borrow;
use std::borrow::BorrowMut;

/// repeatedly applies the quadratic function
/// x - starting value
/// 
/// n - number of iterations
/// 
/// r - constant that modifies the size of the parabola (0.377)
fn logistic_map(x: f64, n: i64, r: f64) -> Vec<f64> {
    let mut xprime = r*x*(1.0-x);
    let mut nums = Vec::new();

    for _ in (0..n).into_iter() {
        
        nums.push(xprime);
        xprime = r*xprime*(1.0-xprime);
        
    };

    nums
}




pub fn iterative_logistic_map(x: f64, n: i64, start_range: f64, step_size: f64, num_steps: i64) -> Vec<Vec<f64>> {
    let mut logistic_map_outputs = Vec::new();
    let mut start = start_range;

    for i in (0..num_steps).into_iter() {
        let output = logistic_map(x, n, start);
        logistic_map_outputs.push(output);
        println!("{}", start);
        start += step_size;
    }

    logistic_map_outputs
}



