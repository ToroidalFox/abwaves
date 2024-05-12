use derive_more::Div;
use image::RgbImage;
use noise::{utils::*, Seedable};
use noise::{Fbm, Perlin};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use rayon::prelude::*;

fn main() {
    primitive_image_test();
}

#[derive(Clone, Copy, Debug, Div)]
struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn length(self) -> f32 {
        f32::sqrt(self.dot(self))
    }

    pub fn dot(self, rhs: Self) -> f32 {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
}

// impl std::ops::Div<f32> for Vec2 {
//     type Output = Vec2;

//     fn div(self, rhs: f32) -> Self::Output {
//         Self {
//             x: self.x / rhs,
//             y: self.y / rhs,
//         }
//     }
// }

struct Dir2(Vec2);
impl Dir2 {
    pub fn new(from: Vec2) -> Result<Self, InvalidDirError> {
        let length = from.length();

        let is_finite = length.is_finite();
        let is_zero = length > 0.0;
        let is_nan = length.is_nan();

        match (is_zero, is_finite, is_nan) {
            (false, true, _) => Ok(from / length),
            (true, _, _) => Err(InvalidDirError::Zero),
            (_, _, true) => Err(InvalidDirError::NaN),
            (_, false, _) => Err(InvalidDirError::Infinite),
        }
        .map(Self)
    }
}

#[derive(Clone, Copy, Debug)]
enum InvalidDirError {
    Zero,
    Infinite,
    NaN,
}
impl InvalidDirError {
    pub fn from_length(length: f32) -> Self {
        if length.is_infinite() {
            Self::Infinite
        } else if length.is_nan() {
            Self::NaN
        } else {
            Self::Zero
        }
    }
}

fn primitive_image_test() {
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
    let result = img_buffer.write_to(&mut new_image, image::ImageFormat::Png);
    dbg!(result);
}

fn test_rng() {
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
