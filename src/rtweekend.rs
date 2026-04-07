use core::f64;
//use rand::prelude::*;

// Constants
pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = f64::consts::PI;

// Utility functions
pub fn degrees_to_radians(degrees: f64) {
    degrees * PI / 180.0;
}

// pub fn random_double() -> f64 {
// returns a random real in <0,1)
// rand::rng().random()
// }

// pub fn random_double_range(min: f64, max: f64) -> f64 {
// returns a random real in <min,max)
// min + (max - min) * random_double()
// }
