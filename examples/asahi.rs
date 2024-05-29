use abwaves::prelude::*;

use image::{Rgba, RgbaImage};
use rand::{Rng, SeedableRng};
use rand_distr::Distribution;
use rayon::prelude::*;

fn main() {
    let seed = 42;
    let mut rng = rand_xoshiro::Xoshiro256PlusPlus::seed_from_u64(seed);

    let color_start = Rgba([0x30, 0x36, 0x50, 255]);
    let color_end = Rgba([0x90, 0xa0, 0xb0, 255]);

    let padding = -1.0 / 10.0;
    // let padding = 0.0;
    let size = 1.0 - 2.0 * padding;
    let div = 6;
    let amp = 1.0 / div as f32 / 3.0;
    let wavlen = |rng: &mut rand_xoshiro::Xoshiro256PlusPlus| {
        let low = 0.30;
        let high = 0.50;
        let dist = rand_distr::Uniform::new(low, high);
        dist.sample(rng)
    };

    let abwaves = AbWaves::new(
        vec![
            Color::Solid(color_start),
            Color::Solid(linear_color_interp(&color_start, &color_end, 1.0 / 5.0)),
            Color::Solid(linear_color_interp(&color_start, &color_end, 2.0 / 5.0)),
            Color::Solid(linear_color_interp(&color_start, &color_end, 3.0 / 5.0)),
            Color::Solid(linear_color_interp(&color_start, &color_end, 4.0 / 5.0)),
            Color::Solid(color_end),
        ],
        vec![
            Wave {
                origin: Vec2::new(0.5, padding + size * 5.0 / div as f32),
                up: Dir2::new(0.0, -1.0).unwrap(),
                series: vec![
                    HeightFn::new_perlin(amp, wavlen(&mut rng), rng.gen(), rng.gen()),
                    HeightFn::new_perlin(amp / 4.0, wavlen(&mut rng) / 2.0, rng.gen(), rng.gen()),
                    HeightFn::new_perlin(amp / 8.0, wavlen(&mut rng) / 4.0, rng.gen(), rng.gen()),
                ],
            },
            Wave {
                origin: Vec2::new(0.5, padding + size * 4.0 / div as f32),
                up: Dir2::new(0.0, -1.0).unwrap(),
                series: vec![
                    HeightFn::new_perlin(amp, wavlen(&mut rng), rng.gen(), rng.gen()),
                    HeightFn::new_perlin(amp / 4.0, wavlen(&mut rng) / 2.0, rng.gen(), rng.gen()),
                    HeightFn::new_perlin(amp / 8.0, wavlen(&mut rng) / 4.0, rng.gen(), rng.gen()),
                ],
            },
            Wave {
                origin: Vec2::new(0.5, padding + size * 3.0 / div as f32),
                up: Dir2::new(0.0, -1.0).unwrap(),
                series: vec![
                    HeightFn::new_perlin(amp, wavlen(&mut rng), rng.gen(), rng.gen()),
                    HeightFn::new_perlin(amp / 4.0, wavlen(&mut rng) / 2.0, rng.gen(), rng.gen()),
                    HeightFn::new_perlin(amp / 8.0, wavlen(&mut rng) / 4.0, rng.gen(), rng.gen()),
                ],
            },
            Wave {
                origin: Vec2::new(0.5, padding + size * 2.0 / div as f32),
                up: Dir2::new(0.0, -1.0).unwrap(),
                series: vec![
                    HeightFn::new_perlin(amp, wavlen(&mut rng), rng.gen(), rng.gen()),
                    HeightFn::new_perlin(amp / 4.0, wavlen(&mut rng) / 2.0, rng.gen(), rng.gen()),
                    HeightFn::new_perlin(amp / 8.0, wavlen(&mut rng) / 4.0, rng.gen(), rng.gen()),
                ],
            },
            Wave {
                origin: Vec2::new(0.5, padding + size * 1.0 / div as f32),
                up: Dir2::new(0.0, -1.0).unwrap(),
                series: vec![
                    HeightFn::new_perlin(amp, wavlen(&mut rng), rng.gen(), rng.gen()),
                    HeightFn::new_perlin(amp / 4.0, wavlen(&mut rng) / 2.0, rng.gen(), rng.gen()),
                    HeightFn::new_perlin(amp / 8.0, wavlen(&mut rng) / 4.0, rng.gen(), rng.gen()),
                ],
            },
        ],
    );

    // let (width, height) = (3041, 2150); // apple macbook air dimension
    let (width, height) = (16, 10);
    let factor = 160 * 9; // 160 for "1440p"
    let (width, height) = (width * factor, height * factor);

    let mut img_buffer = RgbaImage::new(width, height);

    img_buffer
        .par_enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| {
            let x = (x as f32 + 0.5) / width as f32;
            let y = (y as f32 + 0.5) / height as f32;

            *pixel = abwaves.color_of((x, y));
        });

    let mut new_image =
        std::fs::File::create(format!("asahi_{}_{}x{}", seed, width, height)).unwrap();
    let _ = img_buffer.write_to(&mut new_image, image::ImageFormat::Png);
}
