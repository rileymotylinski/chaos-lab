// fixed point iteration - value mapped to itself by the function: f(x) = x 
// where the curve intersects the line y=x? Yes
// - repeatedly applying function over and over again 
// - converges to fixed point
// - how fast does it converge?
// - derivative of fixed point < 1 to converge fast
// - or slope > 45 degrees
// attractors vs. repellers
// repeatedly applying x_{n+1} = rx_n(1-x_n)




pub fn logistic_data(x: f64, n: i64, r: f64) -> Vec<Vec<f64>> {
    let num_decimal_places = 2;
    let pow_10 = (10 as i64).pow(num_decimal_places);

    let bottom_range = (r.floor() as i64)*pow_10;
    let top_range=  (r.ceil() as i64)*pow_10;

    let mut logistic_map_outputs = Vec::new();
    

    for i in (bottom_range..top_range).into_iter() {
        let output = logistic_map(x, n, i as f64/pow_10 as f64);
        logistic_map_outputs.push(output);
    }

    logistic_map_outputs
}

/// repeatedly applies the quadratic function
/// does this really need to be a hash set?
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


