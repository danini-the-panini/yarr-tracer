use crate::{color::Color, object::Hit, ray::Ray};

pub struct Scatter {
    pub att: Color,
    pub ray: Ray,
}

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, hit: &Hit) -> Option<Scatter>;
}
