use crate::{
    aabb::AABB,
    interval::Interval,
    object::{Hit, Object},
    ray::Ray,
};

#[derive(Default)]
pub struct Group {
    pub objects: Vec<Box<dyn Object>>,
    bbox: AABB,
}

impl Group {
    pub fn new(objects: Vec<Box<dyn Object>>) -> Self {
        let mut bbox = AABB::default();
        for object in &objects {
            bbox += object.bbox();
        }
        Self { objects, bbox }
    }

    pub fn add(&mut self, object: Box<dyn Object>) {
        self.bbox += object.bbox();
        self.objects.push(object);
    }
}

impl Object for Group {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<Hit<'_>> {
        let mut rec: Option<Hit> = None;
        let mut closest = ray_t.max;

        for object in &self.objects {
            if let Some(hit) = object.hit(r, &Interval::new(ray_t.min, closest)) {
                closest = hit.t;
                rec = Some(hit)
            }
        }

        rec
    }

    fn bbox(&self) -> &AABB {
        &self.bbox
    }
}
