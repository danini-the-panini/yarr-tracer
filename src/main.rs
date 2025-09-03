use camera::Camera;
use group::Group;
use math::{Point3, Vec3};
use rand::random;
use scene::Scene;
use sphere::Sphere;
use thread_pool::{render_threaded, render_unthreaded};

use crate::{
    color::Color, dielectric::Dielectric, lambertian::Lambertian, metal::Metal,
    util::random_in_range,
};

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
        center: vec3!(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat: Box::new(Lambertian {
            albedo: rgb!(0.5, 0.5, 0.5),
        }),
    };
    world.add(Box::new(ground));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random();
            let center = point!(
                (a as f64) + 0.9 * random::<f64>(),
                0.2,
                (b as f64) + 0.9 * random::<f64>()
            );

            if (center - point!(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    world.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: Box::new(Lambertian { albedo }),
                    }));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_in_range(0.5, 0.1);
                    let fuzz = random_in_range(0.0, 0.5);
                    world.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: Box::new(Metal { albedo, fuzz }),
                    }));
                } else {
                    // glass
                    world.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: Box::new(Dielectric {
                            refraction_index: 1.5,
                        }),
                    }))
                }
            }
        }
    }

    world.add(Box::new(Sphere {
        center: point!(0.0, 1.0, 0.0),
        radius: 1.0,
        mat: Box::new(Dielectric {
            refraction_index: 1.5,
        }),
    }));

    world.add(Box::new(Sphere {
        center: point!(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat: Box::new(Lambertian {
            albedo: rgb!(0.4, 0.2, 0.1),
        }),
    }));

    world.add(Box::new(Sphere {
        center: point!(4.0, 1.0, 0.0),
        radius: 1.0,
        mat: Box::new(Metal {
            albedo: rgb!(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    }));

    let camera = Camera::new(
        400,
        225,
        20.0,
        point!(13.0, 2.0, 3.0),
        point!(0.0, 0.0, 0.0),
        vec3!(0.0, 1.0, 0.0),
        0.6,
        10.0,
        10,
        50,
    );

    let scene = Scene::new(camera, world);

    let cpus = num_cpus::get();

    render_threaded(cpus, &scene);
    // render_unthreaded(&scene);
}
