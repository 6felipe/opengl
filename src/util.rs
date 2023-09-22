use rand_xoshiro::Xoshiro128Plus;
use rand::prelude::*;

pub fn random_triangle() -> Vec<f32> {
    let mut xoshiro_rng = Xoshiro128Plus::from_entropy();
    
    vec![
        xoshiro_rng.gen_range(-1.0..1.0),
        xoshiro_rng.gen_range(-1.0..1.0),
        0.0,
        xoshiro_rng.gen_range(-1.0..1.0),
        xoshiro_rng.gen_range(-1.0..1.0),
        0.0,
        xoshiro_rng.gen_range(-1.0..1.0),
        xoshiro_rng.gen_range(-1.0..1.0),
        0.0,
    ]
}
