use std::sync::Arc;

use crate::{
    group::Group,
    material::Material,
    math::{Point3, Vec3},
    point,
    quad::Quad,
    vec3,
};

pub fn make_box(a: Point3, b: Point3, mat: Arc<dyn Material>) -> Group {
    let min = point!(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = point!(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = vec3!(max.x() - min.x(), 0.0, 0.0);
    let dy = vec3!(0.0, max.y() - min.y(), 0.0);
    let dz = vec3!(0.0, 0.0, max.z() - min.z());

    Group::new(vec![
        Box::new(Quad::new(point!(min.x(), min.y(), max.z()), dx, dy, &mat)), // front
        Box::new(Quad::new(point!(max.x(), min.y(), max.z()), -dz, dy, &mat)), // right
        Box::new(Quad::new(point!(max.x(), min.y(), min.z()), -dx, dy, &mat)), // back
        Box::new(Quad::new(point!(min.x(), min.y(), min.z()), dz, dy, &mat)), // left
        Box::new(Quad::new(point!(min.x(), max.y(), max.z()), dx, -dz, &mat)), // top
        Box::new(Quad::new(point!(min.x(), min.y(), min.z()), dx, dz, &mat)), // bottom
    ])
}
