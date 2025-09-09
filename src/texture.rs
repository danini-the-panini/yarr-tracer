use crate::{
    color::Color,
    math::{Point3, Vec2},
};

pub trait Texture: Send + Sync {
    fn sample(&self, uv: Vec2, p: Point3) -> Color;
}
