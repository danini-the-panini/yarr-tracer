use crate::{
    background::Background,
    color::Color,
    math::{Point3, Vec2, Vec3},
    rgb,
    texture::Texture,
};

pub struct SolidColor(pub Color);

impl SolidColor {
    pub fn rgb(r: f64, g: f64, b: f64) -> Self {
        Self(rgb!(r, g, b))
    }
}

impl Texture for SolidColor {
    fn sample_tex(&self, _uv: &Vec2, _p: &Point3) -> Color {
        self.0
    }
}

impl Background for SolidColor {
    fn sample_bg(&self, _dir: &Vec3) -> Color {
        self.0
    }
}
