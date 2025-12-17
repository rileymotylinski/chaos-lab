use std::error::Error;

/// Use euler-lagrange equations to model motion of a double pendulum
/// steps with rk4
/// 4 variables in state vector to reduce second order diff. eq. to first order (can be solved by rk4); two of which are trivial. Recall an n order differential equation can be written as a system of first order differential equations
/// future note : I really want to see this derivation. I understand (mathematically) how we go from the langrangian -> euler-lagrange equation, but how do we isolate omega_1, omega_2?
pub fn double_pendulum() -> Result<(), Box<dyn Error>> {

    let file_path= "./src/csv/double_pendulum.csv";
    let m1 = 1.0;
    let l1 = 1.0;

    let m2 = 1.0;
    let l2 = 1.0;

    // we know theta' = theta_dot = omega, so redefie the differential equation somehow that way
    // both omegas must be tracked
    // can just add an arbitrary number of variables to "Track"

    // if s = [theta_1, theta_2, omega_1, omega_2]
    let f = |_t: f64, s:&[f64]| -> Vec<f64> {
        
        // for clarity while reading code
        let theta1 = s[0];
        let theta2 = s[1];
        let omega1 = s[2];
        let omega2 = s[3];
        
        vec![
            omega1,
            omega2,
            omega_1_prime(m1, m2, l1, l2, theta1, theta2, omega1, omega2),
            omega_2_prime(m1, m2, l1, l2, theta1, theta2, omega1, omega2)
        ]
    };


    let mut state = [0.1,0.1,0.0,0.0]; 
    let mut t= 0.0; // initial t (horizontal axis) value  
    let dt = 0.1; // t increment

    // create csv writer
    let mut wtr = csv::Writer::from_path(file_path)?;
    // write headers
    let headers = vec!["time","theta1","theta2"];
    wtr.write_record(&headers)?;

    
    let num_steps = 50;
    for _i in 0..num_steps {

        // [t, theta1, theta2]
        let temp = vec![
            t.to_string(),
            state[0].to_string(),
            state[1].to_string()
        ];
        // write state
        wtr.write_record(&temp)?;
        // update state
        crate::integrators::rk4_step(&mut state,t,dt,f);
        // while s.o.e doesn't require t, we are writing to a csv file with t, so I need to make sure it's actually changing for future plotting
        t += dt;

    }

    

    wtr.flush()?;
    Ok(())
}

// just some algebraic nastiness to get the appropriate derivatives for our system
// re: https://ode-solver.readthedocs.io/en/master/double-pendulum-example.html
fn omega_1_prime(m1: f64, m2:f64, l1: f64, l2: f64, theta1: f64, theta2: f64, omega1: f64, omega2: f64) -> f64 {
    let g = 9.81;
    let delta = theta1 - theta2;

    let term1 = -1.0*g*(2.0*m1 + m2)*theta1.sin();
    let term2 = m2*g*(theta1 - (2.0*theta2)).sin();
    let term3 = 2.0*m2*delta.sin()*(omega2.powf(2.0)*l2 + omega1.powf(2.0)*l1*delta.cos());

    let result = term1 - term2 - term3;

    result / (l1 * ((2.0*m1)+m2-(m2*(2.0*delta).cos())))
}

fn omega_2_prime(m1: f64, m2:f64, l1: f64, l2: f64, theta1: f64, theta2: f64, omega1: f64, omega2: f64) -> f64 {
    let g = 9.81;
    let delta = theta1 - theta2;
    let sum_masses = m1 + m2;

    let result = 2.0 * delta.sin() * (omega1.powf(2.0)*l1*sum_masses + g*sum_masses*theta1.cos() + omega2.powf(2.0)*l2*m2*delta.cos());

    result / (l2 * (2.0*m1 + m2 - m2*(2.0*delta).cos()))

}