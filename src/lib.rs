pub mod math;
pub mod prelude;
pub mod wave;

use image::Rgba;

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
            self.colors[index].color_at(-self.waves[0].altitude_at(point))
        } else {
            self.colors[index].color_at(self.waves[index - 1].altitude_at(point))
        }
    }
}

pub enum Color {
    Solid(Rgba<u8>),
    GradientByHeight(Rgba<u8>, Rgba<u8>, f32),
}

impl Color {
    pub fn color_at(&self, altitude: f32) -> Rgba<u8> {
        match self {
            Self::Solid(color) => *color,
            Self::GradientByHeight(start_color, end_color, altitude_max) => {
                gamma_color_interp(start_color, end_color, altitude / altitude_max)
            }
        }
    }
}

pub fn color_from_hex(hex_code: &str) -> Rgba<u8> {
    let hex_code = hex_code.trim_start_matches('#');
    let mut channels = (0..3).map(|i| i * 2).map(|i| &hex_code[i..(i + 2)]);

    let mut rgba = Rgba([u8::MAX; 4]);
    for i in 0..3 {
        rgba.0[i] = u8::from_str_radix(channels.next().unwrap(), 16).unwrap();
    }
    rgba
}

pub fn linear_color_interp(a: &Rgba<u8>, b: &Rgba<u8>, t: f32) -> Rgba<u8> {
    let mut interp = [0u8; 4];
    for i in 0..4 {
        interp[i] = ((1.0 - t) * a.0[i] as f32 + t * b.0[i] as f32).round() as u8;
    }
    Rgba(interp)
}

pub fn gamma_color_interp(a: &Rgba<u8>, b: &Rgba<u8>, t: f32) -> Rgba<u8> {
    const GAMMA: f32 = 2.2;

    // conversions
    let u8_to_f32 = |pixel: &[u8; 4]| {
        let mut f32_pixel = [0.0f32; 4];
        for i in 0..4 {
            f32_pixel[i] = pixel[i] as f32 / u8::MAX as f32;
        }
        f32_pixel
    };
    let f32_to_u8 = |pixel: &[f32; 4]| {
        let mut u8_pixel = [0u8; 4];
        for i in 0..4 {
            u8_pixel[i] = (pixel[i] * u8::MAX as f32).round() as u8;
        }
        u8_pixel
    };

    // gamma
    let gamma_encode = |pixel: &mut [f32; 4]| {
        pixel
            .iter_mut()
            .take(3)
            .for_each(|ch| *ch = ch.powf(1.0 / GAMMA));
    };
    let gamma_decode = |pixel: &mut [f32; 4]| {
        pixel.iter_mut().take(3).for_each(|ch| *ch = ch.powf(GAMMA));
    };

    let t = t.clamp(0.0, 1.0);
    let mut a = u8_to_f32(&a.0);
    let mut b = u8_to_f32(&b.0);
    gamma_decode(&mut a);
    gamma_decode(&mut b);

    // interp: (1 - t)a + tb
    let mut result = [0.0f32; 4];
    result
        .iter_mut()
        .enumerate()
        .for_each(|(i, ch)| *ch = (1.0 - t) * a[i] + t * b[i]);

    gamma_encode(&mut result);

    Rgba(f32_to_u8(&result))
}
