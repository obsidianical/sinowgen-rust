extern crate image;
extern crate opensimplex_noise_rs;

use std::time;
use time::SystemTime;

use opensimplex_noise_rs::OpenSimplexNoise;

use crate::biome::Biome;
use crate::fractal_noise::NoiseCfg;

mod fractal_noise;
mod noise_map_gen;
mod biome;
mod map;

fn main() {
    let gabriel_biome_map: Vec<Biome> = vec![
        Biome::new([10, 28, 38], [49, 87, 115], [0, 127]),
        Biome::new([129, 125, 120], [164, 158, 140], [127, 140]),
        Biome::new([50, 57, 18], [91, 93, 46], [140, 215]),
        Biome::new([105, 105, 105], [205, 200, 200], [215, 255]),
    ];

    let frac = fractal_noise::FractalNoise::new(get_now(), NoiseCfg {
        octaves: 8,
        lacunarity: 2.0,
        gain: 0.5,
    });
    let scale: f64 = 0.005;

    const WIDTH: u32 = 1080;
    const HEIGHT: u32 = 1080;

    type Map = [[u8; HEIGHT as usize]; WIDTH as usize];

    let mut image_buffer = image::ImageBuffer::new(WIDTH, HEIGHT);

    // for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
    //      let (xf, yf) = (x as f64, y as f64);

    //      let noise = frac.make_noise([scale * xf, scale * yf]);

    //      let height = ((noise + 1.0) / 2.0 * 255.0).floor() as u8;
    //      if noise > minmax.0 { minmax.0 = noise };
    //      if noise < minmax.1 { minmax.1 = noise };
    //      *pixel = image::Rgb(apply_biomes(height, &gabriel_biome_map));

    //     if x % 20 == 0 || y % 20 == 0 {
    //         *pixel = image::Rgb([50_u8,50_u8,50_u8]);
    //     } else {
    //         *pixel = image::Rgb([20_u8,20_u8,20_u8]);
    //     }
    // }

    image_buffer.save("fractal.png").unwrap();
}

fn warp_image() {}

fn apply_biomes(height: u8, biome_mappings: &Vec<Biome>) -> [u8; 3] {
    let mut result: [u8; 3] = [255, 0, 0];
    for biome in biome_mappings {
        if height >= biome.bounds[0] && height <= biome.bounds[1] {
            result = biome.get_color(height);
        }
    }
    result
}

fn get_now() -> u32 {
    match SystemTime::now().duration_since(time::UNIX_EPOCH) {
        Ok(n) => n.as_millis() as u32,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

