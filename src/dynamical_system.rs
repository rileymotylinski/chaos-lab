
/// for telling our rk4 integrator how to step the current state
pub trait DynamicalSystem {
    fn dimension(&self) -> usize; // number of dimensions for the system
    fn rhs(&self, t: f64, state: &[f64]) -> Vec<f64>; // returns the next state of the system given the rules (ode's) of the system
}


