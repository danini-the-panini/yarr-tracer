use crate::{
    interval::Interval,
    object::{Hit, Object},
    ray::Ray,
};

#[derive(Default)]
pub struct Group<'a> {
    pub objects: Vec<&'a dyn Object>,
}

impl<'a> Group<'a> {
    pub fn add(&mut self, object: &'a dyn Object) {
        self.objects.push(object);
    }
}

impl<'a> Object for Group<'a> {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<Hit> {
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
}
