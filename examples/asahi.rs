use abwaves::prelude::*;

use std::f32::consts as f32c;

use image::{Rgba, RgbaImage};
use rayon::prelude::*;

fn main() {
    // let seed = rand_xoshiro::Xoshiro256PlusPlus;
    let abwaves = AbWaves::new(
        vec![
            // Rgba([68, 76, 76, 255]),    // "mountain"
            // Rgba([217, 116, 62, 255]),  // "sunset red"
            // Rgba([226, 205, 150, 255]), // "sunset yello"
            // Rgba([107, 114, 125, 255]), // "sky blue"
            // Rgba([254, 238, 212, 255]), // "glowing cloud"
            // new color palette
            Color::Solid(Rgba([76, 76, 68, 255])), // "mountain"
            Color::Solid(Rgba([130, 92, 67, 255])), // "1/4"
            Color::Solid(Rgba([167, 105, 66, 255])), // "2/4"
            Color::Solid(Rgba([198, 117, 65, 255])), // "3/4"
            Color::Solid(Rgba([224, 128, 64, 255])), // "sunset orange"
        ],
        vec![
            Wave {
                origin: Vec2::new(0.5, 0.9),
                up: Dir2::new(0.0, -1.0).unwrap(),
                series: vec![
                    HeightFn::new_sine(0.1, 1.4, 3.0 * f32c::PI / 2.0),
                    HeightFn::new_perlin(0.06, 0.1, 0741), // seed: m0un741n
                ],
            },
            Wave {
                origin: Vec2::new(0.5, 0.8),
                up: Dir2::new(0.0, -1.0).unwrap(),
                series: vec![
                    HeightFn::new_sine(0.1, 1.4, 3.0 * f32c::PI / 2.0),
                    HeightFn::new_perlin(0.05, 0.1, 0741053), // seed: m0un741n cl053
                ],
            },
            Wave {
                origin: Vec2::new(0.5, 0.7),
                up: Dir2::new(0.0, -1.0).unwrap(),
                series: vec![
                    HeightFn::new_sine(0.1, 1.4, 3.0 * f32c::PI / 2.0),
                    HeightFn::new_perlin(0.05, 0.1, 074113), // seed: m0un741n m1ddl3
                ],
            },
            Wave {
                origin: Vec2::new(0.5, 0.6),
                up: Dir2::new(0.0, -1.0).unwrap(),
                series: vec![
                    HeightFn::new_sine(0.1, 1.4, 3.0 * f32c::PI / 2.0),
                    HeightFn::new_perlin(0.05, 0.1, 07414), // seed: m0un741n f4r
                ],
            },
        ],
    );

    let (width, height) = (3041, 2150);
    // let (width, height) = (6082 * 2, 4300 * 2);

    let mut img_buffer = RgbaImage::new(width, height);

    img_buffer
        .par_enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| {
            let x = (x as f32 + 0.5) / width as f32;
            let y = (y as f32 + 0.5) / height as f32;

            *pixel = abwaves.color_of((x, y));
        });

    let mut new_image = std::fs::File::create("asahi.png").unwrap();
    let _ = img_buffer.write_to(&mut new_image, image::ImageFormat::Png);
}
