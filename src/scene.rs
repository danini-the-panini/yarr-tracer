use crate::{
    background::{Background, Gradient},
    camera::Camera,
    color::Color,
    error::Error,
    interval::Interval,
    object::Object,
    ray::Ray,
    rgb,
};

pub struct Scene {
    pub camera: Camera,
    pub world: Box<dyn Object>,
    pub background: Box<dyn Background>,
}

impl Scene {
    pub fn new(
        camera: Camera,
        world: Box<dyn Object>,
        bg: Option<Box<dyn Background>>,
    ) -> Result<Self, Error> {
        Ok(Self {
            camera,
            world,
            background: bg.unwrap_or_else(|| Box::new(Gradient::default())),
        })
    }

    pub fn render(&self, i: usize, j: usize) -> (u8, u8, u8) {
        let mut pixel_color = rgb!(0.0, 0.0, 0.0);
        for _ in 0..self.camera.samples {
            let r = self.camera.get_ray(i, j);
            pixel_color += self.ray_color(&r, self.camera.max_depth);
        }
        (pixel_color * self.camera.samples_scale).to_pixel()
    }

    fn ray_color(&self, r: &Ray, depth: u32) -> Color {
        if depth == 0 {
            return rgb!(0.0, 0.0, 0.0);
        }

        if let Some(hit) = self.world.hit(r, &Interval::from(0.001)) {
            let emitted = hit.mat.emitted(r, &hit);
            if let Some(scatter) = hit.mat.scatter(r, &hit) {
                return emitted + scatter.att * self.ray_color(&scatter.ray, depth - 1);
            } else {
                return emitted;
            }
        }

        let dir = r.direction.unit();
        self.background.sample_bg(&dir)
    }

    pub fn write_image_header(&self) {
        println!(
            "P3\n{} {}\n255",
            self.camera.image_width, self.camera.image_height
        );
    }

    pub fn write_pixel(&self, color: (u8, u8, u8)) {
        println!("{} {} {}", color.0, color.1, color.2);
    }
}
