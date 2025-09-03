use crate::{
    math::{Point3, Vec3},
    point, vec3,
};

#[derive(Debug, Copy, Clone)]
pub struct Scene {
    pub image_width: usize,
    pub image_height: usize,
    pub aspect_ratio: f64,
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub camera_center: Point3,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub pixel00_loc: Point3,
}
impl Scene {
    pub fn new(image_width: usize, image_height: usize) -> Self {
        let aspect_ratio = (image_width as f64) / (image_height as f64);

        // Camera

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let camera_center = point!(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = vec3!(viewport_width, 0.0, 0.0);
        let viewport_v = vec3!(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            camera_center - vec3!(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            image_width,
            image_height,
            aspect_ratio,
            viewport_width,
            viewport_height,
            camera_center,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
        }
    }
}
