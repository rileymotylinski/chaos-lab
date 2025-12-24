use crate::{dynamical_system::DynamicalSystem};
pub struct DoublePendulum {
    pub m1: f64,
    pub m2: f64,

    pub l1: f64,
    pub l2: f64
}

impl DynamicalSystem for DoublePendulum {
    fn dimension(&self) -> usize {
        4
    }

    fn rhs(&self, _t: f64, state: &[f64]) -> Vec<f64> {
        // we know theta' = theta_dot = omega, so redefie the differential equation somehow that way
        // both omegas must be tracked
        // can just add an arbitrary number of variables to "Track"

        // if s = [theta_1, theta_2, omega_1, omega_2]
            
        // for clarity while reading code
        let theta1 = state[0];
        let theta2 = state[1];
        let omega1 = state[2];
        let omega2 = state[3];
        
        vec![
            omega1,
            omega2,
            omega_1_prime(self.m1,self.m2, self.l1, self.l2, theta1, theta2, omega1, omega2),
            omega_2_prime(self.m1, self.m2, self.l1, self.l2, theta1, theta2, omega1, omega2)
        ]
    }
}

impl Default for DoublePendulum {
    fn default() -> Self {
        Self { m1: 1.0, m2: 1.0, l1: 1.0, l2: 1.0 }
    }
}


// just some algebraic nastiness to get the appropriate derivatives for our system
// re: https://ode-solver.readthedocs.io/en/master/double-pendulum-example.html
fn omega_1_prime(m1: f64, m2:f64, l1: f64, l2: f64, theta1: f64, theta2: f64, omega1: f64, omega2: f64) -> f64 {
    let g = 9.81;
    let delta = theta1 - theta2;

    let term1 = -1.0*g*(2.0*m1 + m2)*theta1.sin();
    let term2 = m2*g*(theta1 - (2.0*theta2)).sin();
    let term3 = 2.0*m2*delta.sin()*(omega2*omega2*l2 + omega1*omega1*l1*delta.cos());

    let result = term1 - term2 - term3;
    let denom = l1 * ((2.0*m1) + m2 - (m2*(2.0*delta).cos()));
    
    // protecting against sudden/chaotic divergence. 
    // While not totally accurate, as long as the divergence
    // isn't unreasonable it's  still a good simulation
    if denom.abs() < 1e-6 {
        return 0.0; 
    }

    result / denom
}

// more algebraic nastiness. re: above
fn omega_2_prime(m1: f64, m2:f64, l1: f64, l2: f64, theta1: f64, theta2: f64, omega1: f64, omega2: f64) -> f64 {
    let g = 9.81;
    let delta = theta1 - theta2;
    let sum_masses = m1 + m2;

    let term1 = omega1*omega1*l1*sum_masses;
    let term2 = g*sum_masses*theta1.cos();
    let term3 = omega2*omega2*l2*m2*delta.cos();

    let result = 2.0 * delta.sin() * (term1 + term2 + term3);
    let denom = l2 * ((2.0*m1) + m2 - (m2*(2.0*delta).cos()));

    if denom.abs() < 1e-6 {
        return 0.0; // or damped fallback
    }

    result / denom

}