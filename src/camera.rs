use rand::random;

use crate::ray::Ray;
use crate::{point, vec3};

use crate::math::{Point3, Vec3};

pub struct Camera {
    pub image_width: usize,
    pub image_height: usize,
    pub aspect_ratio: f64,
    pub samples: u32,
    pub samples_scale: f64,
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub center: Point3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub pixel00_loc: Point3,
}

impl Camera {
    pub fn new(image_width: usize, image_height: usize, samples: u32) -> Self {
        let aspect_ratio = (image_width as f64) / (image_height as f64);

        // Camera

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let center = point!(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = vec3!(viewport_width, 0.0, 0.0);
        let viewport_v = vec3!(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - vec3!(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            image_width,
            image_height,
            aspect_ratio,
            samples,
            samples_scale: 1.0 / (samples as f64),
            viewport_width,
            viewport_height,
            center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
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
