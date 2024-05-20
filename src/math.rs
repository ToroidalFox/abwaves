use derive_more::{Add, Deref, Div, From, Mul, Sub};

#[derive(Clone, Copy, Debug, From, Add, Sub, Mul, Div)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn length(self) -> f32 {
        f32::sqrt(self.dot(self))
    }
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }
    pub fn distance(self, rhs: impl Into<Self>) -> f32 {
        (self - rhs.into()).length()
    }
    pub fn dot(self, rhs: impl Into<Self>) -> f32 {
        let rhs = rhs.into();
        (self.x * rhs.x) + (self.y * rhs.y)
    }
    pub fn is_normalized(self) -> bool {
        f32::abs(self.length_squared() - 1.0) <= 1e-4
    }
}
impl From<Dir2> for Vec2 {
    fn from(value: Dir2) -> Self {
        Self {
            x: value.0.x,
            y: value.0.y,
        }
    }
}

#[derive(Clone, Copy, Deref)]
pub struct Dir2(Vec2);
impl Dir2 {
    pub const UP: Dir2 = Dir2(Vec2 { x: 0.0, y: 1.0 });
    pub fn new(x: f32, y: f32) -> Result<Self, InvalidDirError> {
        Self::from_vec2((x, y))
    }
    pub fn new_unchecked(from: impl Into<Vec2>) -> Dir2 {
        let from = from.into();
        debug_assert!(from.is_normalized());
        Self(from)
    }
    pub fn from_vec2(from: impl Into<Vec2>) -> Result<Self, InvalidDirError> {
        let from = from.into();
        let length = from.length();

        let is_finite = length.is_finite();
        let is_zero = length <= 0.0;
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

impl std::ops::Mul<f32> for Dir2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        self.0 * rhs
    }
}

#[derive(Clone, Copy, Debug)]
pub enum InvalidDirError {
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
