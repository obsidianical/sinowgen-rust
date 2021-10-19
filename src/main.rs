mod fractal_noise;
mod noise_map_gen;
mod biome;

extern crate image;
extern crate noise;

use std::time;
use time::SystemTime;
use crate::biome::Biome;
use noise::{NoiseFn, OpenSimplex, Seedable};

fn main() {
    let gabriel_biome_map: Vec<Biome>= vec![
        Biome::new([10, 28, 38], [49, 87, 115], [0, 127]),
        Biome::new([129, 125, 120], [164, 158, 140], [127, 140]),
        Biome::new([50, 57, 18], [91, 93, 46], [140, 215]),
        Biome::new([105, 105, 105], [205, 200, 200], [215, 255])
    ];

    //let frac = fractal_noise::FractalNoise::new(get_now(), 8);
    let frac = OpenSimplex::new().set_seed(get_now());
    let scale : f64= 0.005;

    let width: u32 = 1080;
    let height: u32 = 1080;

    let mut image_buffer = image::ImageBuffer::new(width, height);

    let mut minmax: (f64, f64) = (-1.0, 1.0);

    for (x, y, pixel) in image_buffer.enumerate_pixels_mut() {
        let (xf, yf) = (x as f64, y as f64);
        let noise = frac.get([scale * xf, scale * yf]);
        let height = ((noise + 1.0) / 2.0 * 255.0).floor() as u8;
        if noise > minmax.0 { minmax.0 = noise };
        if noise < minmax.1 { minmax.1 = noise };
        *pixel = image::Rgb(apply_biomes(height, &gabriel_biome_map));
    }
    println!("minmax: {:?}", minmax);

    image_buffer.save("fractal.png").unwrap();
}

fn apply_biomes(height: u8, biome_mappings: &Vec<Biome>) -> [u8; 3] {
    let mut result: [u8;3] = [255, 0, 0];
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
