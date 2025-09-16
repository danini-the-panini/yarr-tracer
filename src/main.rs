use std::env;

use thread_pool::{render_threaded, render_unthreaded};

use crate::loader::load_scene;

mod aabb;
mod background;
mod bvh;
mod camera;
mod checker;
mod color;
mod dielectric;
mod diffuse_light;
mod error;
mod expression;
mod group;
mod image;
mod interval;
mod lambertian;
mod loader;
mod material;
mod math;
mod metal;
mod object;
mod perlin;
mod quad;
mod ray;
mod scene;
mod shapes;
mod solid_color;
mod sphere;
mod test_data;
mod texture;
mod thread_pool;
mod transform;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: yarr <path_to_file.kdl>");
        return;
    }

    let scene =
        load_scene(args[1].clone()).unwrap_or_else(|err| panic!("Failed to load scene: {}", err.0));

    let cpus = num_cpus::get();

    render_threaded(cpus, &scene);
    // render_unthreaded(&scene);
}
