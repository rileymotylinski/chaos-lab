use std::error::Error;

pub fn lorenz() -> Result<(), Box<dyn Error>> {


    let sigma = 10.0;
    let ro = 28.0;
    let beta = 8.0/3.0;
    let output_file = "./lorenz.csv";
    
    // implementing lorenz system derivatives
    // if s = [x,y,z]
    let f = |_t: f64, s: &[f64]| -> Vec<f64> {
        vec![
            sigma*(s[1]-s[0]),
            s[0]*(ro-s[2])-s[1],
            (s[0]*s[1]) - (beta*s[2])
        ]
    };

    let mut state = [1.0,1.0,1.0]; // inital x (vertical axis) value
    let t= 0.0; // initial t (horizontal axis) value  
    let dt = 0.1; // t increment

    // create csv writer
    let mut wtr = csv::Writer::from_path(output_file)?;
    // write headers
    let headers = vec!["x","y","z"];
    wtr.write_record(&headers)?;

    
    let num_steps = 500;
    for _i in 0..num_steps {
        let temp = state.iter().map(|x|-> String {x.to_string()}).collect::<Vec<String>>();
        // write state
        wtr.write_record(&temp)?;
        // update state
        crate::integrators::rk4_step(&mut state,t,dt,f);

    }

    

    wtr.flush()?;
    Ok(())


}
