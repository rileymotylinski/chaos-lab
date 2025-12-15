#[cfg(test)]

use crate::math::Vec2;
use crate::integrators::{euler_step, rk4_step};

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
    let f = |x: f64, s: &[f64]| {s.iter().map(|t| {x-t}).collect()};

    euler_step(&mut state, t, dt, f);
}

#[test]
fn test_integrators_rk4() {
    
    let mut state = [1.0]; // inital x (vertical axis) value
    let t= 0.0; // initial t (horizontal axis) value  
    let dt = 0.1; // t increment

    
    // ugly expression, but should step along 
    // x' = -t
    // -> x = -0.5(t^2)
    let f = |x: f64, s: &[f64]| {s.iter().map(|t| {x-t}).collect()};

    rk4_step(&mut state,t,dt,f);

    assert_eq!(0.0,state[0]);
}
