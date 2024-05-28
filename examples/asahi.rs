use abwaves::{linear_color_interp, prelude::*};

use image::{Rgba, RgbaImage};
use rand::{Rng, SeedableRng};
use rand_distr::Distribution;
use rayon::prelude::*;

fn main() {
    let mut rng = rand_xoshiro::Xoshiro256PlusPlus::seed_from_u64(45411); // seed: 454h1 l1nux
    let gaussian_dist = rand_distr::Normal::new(0.1, 0.02).unwrap();

    let color_start = Rgba([0x23, 0x23, 0x3d, 255]);
    // let color_end = Rgba([0x50, 0x65, 0x80, 255]);
    let color_end = Rgba([0x60, 0x70, 0x80, 255]);

    let div = 6;
    let amp = 1.0 / div as f32 / 6.0;
    let wavelength = 1.0 / 2.6;

    let abwaves = AbWaves::new(
        vec![
            Color::Solid(color_start),
            Color::Solid(linear_color_interp(&color_start, &color_end, 2.0 / 6.0)),
            Color::Solid(linear_color_interp(&color_start, &color_end, 3.0 / 6.0)),
            Color::Solid(linear_color_interp(&color_start, &color_end, 4.0 / 6.0)),
            Color::Solid(linear_color_interp(&color_start, &color_end, 5.0 / 6.0)),
            Color::Solid(color_end),
        ],
        vec![
            Wave {
                origin: Vec2::new(0.5, 5.0 / div as f32),
                up: Dir2::new(gaussian_dist.sample(&mut rng), -1.0).unwrap(),
                series: vec![
                    HeightFn::new_perlin(amp, wavelength, rng.gen(), rng.gen()),
                    HeightFn::new_perlin(amp / 2.0, wavelength / 2.0, rng.gen(), rng.gen()),
                    HeightFn::new_perlin(amp / 4.0, wavelength / 4.0, rng.gen(), rng.gen()),
                    // HeightFn::new_perlin(amp / 8.0, wavelength / 8.0, rng.gen(), rng.gen()),
                ],
            },
            Wave {
                origin: Vec2::new(0.5, 4.0 / div as f32),
                up: Dir2::new(-gaussian_dist.sample(&mut rng), -1.0).unwrap(),
                series: vec![
                    HeightFn::new_perlin(amp, wavelength, rng.gen(), rng.gen()),
                    HeightFn::new_perlin(amp / 2.0, wavelength / 2.0, rng.gen(), rng.gen()),
                    HeightFn::new_perlin(amp / 4.0, wavelength / 4.0, rng.gen(), rng.gen()),
                ],
            },
            Wave {
                origin: Vec2::new(0.5, 3.0 / div as f32),
                up: Dir2::new(gaussian_dist.sample(&mut rng), -1.0).unwrap(),
                series: vec![
                    HeightFn::new_perlin(amp, wavelength, rng.gen(), rng.gen()),
                    HeightFn::new_perlin(amp / 2.0, wavelength / 2.0, rng.gen(), rng.gen()),
                    HeightFn::new_perlin(amp / 4.0, wavelength / 4.0, rng.gen(), rng.gen()),
                ],
            },
            Wave {
                origin: Vec2::new(0.5, 2.0 / div as f32),
                up: Dir2::new(-gaussian_dist.sample(&mut rng), -1.0).unwrap(),
                series: vec![
                    HeightFn::new_perlin(amp, wavelength, rng.gen(), rng.gen()),
                    HeightFn::new_perlin(amp / 2.0, wavelength / 2.0, rng.gen(), rng.gen()),
                    HeightFn::new_perlin(amp / 4.0, wavelength / 4.0, rng.gen(), rng.gen()),
                ],
            },
            Wave {
                origin: Vec2::new(0.5, 1.0 / div as f32),
                up: Dir2::new(gaussian_dist.sample(&mut rng), -1.0).unwrap(),
                series: vec![
                    HeightFn::new_perlin(amp, wavelength, rng.gen(), rng.gen()),
                    HeightFn::new_perlin(amp / 2.0, wavelength / 2.0, rng.gen(), rng.gen()),
                    HeightFn::new_perlin(amp / 4.0, wavelength / 4.0, rng.gen(), rng.gen()),
                ],
            },
        ],
    );

    // body's aspect ratio
    // let (width, height) = (3041, 2150);
    let (width, height) = (304, 215);
    let factor = 4;
    let (width, height) = (width * factor, height * factor);
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
