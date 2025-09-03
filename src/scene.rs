use crate::{
    camera::Camera, color::Color, group::Group, interval::Interval, math::Vec3, object::Object,
    ray::Ray, rgb,
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
            let direction = hit.normal + Vec3::random_unit();
            return 0.5 * self.ray_color(&Ray::new(hit.p, direction, 0.0), depth - 1);
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
