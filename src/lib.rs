pub mod math;
pub mod wave;

use image::{Pixel, Rgba};

pub struct AbWaves {
    colors: Vec<Rgba<u8>>,
    waves: Vec<wave::Wave>,
}

impl AbWaves {
    pub fn new(colors: Vec<Rgba<u8>>, waves: Vec<wave::Wave>) -> Self {
        assert!(colors.len() == waves.len() + 1);
        Self { colors, waves }
    }
    pub fn color_of(&self, point: impl Into<math::Vec2>) -> Rgba<u8> {
        let point = point.into();
        let index = self
            .waves
            .iter()
            .position(|wave| wave.height_at(point) <= 0.0)
            .unwrap_or(self.colors.len() - 1);

        self.colors[index]
    }
}

pub enum Color {
    Solid(Rgba<u8>),
    GradientByHeight(Rgba<u8>, Rgba<u8>),
}

impl Color {
    pub fn color_at(&self, height: f32) -> Rgba<u8> {
        match self {
            Self::Solid(color) => *color,
            Self::GradientByHeight(start_color, end_color) => {
                gamma_corrected_color_interpolation(start_color, end_color, height)
            }
        }
    }
}

fn gamma_corrected_color_interpolation(a: &Rgba<u8>, b: &Rgba<u8>, t: f32) -> Rgba<u8> {
    const GAMMA: f32 = 2.2;

    let u8_as_f32 = |val: &u8| *val as f32 / u8::MAX as f32;
    let pixel_inverse_gamma_correction = |val: &u8| u8_as_f32(val).powf(GAMMA);
    let pixel_gamma_correction = |val: &f32| (val * u8::MAX as f32) as u8;

    let a = a.0;
    let b = b.0;

    let mut a_f32 = [0.0_f32; 4];
    let mut b_f32 = [0.0_f32; 4];

    for i in 0..a.len() - 1 {
        a_f32[i] = pixel_inverse_gamma_correction(&a[i]);
        b_f32[i] = pixel_inverse_gamma_correction(&b[i]);
    }
    a_f32[3] = u8_as_f32(&a[3]);
    b_f32[3] = u8_as_f32(&b[3]);

    todo!();
}
