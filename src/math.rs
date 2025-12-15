#[derive(Debug, Clone, Copy)]
pub struct Vec2 { pub x: f64, pub y: f64 } // creating Vec2 struct

impl Vec2 { 
    pub fn new(x: f64, y:f64) -> Self { Vec2 { x,y }} // Self is the current types
    pub fn dot(&self, v2: &Vec2) -> f64 {(self.x * v2.x) + (self.y * v2.y)} // reference self 
    pub fn norm(&self) -> f64 {((self.x * self.x) + (self.y * self.y)).sqrt()}
}

use std::ops::{Add, Sub, Mul};
use std::cmp::PartialEq;

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec2::new(self.x*rhs, self.y*rhs)
    }
}


impl PartialEq for Vec2 {
    fn eq(&self, other: &Vec2) -> bool {
        let e = std::f64::EPSILON;
        (self.x - other.x).abs() < e && (self.y - other.y).abs() < e
    }
}
