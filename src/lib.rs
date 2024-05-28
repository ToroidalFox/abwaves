pub mod math;
pub mod prelude;
pub mod wave;

use image::{Pixel, Rgba};

pub struct AbWaves {
    colors: Vec<Color>,
    waves: Vec<wave::Wave>,
}

impl AbWaves {
    pub fn new(colors: Vec<Color>, waves: Vec<wave::Wave>) -> Self {
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

        if index == 0 {
            self.colors[index].color_at(-self.waves[0].height_at(point))
        } else {
            self.colors[index].color_at(self.waves[index - 1].height_at(point))
        }
    }
}

pub enum Color {
    Solid(Rgba<u8>),
    GradientByHeight(Rgba<u8>, Rgba<u8>, f32),
}

impl Color {
    pub fn color_at(&self, height: f32) -> Rgba<u8> {
        match self {
            Self::Solid(color) => *color,
            Self::GradientByHeight(start_color, end_color, height_max) => {
                gamma_corrected_color_interpolation(start_color, end_color, height / height_max)
            }
        }
    }
}

fn gamma_corrected_color_interpolation(a: &Rgba<u8>, b: &Rgba<u8>, t: f32) -> Rgba<u8> {
    const GAMMA: f32 = 2.2;

    let u8_to_f32 = |pixel: &[u8; 4]| {
        let mut f32_pixel = [0.0f32; 4];
        pixel
            .iter()
            .enumerate()
            .for_each(|(i, ch)| f32_pixel[i] = *ch as f32 / u8::MAX as f32);
        f32_pixel
    };
    let f32_to_u8 = |pixel: &[f32; 4]| {
        let mut u8_pixel = [0u8; 4];
        pixel
            .iter()
            .enumerate()
            .for_each(|(i, ch)| u8_pixel[i] = (*ch * u8::MAX as f32) as u8);
        u8_pixel
    };

    let pixel_inverse_gamma_correction = |pixel: &mut [f32; 4]| {
        pixel.iter_mut().take(3).for_each(|ch| *ch = ch.powf(GAMMA));
    };
    let pixel_gamma_correction = |pixel: &mut [f32; 4]| {
        pixel
            .iter_mut()
            .take(3)
            .for_each(|ch| *ch = ch.powf(1.0 / GAMMA));
    };

    let t = t.clamp(0.0, 1.0);
    let mut a = u8_to_f32(&a.0);
    let mut b = u8_to_f32(&b.0);
    pixel_inverse_gamma_correction(&mut a);
    pixel_inverse_gamma_correction(&mut b);

    a.iter_mut().for_each(|ch| *ch = t * *ch);
    b.iter_mut().for_each(|ch| *ch = (1.0 - t) * *ch);
    let mut result = [0.0f32; 4];
    result
        .iter_mut()
        .enumerate()
        .for_each(|(i, ch)| *ch = t * a[i] + (1.0 - t) * b[i]);

    pixel_gamma_correction(&mut result);

    Rgba(f32_to_u8(&result))
}
