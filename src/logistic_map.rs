use std::error::Error;

use crate::dynamical_system::DynamicalSystem;

// fixed point iteration - value mapped to itself by the function: f(x) = x 
// where the curve intersects the line y=x? Yes
// - repeatedly applying function over and over again 
// - converges to fixed point
// - how fast does it converge?
// - derivative of fixed point < 1 to converge fast
// - or slope > 45 degrees
// attractors vs. repellers
// repeatedly applying x_{n+1} = rx_n(1-x_n)

pub struct LogisticMap {
    pub r: f64
}

impl DynamicalSystem for LogisticMap {
    fn dimension(&self) -> usize {
        1
    }

    fn rhs(&self, t: f64, _state: &[f64]) -> Vec<f64> {
        vec![
            self.r*t*(1.0-t)
        ]
    }
}

impl Default for LogisticMap {
    fn default() -> Self {
        Self { r: 0.3 }
    }
}


/// repeatedly applies the quadratic function. Only defined on [0,1]
/// x - starting value. 0 < x < 1
/// 
/// n - number of iterations
/// 
/// r - constant that modifies the size of the parabola (0.377)
pub fn logistic_map(x: f64, n: i64, r: f64) -> Vec<f64> {
    let mut xprime = r*x*(1.0-x);
    let mut nums = Vec::new();

    for _ in 0..n {
        
        nums.push(xprime);
        xprime = r*xprime*(1.0-xprime);
        
    };

    nums
}


