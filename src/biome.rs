pub struct Biome {
    color_high_rel: Color,
    color_low: Color,
    biome_height: f64,
    pub bounds: [u8; 2],
}

impl Biome {
    pub(crate) fn new(color_low: Color, color_high: Color, bounds: [u8; 2]) -> Biome {
        Biome {
            color_high_rel: [
                color_high[0] - color_low[0],
                color_high[1] - color_low[1],
                color_high[2] - color_low[2]
            ],
            color_low,
            biome_height: if bounds[0] > bounds[1] { bounds[0] - bounds[1] } else { bounds[1] - bounds[0] } as f64,
            bounds,
        }
    }

    pub(crate) fn get_color(&self, height: u8) -> Color {
        let rel_height: f64 = (height - self.bounds[0]) as f64 / self.biome_height;
        [
            f64::floor(self.color_low[0] as f64 + (self.color_high_rel[0] as f64 * rel_height)) as u8,
            f64::floor(self.color_low[1] as f64 + (self.color_high_rel[1] as f64 * rel_height)) as u8,
            f64::floor(self.color_low[2] as f64 + (self.color_high_rel[2] as f64 * rel_height)) as u8,
        ]
    }
}

type Color = [u8; 3];
