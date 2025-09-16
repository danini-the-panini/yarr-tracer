use image::{ImageReader, ImageResult, Rgb32FImage};

use crate::{
    background::Background,
    color::Color,
    math::{Point3, Vec2, Vec3},
    texture::Texture,
    util::sphere_uv,
};

pub struct Image(Rgb32FImage);

impl Image {
    pub fn load(path: &str) -> ImageResult<Self> {
        Ok(Self(ImageReader::open(path)?.decode()?.into_rgb32f()))
    }
}

impl Texture for Image {
    fn sample_tex(&self, uv: &Vec2, _p: &Point3) -> Color {
        let u = uv.u().clamp(0.0, 1.0);
        let v = 1.0 - uv.v().clamp(0.0, 1.0);

        let i = (u * self.0.width() as f64) as u32;
        let j = (v * self.0.height() as f64) as u32;

        self.0
            .get_pixel(
                i.clamp(0, self.0.width() - 1),
                j.clamp(0, self.0.height() - 1),
            )
            .into()
    }
}

impl Background for Image {
    fn sample_bg(&self, dir: &Vec3) -> Color {
        let uv = sphere_uv(dir);
        self.sample_tex(&uv, dir)
    }
}
