use rand::random;

use crate::ray::Ray;
use crate::vec3;

use crate::math::{Point3, Vec3};

pub struct Camera {
    pub image_width: usize,
    pub image_height: usize,
    pub samples: u32,
    pub samples_scale: f64,
    pub max_depth: u32,

    center: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(
        image_width: usize,
        image_height: usize,
        vfov: f64,
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        samples: u32,
        max_depth: u32,
    ) -> Self {
        let aspect_ratio = (image_width as f64) / (image_height as f64);

        // Camera

        let center = lookfrom;

        let focal_length = (lookfrom - lookat).length();
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * aspect_ratio;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = center - (focal_length * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            image_width,
            image_height,
            samples,
            samples_scale: 1.0 / (samples as f64),
            max_depth,
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = vec3!(random::<f64>() - 0.5, random::<f64>() - 0.5, 0.0);
        let pixel_sample = self.pixel00_loc
            + (((i as f64) + offset.x()) * self.pixel_delta_u)
            + (((j as f64) + offset.x()) * self.pixel_delta_v);

        Ray::new(self.center, pixel_sample - self.center, 0.0)
    }
}
