use std::f64;
use opensimplex_noise_rs::OpenSimplexNoise;

pub struct FractalNoise {
    seed: u32,
    noise_cfg: NoiseCfg,
    noise2d: OpenSimplexNoise,
}

impl FractalNoise {
    pub(crate) fn new(seed: u32, noise_cfg: NoiseCfg) -> FractalNoise {
        FractalNoise {
            seed,
            noise_cfg,
            //noise2d: OpenSimplex::new().set_seed(seed),
            noise2d: OpenSimplexNoise::new(Option::from(seed as i64))
        }
    }

    pub(crate) fn make_noise(&self ,pos: [f64; 2]) -> f64 {
        let mut y: f64 = 0.0;

        let mut amplitude = 0.75;
        let mut frequency = 1.0;
        for i in 0..self.noise_cfg.octaves {
            // let factor: f64 = f64::powi(2.0, i as i32);
            // let transformed_pos = [pos[0] * factor, pos[1] * factor];
            // r += self.noise2d.get(transformed_pos).clamp(-0.5, 0.5) / factor;

            // y += amplitude * self.noise2d.get(pos).clamp(-0.5, 0.5);
            y += amplitude * self.noise2d.eval_2d(pos[0] * frequency, pos[1] * frequency);
            frequency *= self.noise_cfg.lacunarity;
            amplitude *= self.noise_cfg.gain;
        }
        y
    }
}

pub struct NoiseCfg {
    pub(crate) octaves: u8,
    pub(crate) lacunarity: f64,
    pub(crate) gain: f64,
}

impl Copy for NoiseCfg {}

impl Clone for NoiseCfg {
    fn clone(&self) -> Self {
        *self
    }
}