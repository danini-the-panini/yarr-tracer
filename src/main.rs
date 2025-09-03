use camera::Camera;
use group::Group;
use math::Vec3;
use scene::Scene;
use sphere::Sphere;
use thread_pool::{render_threaded, render_unthreaded};

mod camera;
mod color;
mod group;
mod interval;
mod math;
mod object;
mod ray;
mod scene;
mod sphere;
mod test_data;
mod thread_pool;
mod util;

fn main() {
    let mut world = Group::default();

    let sphere = Sphere {
        center: vec3!(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    let ground = Sphere {
        center: vec3!(0.0, -100.5, -1.0),
        radius: 100.0,
    };

    world.add(&sphere);
    world.add(&ground);

    let camera = Camera::new(400, 225, 100);

    let scene = Scene::new(camera, world);

    let cpus = num_cpus::get();

    render_threaded(cpus, &scene);
    // render_unthreaded(&scene);
}
