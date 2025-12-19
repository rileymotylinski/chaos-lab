#[cfg(test)]


mod tests {
    use crate::{integrators::{euler_step, rk4_step}, lyapunov, math::Vec2};

    

    #[test]
    fn test_vec2_add() {
        let v1 = Vec2::new(1.0,2.0);
        let v2 = Vec2::new(1.0,2.0);

        assert_eq!(Vec2::new(2.0,4.0),v1+v2);
    }

    #[test]
    fn test_vec2_sub() {
        let v1 = Vec2::new(1.0,2.0);
        let v2 = Vec2::new(1.0,2.0);

        assert_eq!(Vec2::new(0.0,0.0),v1-v2);
    }

    #[test]
    fn test_vec2_mul() {
        let v1 = Vec2::new(1.0,2.0);
        

        assert_eq!(Vec2::new(2.0,4.0),v1*2.0);
    }

    #[test]
    fn test_vec2_dot() {
        let v1 = Vec2::new(1.0,2.0);
        let v2 = Vec2::new(1.0,2.0);

        assert_eq!(5.0,v1.dot(&v2));
    }

    #[test]
    fn test_vec2_norm() {
        let v1 = Vec2::new(1.0,2.0);

        assert_eq!((5.0 as f64).sqrt(),v1.norm());
    }

    #[test]
    fn test_euler_step() {
        let mut state = [1.0]; // inital x (vertical axis) value
        let t= 0.0; // initial t (horizontal axis) value 
        let dt = 0.1; // t increment

        
        let f = |t: f64, _s: &[f64]| -> Vec<f64> {
            vec![-t]
        };

        euler_step(&mut state, t, dt, f);

        assert_eq!(1.0,state[0]);
    }

    #[test]
    fn test_integrators_rk4() {
        
        let mut state = [1.0]; // inital x (vertical axis) value
        let t= 0.0; // initial t (horizontal axis) value  
        let dt = 0.1; // t increment

        
        // ugly expression, but should step along 
        // x' = -t
        // integrating, we get x = -0.5(t^2)
        // state length is one because we only depend on one thing: initial position along the vertical axis
        let f = |t: f64, _s: &[f64]| -> Vec<f64> {
            vec![-t]
        };


        rk4_step(&mut state,t,dt,f);

        assert_eq!(0.995,state[0]);
    }

    #[test]
    fn test_lyapunov_unstable() {
        
        // logistic map r
        let r = 4.0;

        let num_iterations: i64 = 100000;
        let f = |t: f64| -> f64 {
            r*t*(1.0-t)
        };

        // logistic map is only defined on [0,1]
        let result = crate::lyapunov::lyapunov(0.9,num_iterations, f);
        assert!(result - 0.693 < 0.001);
    }

    #[test]
    fn test_lyapunov_stable() {
        let num_iterations: i64 = 100000;
        let f = |x: f64| 0.5 * x;

        let result = crate::lyapunov::lyapunov(0.9,num_iterations, f);
        assert!(result < 0.0);
    }

}
