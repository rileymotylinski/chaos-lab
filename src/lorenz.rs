use crate::dynamical_system::DynamicalSystem;

pub struct Lorenz {
    pub sigma: f64,
    pub ro: f64,
    pub beta: f64
}

impl DynamicalSystem for Lorenz {
    fn dimension(&self) -> usize {
        3
    }

    fn rhs(&self, _t: f64, state: &[f64]) -> Vec<f64> {
        vec![
            self.sigma*(state[1]-state[0]),
            (state[0]*(self.ro-state[2]))-state[1],
            (state[0]*state[1]) - (self.beta*state[2])
        ]
    }
}

impl Default for Lorenz {
    fn default() -> Self {
        // default chaotic state for lorenz attractor
        // use conventional values: sigma=10, rho=28, beta=8/3
        Self { sigma: 10.0, ro: 28.0, beta: 8.0/3.0 }
    }
}

