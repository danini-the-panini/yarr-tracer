use crate::{color::Color, rgb, texture::Texture};

pub struct SolidColor(pub Color);

impl SolidColor {
    pub fn rgb(r: f64, g: f64, b: f64) -> Self {
        Self(rgb!(r, g, b))
    }
}

impl Texture for SolidColor {
    fn sample(&self, _uv: crate::math::Vec2, _p: crate::math::Point3) -> Color {
        self.0
    }
}
