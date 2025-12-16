
/// # Euler Step
/// takes one step along the curve
///  
/// `state` - set of discrete poitns representing the function  
/// 
/// `step` - can also be thought of as `dt`
///   
/// `f` - the functions derivative
/// 
/// `t` - start along horizontal axis
pub fn euler_step<F>(state: &mut [f64], t: f64, dt: f64, f: F)
where F:Fn(f64, &[f64]) -> Vec<f64> {
    // find slopes for each point in the array
    let deriv = f(t, state);
    println!("{}", deriv[0]);
    

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
/// __state__ - inital state vector. Holds information about the current state like poisition, velocity, etc.. Simply a list of variables that need to be tracked and are unified by the input variable, `t`
/// 
/// __t__ - starting position along t/x/horizontal axis. While it could be thought of as just another piece of 
/// information about our function (in other words, another part of the state vector), we choose to seperate 
/// it into it's own variablebecause it is independent of everything else. It will always change at the 
/// same rate without dpeending on other variables.
/// 
/// __dt__ - time step taken
/// 
/// __f__ - derivative of "position" (or some other value?) function. Perhaps more generally, this can be thought of as "the rule" to get the "next state" for the vector, re: below
/// 
/// ### What is the derivative?
/// Mathematically speaking, the derivative gives the slope at a certain point. But in the case of differential equations,
/// we need to speak more generally on what the derivative actually is:
/// A _derivative_ can be seen as a rule to follow to retrieve some prediction about what the next value of the function could be based on the current 
/// point, or, to speak in higher dimensions (and, conveniently, more specific to our use case), _state_. For example. consider the following second-order differntial equation for a spring-mass system:
/// ```latex
/// x′′=−kx
/// ```
/// By reducing the order of the differential equation, it can also be written as:
/// ```latex
/// x′= v
/// v′= −kx​
/// ```
/// our state vector is now everything on the rhs of the equal signs:
/// `state = [x,v]`, each of which is tracked by `t`. Additionally, we have the rules for our derivatives, meaning our rk4 integrator _can_ solve our system.
/// 
/// Here is the appropriate rust code:
/// 
/// ```rust
/// let f = |_x: f64, s: &[f64]| -> Vec<f64> {
///     // if our state vector, s, is [x,v]
///     vec![s[1],
///          -k*s[0]
///     ]
/// }
/// ```
/// remember: this does not give the updated values (that is the job of the rk4_step function), this simply returns the derivative. Note that in this case, 
/// we do not use x as it is not part of our rules.
/// 
/// #### __But why did we choose `x′=v`__?
/// it was a matter of reducing our higher order differential equation into a single order diff.eq.. Had we made the choice for `x′′=v`, v = kx, 
/// and we have no way of knowing v and our rk4 integrator can only handle first order differnetial equations. on the other hand a choice for v = x would be redundant.
/// 
/// If we were to find the system for a single variable function:
/// ```latex
/// y`=-x
/// ```
/// and lets pretend, for a moment, that we didn't know how to integrate this, so instead we must write the code to estimate the parent function:
/// ```rust
/// let f = |x: f64, _s: &[f64]| -> Vec<f64> {
///     vec![-x]
/// }
/// ```
/// We choose to include to underscore to indicate we never use our state variable, it's merely a formality for consistencies sake
/// the previous example of a mass-spring system is simply an extension of this idea; we are just given the first-order derivative directly instead of having to perform a substitution.
///
/// 


pub fn rk4_step<F>(state: &mut [f64], t: f64, dt: f64, f: F)
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