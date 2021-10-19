use crate::biome::Biome;
use crate::fractal_noise::FractalNoise;

pub struct NoiseMapGen {
    mountain_generator: FractalNoise,
    detail_generator: FractalNoise,
    mountain_cfg: GeneratorCfg,
    detail_cfg: GeneratorCfg,
    size: [i32; 2],
    map: Vec<Vec<u8>>
}

impl NoiseMapGen {
    pub(crate) fn new(mountain_cfg: GeneratorCfg, detail_cfg: GeneratorCfg, size: [i32; 2]) -> NoiseMapGen {
        NoiseMapGen {
            mountain_generator: FractalNoise::new(mountain_cfg.seed, mountain_cfg.octaves),
            detail_generator: FractalNoise::new(detail_cfg.seed, detail_cfg.octaves),
            mountain_cfg,
            detail_cfg,
            size,
            map: vec![]
        }
    }

    pub(crate) fn make_color_map(&mut self, biome_mappings: Vec<Biome>) {
        for x in 0..self.size[0] {
            self.map[x as usize] = vec![];
            for y in 0..self.size[1] {
                let (xf, yf) = (x as f64, y as f64);
                // let height = ((frac.make_noise([scale * xf, scale * yf]) + 1.0) * 127.5
                //     ).floor() as u8;
            }
        }
    }
}

pub struct GeneratorCfg {
    seed: u32,
    scale: f64,
    relevance: f64,
    octaves: u8,
}