use crate::{
    camera::Camera, color::Color, group::Group, interval::Interval, object::Object, ray::Ray, rgb,
};

pub struct Scene<'a> {
    pub camera: Camera,
    pub world: Group<'a>,
}

impl<'a> Scene<'a> {
    pub fn new(camera: Camera, world: Group<'a>) -> Self {
        Self { camera, world }
    }

    pub fn render(&self, i: usize, j: usize) -> (u8, u8, u8) {
        let pixel_center = self.camera.pixel00_loc
            + ((i as f64) * self.camera.pixel_delta_u)
            + ((j as f64) * self.camera.pixel_delta_v);
        let ray_direction = pixel_center - self.camera.center;
        let r = Ray::new(self.camera.center, ray_direction, 0.0);
        let pixel_color = self.ray_color(&r);

        pixel_color.to_pixel()
    }

    fn ray_color(&self, r: &Ray) -> Color {
        if let Some(hit) = self.world.hit(r, &Interval::from(0.0)) {
            return 0.5 * (hit.normal + rgb!(1.0, 1.0, 1.0));
        }

        let unit_direction = r.direction.unit();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * rgb!(1.0, 1.0, 1.0) + a * rgb!(0.5, 0.7, 1.0)
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
