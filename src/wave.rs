use crate::math::{Dir2, Vec2};

use noise::{NoiseFn, Perlin};
use std::f32::consts as f32c;

pub struct Wave {
    pub origin: Vec2,
    pub up: Dir2,
    pub series: Vec<HeightFn>,
}

impl Wave {
    pub fn height_at(&self, point: impl Into<Vec2>) -> f32 {
        let point = point.into();
        let t = (point - self.origin).dot(self.up.rotate_90());
        let altitude = (point - self.origin).dot(self.up);
        let elevation = self
            .series
            .iter()
            .map(|height_fn| height_fn.get(t))
            .sum::<f32>();

        altitude - elevation
    }
}

pub enum HeightFn {
    Sine(SineWave),
    Perlin(PerlinWave),
}

impl HeightFn {
    pub fn new_sine(amplitude: f32, wavelength: f32, phase: f32) -> Self {
        Self::Sine(SineWave::new(amplitude, wavelength, phase))
    }
    pub fn new_perlin(amplitude: f32, wavelength: f32, seed: u32) -> Self {
        Self::Perlin(PerlinWave::new(amplitude, wavelength, seed))
    }

    fn get(&self, x: f32) -> f32 {
        match self {
            Self::Sine(wave) => wave.get(x),
            Self::Perlin(wave) => wave.get(x),
        }
    }
}

pub struct SineWave {
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

    pub fn phase(&self) -> f32 {
        self.phase
    }

    pub fn get(&self, x: f32) -> f32 {
        let x = (x * f32c::TAU / self.wavelength).rem_euclid(f32c::TAU) + self.phase();
        self.amplitude * f32::sin(x)
    }
}

struct PerlinWave {
    pub amplitude: f32,
    pub wavelength: f32,
    perlin: Perlin,
}

impl PerlinWave {
    pub fn new(amplitude: f32, wavelength: f32, seed: u32) -> Self {
        Self {
            amplitude,
            wavelength,
            perlin: Perlin::new(seed),
        }
    }
    pub fn get(&self, x: f32) -> f32 {
        self.perlin.get([x as f64]) as f32
    }
}
