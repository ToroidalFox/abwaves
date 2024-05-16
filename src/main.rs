mod math;

use std::f32::consts as f32c;

use image::RgbImage;
use noise::{utils::*, Seedable};
use noise::{Fbm, Perlin};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use rayon::prelude::*;

fn main() {
    _angled_fourier_series_test();
}

fn _angled_fourier_series_test() {
    let line = AbstractLine {
        level: math::Level {
            origin: math::Vec2::new(500.0, 500.0),
            // up: math::Dir2::new((-1.0, 2.0)).unwrap_or(math::Dir2::UP),
            up: math::Dir2::new((1.0, -2.0)).unwrap(),
        },
        height_fn: vec![
            SineWave::new(32.0, 512.0, 0.0),
            SineWave::new(16.0, 256.0, 0.0),
            SineWave::new(8.0, 128.0, 0.0),
            SineWave::new(4.0, 64.0, 0.0),
            SineWave::new(2.0, 32.0, 0.0),
        ],
    };
    let (width, height) = (1920, 1080);

    let mut img_buffer = RgbImage::new(width, height);

    img_buffer
        .par_enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| {
            if line.is_above((x as f32, y as f32)) {
                pixel.0 = [192, 192, 192];
            } else {
                pixel.0 = [63, 63, 63];
            }
        });

    let mut new_image = std::fs::File::create("angled_fourier_series_test.png").unwrap();
    let _ = img_buffer.write_to(&mut new_image, image::ImageFormat::Png);
}

fn _angled_primitive_pattern_test() {
    let line = AbstractLine {
        level: math::Level {
            origin: math::Vec2::new(500.0, 500.0),
            // up: math::Dir2::new((-1.0, 2.0)).unwrap_or(math::Dir2::UP),
            up: math::Dir2::new((-1.0, -2.0)).unwrap(),
        },
        height_fn: SineWave::new(50.0, 300.0, 0.0),
    };
    let (width, height) = (1920, 1080);

    let mut img_buffer = RgbImage::new(width, height);

    img_buffer
        .par_enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| {
            if line.is_above((x as f32, y as f32)) {
                pixel.0 = [192, 192, 192];
            } else {
                pixel.0 = [63, 63, 63];
            }
        });

    let mut new_image = std::fs::File::create("angled_primitive_pattern_test.png").unwrap();
    let _ = img_buffer.write_to(&mut new_image, image::ImageFormat::Png);
}

struct AbstractLine<T: math::HeightFn> {
    pub level: math::Level,
    pub height_fn: T,
}

impl<T: math::HeightFn> AbstractLine<T> {
    pub fn height(&self, from: impl Into<math::Vec2>) -> f32 {
        let from = from.into();
        self.level.distance_from_point(from)
            + self
                .height_fn
                .height(self.level.distance_to_projection(from))
    }
    pub fn is_above(&self, point: impl Into<math::Vec2>) -> bool {
        self.height(point) > 0.0
    }
}

struct SineWave {
    pub amplitude: f32,
    pub wavelength: f32,
    phase: f32,
}
impl SineWave {
    pub fn new(amplitude: f32, wavelength: f32, phase: f32) -> Self {
        Self {
            amplitude,
            wavelength,
            phase: phase.rem_euclid(f32c::TAU),
        }
    }

    fn phase(&self) -> f32 {
        self.phase
    }
}

impl math::HeightFn for SineWave {
    fn height(&self, at: f32) -> f32 {
        let t = (at / self.wavelength * f32c::TAU).rem_euclid(f32c::TAU) + self.phase();
        self.amplitude * f32::sin(t)
    }
}
impl<T: math::HeightFn, S: std::ops::Deref<Target = [T]>> math::HeightFn for S {
    fn height(&self, at: f32) -> f32 {
        self.as_ref().iter().map(|h| h.height(at)).sum()
    }
}

fn _primitive_image_test() {
    let amplitude = 300.0; // pixels
    let wavelength = 600.0; // pixels
    let phase = 0.0 * std::f32::consts::TAU;
    let axis_height = 540.0;

    let (width, height) = (1920, 1080);

    let mut img_buffer = RgbImage::new(width, height);

    img_buffer
        .par_enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| {
            if (y as f32)
                < axis_height
                    + amplitude * f32::sin(x as f32 / wavelength * std::f32::consts::TAU + phase)
            {
                pixel.0 = [192, 192, 192];
            } else {
                pixel.0 = [63, 63, 63];
            }
        });

    let mut new_image = std::fs::File::create("primitive_pattern_test.png").unwrap();
    let _ = img_buffer.write_to(&mut new_image, image::ImageFormat::Png);
}

fn _test_rng() {
    let seeds = vec![69420, 0, 42, 69, 420];

    for seed in seeds {
        let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
        let f_rand = rng.gen::<f64>();
        dbg!(f_rand);
    }
}

fn _proto_fbm_example() {
    let fbm = Fbm::<Perlin>::new(69420);

    let width = 1920u32;
    let height = 1920u32;

    let mut new_image = RgbImage::new(width, height);

    // lets test with image with simple sine wave.

    PlaneMapBuilder::new(fbm)
        .set_size(1000, 1000)
        .set_x_bounds(-5.0, 5.0)
        .set_y_bounds(-5.0, 5.0)
        .build()
        .write_to_file(std::path::Path::new("fbm_perlin_test.png"));
}
