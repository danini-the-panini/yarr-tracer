use image::{DynamicImage, GenericImageView, ImageReader, ImageResult};

use crate::{
    color::Color,
    math::{Point3, Vec2},
    texture::Texture,
};

pub struct Image {
    img: DynamicImage,
}

impl Image {
    pub fn load(path: &str) -> ImageResult<Self> {
        Ok(Self {
            img: ImageReader::open(path)?.decode()?,
        })
    }
}

impl Texture for Image {
    fn sample(&self, uv: Vec2, _p: Point3) -> Color {
        let u = uv.u().clamp(0.0, 1.0);
        let v = 1.0 - uv.v().clamp(0.0, 1.0);

        let i = (u * self.img.width() as f64) as u32;
        let j = (v * self.img.height() as f64) as u32;

        self.img.get_pixel(i, j).into()
    }
}
