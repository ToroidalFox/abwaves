mod math;

use std::f32::consts as f32c;

use image::RgbImage;
use noise::{utils::*, Seedable};
use noise::{Fbm, Perlin};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use rayon::prelude::*;

fn main() {}

struct AbstractLine<T> {
    pub level: math::Level,
    height_fn: T,
}

impl<T> AbstractLine<T>
where
    T: math::HeightFn,
{
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
impl<T> math::HeightFn for &[T]
where
    T: math::HeightFn,
{
    fn height(&self, at: f32) -> f32 {
        self.iter().map(|h| h.height(at)).sum()
    }
}

fn _angled_primitive_pattern_test() {}

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
