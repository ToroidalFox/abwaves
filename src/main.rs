mod math;

use std::f32::consts as f32c;

use image::{Rgba, RgbaImage};
use noise::{utils::*, NoiseFn, Seedable};
use noise::{Fbm, Perlin};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;
use rayon::prelude::*;

fn main() {
    _angled_perlin_series_test();
}

fn _angled_perlin_series_test() {
    let line = AbLines {
        background_color: Rgba([223, 223, 223, 255]),
        abstract_lines: vec![(
            Box::new(PerlinSeries {
                origin: math::Vec2::new(900.0, 300.0),
                up: math::Dir2::new(-0.3, -1.0).unwrap(),
                series: vec![
                    PerlinWave::new(50.0, 500.0, 42),
                    PerlinWave::new(25.0, 250.0, 69),
                    PerlinWave::new(12.0, 120.0, 420),
                ],
            }),
            Rgba([127, 127, 127, 255]),
        )],
    };

    let (width, height) = (1920, 1080);

    let mut img_buffer = RgbaImage::new(width, height);

    img_buffer
        .par_enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| {
            let origin = math::Vec2::new(900.0, 300.0);
            let x = x as f32;
            let y = y as f32;
            if origin.distance((x, y)) <= 5.0 {
                *pixel = Rgba([255, 0, 0, 255]);
            } else {
                *pixel = line.color_of((x as f32, y as f32));
            }
        });

    let mut new_image = std::fs::File::create("perlin_series_test.png").unwrap();
    let _ = img_buffer.write_to(&mut new_image, image::ImageFormat::Png);
}

fn _angled_fourier_series_test() {
    let line = AbLines {
        background_color: Rgba([223, 223, 223, 255]),
        abstract_lines: vec![
            (
                Box::new(FourierSeries {
                    origin: math::Vec2::new(900.0, 600.0),
                    up: math::Dir2::new(0.2, -1.0).unwrap(),
                    series: vec![
                        SineWave::new(50.0, 300.0, 0.0),
                        SineWave::new(30.0, 240.0, 4.2),
                        SineWave::new(10.0, 150.0, 6.9),
                    ],
                }),
                Rgba([127, 63, 63, 255]),
            ),
            (
                Box::new(FourierSeries {
                    origin: math::Vec2::new(900.0, 200.0),
                    up: math::Dir2::new(0.4, -1.0).unwrap(),
                    series: vec![
                        SineWave::new(50.0, 300.0, 0.0),
                        SineWave::new(30.0, 240.0, 4.2),
                        SineWave::new(10.0, 150.0, 6.9),
                    ],
                }),
                Rgba([63, 127, 127, 255]),
            ),
        ],
    };
    let (width, height) = (1920, 1080);

    let mut img_buffer = RgbaImage::new(width, height);

    img_buffer
        .par_enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| {
            *pixel = line.color_of((x as f32, y as f32));
        });

    let mut new_image = std::fs::File::create("angled_fourier_series_test.png").unwrap();
    let _ = img_buffer.write_to(&mut new_image, image::ImageFormat::Png);
}

struct AbLines<T = image::Rgba<u8>> {
    background_color: T,
    abstract_lines: Vec<(Box<dyn AbstractLine>, T)>,
}

impl AbLines {
    pub fn color_of(&self, point: impl Into<math::Vec2>) -> Rgba<u8> {
        let point = point.into();
        self.abstract_lines
            .iter()
            .find(|(line, _)| line.height_of(point) <= 0.0)
            .map(|(_, pixel)| *pixel)
            .unwrap_or(self.background_color)
    }
}

struct PerlinSeries {
    origin: math::Vec2,
    up: math::Dir2,
    series: Vec<PerlinWave>,
}

struct PerlinWave {
    pub amplitude: f32,
    pub wavelength: f32,
    perlin: Perlin,
}

struct FourierSeries {
    origin: math::Vec2,
    up: math::Dir2,
    series: Vec<SineWave>,
}

struct SineWave {
    pub amplitude: f32,
    pub wavelength: f32,
    phase: f32,
}

trait AbstractLine: Sync {
    fn origin(&self) -> math::Vec2;
    fn up(&self) -> math::Dir2;

