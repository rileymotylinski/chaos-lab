/// finds the largest lyapunov exponent for a given function via double trajectory method
/// 
/// `x0` - inital state vector in this case, x0 is *NOT* the value along the horizontal axis, but rather the inital state "vector" (to generalize) which will evolve across time (n)
/// 
/// `n` - number of iterations to run to calculate the lyapunov exponent
/// 
/// `f` - function to calculate the lyapunov exponent for 
/// 
/// ### So what is the lyapunov exponent telling us?
/// The simpilest example is that of the logistic map. The lyapunov attempts to answer the
/// question of how quickly the two trajectories representing the output of the logistic map at each step diverge. 
/// If we choose some arbirtry r value and another at r + \epsilon, how quickly do those values Seperate? 
/// 
/// For a visual example, think about the shape of a parabole and how the constant in front modifies the shape.
/// We are essentially picking that constant and another constant epsilon close and testing how "different" those two parabolas are.
/// 
/// Think about the double pendulum system and how it can differ greatly in 
/// given a dynamical system (a series of evolving state vectors), we must see the divergence for certain values of 
/// 
/// ### An interesting note for later
/// 
pub fn lyapunov<F>(x0: f64, n: i64, f:F) -> f64 
where F: Fn(f64) -> f64 {
    // initial starting vector state
    let mut x = x0;
    // arbitrary difference in starting values.
    // In the pendulum system this could mean the difference in starting with an angular velocity of 0.1 and 0.2
    let eps = 1e-8;
    // the starting state vector for the other trajectory. "perturbed" trajectory in formal terms
    let mut x_pert = x + eps;
    // inital differnce in the two trajectories
    let mut sum = 0.0;

    for _ in 0..n {
        // advancing the system
        x = f(x);
        // advancing the nearby trajectory
        x_pert = f(x_pert);

        // difference between the two trajectories
        let delta = (x_pert-x).abs();

        // if the differnce is ever zero, we can't take ln of that
        if delta == 0.0 {
            continue;
        }

        // multiplicative growth -> additive growth. Easier to averge. Think about it: it chaotic systems, the rate of divergence is exponential,
        // so the lyapunov exponent would be dominated by the most recent term. by taking the log of the ratio, we "scrunch down"
        // (think about the shape of the log graph) the large numbers so that the lyapunov exponent is more representative of the functions divergence.
        sum += (delta/eps).ln();

        // renormalizing
        // you are not adding epsilon every time, but rather the same relative value between x and x_pert everytime
        // once the divergence between the two trajactories starts to increase, our original eps becomes useless,
        // so we must rescale it to measure the linear growth 
        // modifying eps by multiplicative growth we see re: above step where we take the log of the multiplicative growth we see 
        x_pert = x + (eps * (x_pert - x) / delta);
     }

    // divide by n because the lyapunov exponent is the average rate of divergence
    sum / (n as f64)
    

}
