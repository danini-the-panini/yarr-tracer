use camera::Camera;
use group::Group;
use math::{Point3, Vec3};
use scene::Scene;
use sphere::Sphere;
use thread_pool::{render_threaded, render_unthreaded};

use crate::{color::Color, dielectric::Dielectric, lambertian::Lambertian, metal::Metal};

mod camera;
mod color;
mod dielectric;
mod group;
mod interval;
mod lambertian;
mod material;
mod math;
mod metal;
mod object;
mod ray;
mod scene;
mod sphere;
mod test_data;
mod thread_pool;
mod util;

fn main() {
    let mut world = Group::default();

    let ground = Sphere {
        center: vec3!(0.0, -100.5, -1.0),
        radius: 100.0,
        mat: Lambertian {
            albedo: rgb!(0.8, 0.8, 0.0),
        },
    };
    let sphere_center = Sphere {
        center: vec3!(0.0, 0.0, -1.2),
        radius: 0.5,
        mat: Lambertian {
            albedo: rgb!(0.1, 0.2, 0.5),
        },
    };
    let sphere_left = Sphere {
        center: vec3!(-1.0, 0.0, -1.0),
        radius: 0.5,
        mat: Dielectric {
            refraction_index: 1.50,
        },
    };
    let sphere_bubble = Sphere {
        center: vec3!(-1.0, 0.0, -1.0),
        radius: 0.4,
        mat: Dielectric {
            refraction_index: 1.00 / 1.50,
        },
    };
    let sphere_right = Sphere {
        center: vec3!(1.0, 0.0, -1.0),
        radius: 0.5,
        mat: Metal {
            albedo: rgb!(0.8, 0.6, 0.2),
            fuzz: 1.0,
        },
    };

    world.add(&ground);
    world.add(&sphere_center);
    world.add(&sphere_left);
    world.add(&sphere_bubble);
    world.add(&sphere_right);

    let camera = Camera::new(
        400,
        225,
        20.0,
        point!(-2.0, 2.0, 1.0),
        point!(0.0, 0.0, -1.0),
        vec3!(0.0, 1.0, 0.0),
        100,
        50,
    );

    let scene = Scene::new(camera, world);

    let cpus = num_cpus::get();

    render_threaded(cpus, &scene);
    // render_unthreaded(&scene);
}
