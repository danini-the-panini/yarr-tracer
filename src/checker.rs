use std::sync::Arc;

use crate::{
    color::Color,
    math::{Point3, Vec2},
    solid_color::SolidColor,
    texture::Texture,
};

pub struct Checker {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl Checker {
    pub fn new(scale: f64, even: &Arc<dyn Texture>, odd: &Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::clone(even),
            odd: Arc::clone(odd),
        }
    }

    pub fn solid(scale: f64, even: Color, odd: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(SolidColor(even)),
            odd: Arc::new(SolidColor(odd)),
        }
    }
}

impl Texture for Checker {
    fn sample_tex(&self, uv: &Vec2, p: &Point3) -> Color {
        let x = (self.inv_scale * p.x()).floor() as i32;
        let y = (self.inv_scale * p.y()).floor() as i32;
        let z = (self.inv_scale * p.z()).floor() as i32;

        if (x + y + z) % 2 == 0 {
            self.even.sample_tex(uv, p)
        } else {
            self.odd.sample_tex(uv, p)
        }
    }
}
