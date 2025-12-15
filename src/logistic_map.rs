// fixed point iteration - value mapped to itself by the function: f(x) = x 
// where the curve intersects the line y=x? Yes
// - repeatedly applying function over and over again 
// - converges to fixed point
// - how fast does it converge?
// - derivative of fixed point < 1 to converge fast
// - or slope > 45 degrees
// attractors vs. repellers
// repeatedly applying x_{n+1} = rx_n(1-x_n)

/// repeatedly applies the quadratic function
/// x - starting value. 0 < x < 1
/// 
/// n - number of iterations
/// 
/// r - constant that modifies the size of the parabola (0.377)
pub fn logistic_map(x: f64, n: i64, r: f64) -> Vec<f64> {
    let mut xprime = r*x*(1.0-x);
    let mut nums = Vec::new();

    for _ in (0..n).into_iter() {
        
        nums.push(xprime);
        xprime = r*xprime*(1.0-xprime);
        
    };

    nums
}



/// runs logistic_map over a range of values
/// 
/// `x` - initial starting value < 1
/// 
/// `n` - number of iterations to run logistic map
/// 
/// `start_r` - starting r value (recall: modifies size of parabola)
/// 
/// `r_step_size` - amount to increment r by
/// 
/// `r_num_steps` - number of times to increase r by
pub fn iterative_logistic_map(x: f64, n: i64, start_r: f64, r_step_size: f64, r_num_steps: i64) -> Vec<Vec<f64>> {
    let mut logistic_map_outputs = Vec::new();
    let mut start = start_r;

    for i in (0..r_num_steps).into_iter() {
        let output = logistic_map(x, n, start);
        
      
        logistic_map_outputs.push(output);
        
        start += r_step_size;
    }
    
    logistic_map_outputs
}



