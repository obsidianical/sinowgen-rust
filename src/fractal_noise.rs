use noise::{NoiseFn, OpenSimplex, Seedable};
use std::f64;

pub struct FractalNoise {
    seed: u32,
    octaves: u8,
    noise2d: OpenSimplex
}

impl FractalNoise {
    pub(crate) fn new(seed: u32, octaves: u8) -> FractalNoise {
        FractalNoise {
            seed: seed,
            octaves: octaves,
            noise2d: OpenSimplex::new().set_seed(seed)
        }
    }

    pub(crate) fn make_noise(&self ,pos: [f64; 2]) -> f64 {
        let mut r: f64 = 0.0;

        for i in 0..self.octaves {
            let i_f = i as f64;
            let factor: f64 = f64::powf(2.0, i_f);
            let transformed_pos = [pos[0] * factor, pos[1] * factor];
            r += self.noise2d.get(transformed_pos) / factor;
        }

        r
    }
}