    fn cross(&self) -> math::Dir2 {
        let up = self.up();
        math::Dir2::new_unchecked((-up.y, -up.x))
    }
    fn elevation_fn(&self, t: f32) -> f32;
    /// Distance from _origin_ to projection of the point.
    fn t_of(&self, point: math::Vec2) -> f32 {
        (point - self.origin()).dot(self.cross())
    }
    /// Distance from _sea level_ to the point.
    fn altitude_of(&self, point: math::Vec2) -> f32 {
        (point - self.origin()).dot(self.up())
    }
    /// `altitude - elevation`
    fn height_of(&self, point: math::Vec2) -> f32 {
        self.altitude_of(point) - self.elevation_fn(self.t_of(point))
    }
}

impl AbstractLine for PerlinSeries {
    fn origin(&self) -> math::Vec2 {
        self.origin
    }

    fn up(&self) -> math::Dir2 {
        self.up
    }

    fn elevation_fn(&self, t: f32) -> f32 {
        self.series
            .iter()
            .map(|noise| noise.get([t as f64]))
            .sum::<f64>() as f32
    }
}

impl AbstractLine for FourierSeries {
    fn origin(&self) -> math::Vec2 {
        self.origin
    }

    fn up(&self) -> math::Dir2 {
        self.up
    }

    fn elevation_fn(&self, t: f32) -> f32 {
        self.series.iter().map(|sine_wave| sine_wave.value(t)).sum()
    }
}

impl PerlinWave {
    pub fn new(amplitude: f32, wavelength: f32, seed: u32) -> Self {
        Self {
            amplitude,
            wavelength,
            perlin: Perlin::new(seed),
        }
    }
}

impl NoiseFn<f64, 1> for PerlinWave {
    fn get(&self, mut point: [f64; 1]) -> f64 {
        point.iter_mut().for_each(|e| *e /= self.wavelength as f64);
        self.amplitude as f64 * self.perlin.get(point) as f64
    }
}

impl SineWave {
    pub fn new(amplitude: f32, wavelength: f32, phase: f32) -> Self {
        Self {
            amplitude,
            wavelength,
            phase: phase.rem_euclid(f32c::TAU),
        }
    }

    pub fn phase(&self) -> f32 {
        self.phase
    }

    pub fn value(&self, x: f32) -> f32 {
        let x = (x * f32c::TAU / self.wavelength).rem_euclid(f32c::TAU) + self.phase();
        self.amplitude * f32::sin(x)
    }
}

// fn _angled_primitive_pattern_test() {
//     let line = AbstractLine {
//         level: math::Level {
//             origin: math::Vec2::new(500.0, 500.0),
//             // up: math::Dir2::new((-1.0, 2.0)).unwrap_or(math::Dir2::UP),
//             up: math::Dir2::new((-1.0, -2.0)).unwrap(),
//         },
//         height_fn: SineWave::new(50.0, 300.0, 0.0),
//     };
//     let (width, height) = (1920, 1080);

//     let mut img_buffer = RgbImage::new(width, height);

//     img_buffer
//         .par_enumerate_pixels_mut()
//         .for_each(|(x, y, pixel)| {
//             if line.is_above((x as f32, y as f32)) {
//                 pixel.0 = [192, 192, 192];
//             } else {
//                 pixel.0 = [63, 63, 63];
//             }
//         });

//     let mut new_image = std::fs::File::create("angled_primitive_pattern_test.png").unwrap();
//     let _ = img_buffer.write_to(&mut new_image, image::ImageFormat::Png);
// }

// fn _primitive_image_test() {
//     let amplitude = 300.0; // pixels
//     let wavelength = 600.0; // pixels
//     let phase = 0.0 * std::f32::consts::TAU;
//     let axis_height = 540.0;

//     let (width, height) = (1920, 1080);

//     let mut img_buffer = RgbImage::new(width, height);

//     img_buffer
//         .par_enumerate_pixels_mut()
//         .for_each(|(x, y, pixel)| {
//             if (y as f32)
//                 < axis_height
//                     + amplitude * f32::sin(x as f32 / wavelength * std::f32::consts::TAU + phase)
//             {
//                 pixel.0 = [192, 192, 192];
//             } else {
//                 pixel.0 = [63, 63, 63];
//             }
//         });

//     let mut new_image = std::fs::File::create("primitive_pattern_test.png").unwrap();
//     let _ = img_buffer.write_to(&mut new_image, image::ImageFormat::Png);
// }

// fn _test_rng() {
//     let seeds = vec![69420, 0, 42, 69, 420];

//     for seed in seeds {
//         let mut rng = Xoshiro256PlusPlus::seed_from_u64(seed);
//         let f_rand = rng.gen::<f64>();
//         dbg!(f_rand);
//     }
// }
