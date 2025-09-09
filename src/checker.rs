use crate::{
    color::Color,
    math::{Point3, Vec2},
    solid_color::SolidColor,
    texture::Texture,
};

pub struct Checker {
    inv_scale: f64,
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl Checker {
    pub fn new(scale: f64, even: Box<dyn Texture>, odd: Box<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn solid(scale: f64, even: Color, odd: Color) -> Self {
        Self::new(scale, Box::new(SolidColor(even)), Box::new(SolidColor(odd)))
    }
}

impl Texture for Checker {
    fn sample(&self, uv: Vec2, p: Point3) -> Color {
        let x = (self.inv_scale * p.x()).floor() as i32;
        let y = (self.inv_scale * p.y()).floor() as i32;
        let z = (self.inv_scale * p.z()).floor() as i32;

        if (x + y + z) % 2 == 0 {
            self.even.sample(uv, p)
        } else {
            self.odd.sample(uv, p)
        }
    }
}
