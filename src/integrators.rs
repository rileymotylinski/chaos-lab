
/// # Euler Step
/// uses eulers method of integration to find the area under the given curve 
///  
/// `state` - set of discrete poitns representing the function  
/// 
/// `step` - can also be thought of as `dt`
///   
/// `f` - the functions derivative
/// 
/// `t`
pub fn euler_step<F>(state: &mut [f64], t: f64, dt: f64, f: F)
where F:Fn(f64, &[f64]) -> Vec<f64> {
    // find slopes for each point in the array
    let deriv = f(t, state);

    // setting each value in the array to 
    // the area of the rectangle under the curve
    for i in 0..state.len() {
        state[i] += dt*deriv[i];
    }
}


/// rk4 integration to predict next vector given the functions derivative
/// ### What is "the function"
/// - simply some vector output function that represents an 
/// objects position (or some other value?) at time _t_ (must be a single varaible)
/// 
/// __state__ - inital state vector
/// 
/// __t__ - starting position along t/x/horizontal axis
/// 
/// __dt__ - time step taken
/// 
/// __f__ - derivative of "position" (or some other value?) function

pub fn rk4_step<F>(state: &mut [f64],t: f64, dt: f64, f: F)
where F:Fn(f64, &[f64]) -> Vec<f64> {
    // k1​=f(t,x)
    let k1 = f(t,state);

    // k2​=f(t+2dt​,x+2dt​k1​)
    // x + 0.5dt​k1
    // the reason we pass in the previous k into k2 is because we are using the direction of the previously 
    // determined vector to find the slope at some halfway predicted point to find new direction of the point.
    // We do this 4 times, then take a big, full step in the predicted direction.
    // almost like using more derivatives in the taylor series to better approximate a function at a point
    // multiplied by 0.5 as a "weighted" average of the directions
    let k2_data: Vec<f64> = state.iter()
    .zip(&k1)
    .map(|(s,k)| {s + (k * 0.5 * dt)})
    .collect();

    let k2 = f(t + (dt*0.5), &k2_data);

    // x + 0.5dt​k2
    let k3_data: Vec<f64> = state.iter()
    .zip(&k2)
    .map(|(s,k)| {s + (k * 0.5 * dt)})
    .collect();

    let k3 = f(t + (dt*0.5), &k3_data);

    let k4_data: Vec<f64> = state.iter()
    .zip(&k3)
    .map(|(s,k)| {s + (k * 0.5 * dt)})
    .collect();

    let k4 = f(t + dt, &k4_data);
    

    // state + (dt / 6.0)*(k1 + (2*k2) + (2*k3) + k4)
    // dividing by 6 because you are averaging six numbers
    for i in 0..state.len() {
        state[i] = state[i] + ((dt / 6.0) * (k1[i] + (2.0 * k2[i]) + (2.0 * k3[i]) + k4[i]));
    }
    
    
}