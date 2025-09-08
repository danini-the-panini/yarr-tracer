use std::cmp::Ordering;

use rand::random_range;

use crate::{
    aabb::AABB,
    group::Group,
    interval::Interval,
    object::{Hit, Object},
    ray::Ray,
};

pub struct BVH {
    left: Box<dyn Object>,
    right: Box<dyn Object>,
    bbox: AABB,
}

fn box_compare(a: &Box<dyn Object>, b: &Box<dyn Object>, axis: usize) -> Ordering {
    let a_axis = a.bbox().axis(axis);
    let b_axis = b.bbox().axis(axis);
    f64::total_cmp(&a_axis.min, &b_axis.min)
}

fn box_x_compare(a: &Box<dyn Object>, b: &Box<dyn Object>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Box<dyn Object>, b: &Box<dyn Object>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Box<dyn Object>, b: &Box<dyn Object>) -> Ordering {
    box_compare(a, b, 2)
}

impl BVH {
    pub fn new(mut objects: Vec<Box<dyn Object>>) -> Box<dyn Object> {
        let mut bbox = AABB::default();
        for object in &objects {
            bbox += object.bbox()
        }

        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };

        match objects.len() {
            1 => {
                let obj = objects.pop().unwrap();
                obj
            }
            2 => {
                let right = objects.pop().unwrap();
                let left = objects.pop().unwrap();
                let bbox = left.bbox() + right.bbox();
                Box::new(Self { left, right, bbox })
            }
            _ => {
                objects.sort_by(comparator);

                let right = objects.split_off(objects.len() / 2);

                let left = BVH::new(objects);
                let right = BVH::new(right);
                let bbox = left.bbox() + right.bbox();

                Box::new(Self { left, right, bbox })
            }
        }
    }

    pub fn from_group(group: Group) -> Box<dyn Object> {
        Self::new(group.objects)
    }
}

impl Object for BVH {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<Hit<'_>> {
        if !self.bbox.hit(r, *ray_t) {
            return None;
        }

        if let Some(hit_left) = self.left.hit(r, ray_t) {
            self.right
                .hit(r, &Interval::new(ray_t.min, hit_left.t))
                .or(Some(hit_left))
        } else {
            self.right.hit(r, ray_t)
        }
    }

    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}
