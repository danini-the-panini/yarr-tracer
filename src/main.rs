use std::{env, fs};

use camera::Camera;
use group::Group;
use kdl::{KdlDocument, KdlNode};
use material::Material;
use math::Vec3;
use scene::Scene;
use sphere::Sphere;
use thread_pool::{render_threaded, render_unthreaded};

use crate::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};

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

#[derive(Debug)]
pub struct Error(String);

impl<E> From<E> for Error
where
    E: ToString,
{
    fn from(value: E) -> Self {
        Self(value.to_string())
    }
}

fn get_vec(node: &KdlNode, key: &str) -> Vec3 {
    let values: Vec<f64> = node
        .children()
        .unwrap()
        .iter_args(key)
        .map(|v| v.as_float().unwrap())
        .collect();
    Vec3::new(values[0], values[1], values[2])
}

fn get_float(node: &KdlNode, key: &str) -> f64 {
    node.children()
        .unwrap()
        .get_arg(key)
        .unwrap()
        .as_float()
        .unwrap()
}
fn get_int(node: &KdlNode, key: &str) -> i128 {
    node.children()
        .unwrap()
        .get_arg(key)
        .unwrap()
        .as_integer()
        .unwrap()
}

fn get_camera(kdl: &KdlDocument) -> Camera {
    let camera = kdl.get("Camera").unwrap();
    Camera::new(
        get_int(&camera, "image_width") as usize,
        get_int(&camera, "image_height") as usize,
        get_float(&camera, "vfov"),
        get_vec(&camera, "lookfrom"),
        get_vec(&camera, "lookat"),
        get_vec(&camera, "vup"),
        get_float(&camera, "defocus_angle"),
        get_float(&camera, "focus_dist"),
        get_int(&camera, "samples") as u32,
        get_int(&camera, "max_depth") as u32,
    )
}

fn parse_mat(node: &KdlNode) -> Box<dyn Material> {
    match node.get(0).unwrap().as_string().unwrap() {
        "Lambertian" => Box::new(Lambertian {
            albedo: get_vec(&node, "albedo"),
        }),
        "Metal" => Box::new(Metal {
            albedo: get_vec(&node, "albedo"),
            fuzz: get_float(&node, "fuzz"),
        }),
        "Dielectric" => Box::new(Dielectric {
            refraction_index: get_float(&node, "refraction_index"),
        }),
        _ => panic!("Unknown object type {}", node.name().value()),
    }
}

fn parse_group(kdl: &KdlDocument) -> Group {
    let mut group = Group::default();
    for node in kdl.nodes() {
        group.add(match node.name().value() {
            "Sphere" => Box::new(Sphere {
                center: get_vec(&node, "center"),
                radius: get_float(&node, "radius"),
                mat: parse_mat(&node.children().unwrap().get("mat").unwrap()),
            }),
            _ => panic!("Unknown object type {}", node.name().value()),
        });
    }
    group
}

fn get_world(kdl: &KdlDocument) -> Group {
    parse_group(kdl.get("World").unwrap().children().unwrap())
}

fn load_scene(path: String) -> Result<Scene, Error> {
    let scene_kdl = KdlDocument::parse_v2(&fs::read_to_string(path)?)?;

    Ok(Scene::new(get_camera(&scene_kdl), get_world(&scene_kdl)))
}

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
