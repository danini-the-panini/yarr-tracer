use std::fs;

use crate::bvh::BVH;
use crate::group::Group;
use crate::material::Material;
use crate::math::Vec3;
use crate::object::Object;
use crate::scene::Scene;
use crate::sphere::Sphere;
use crate::{camera::Camera, error::Error};
use kdl::{KdlDocument, KdlNode};

use crate::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};

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
            albedo: get_vec(node, "albedo"),
        }),
        "Metal" => Box::new(Metal {
            albedo: get_vec(node, "albedo"),
            fuzz: get_float(node, "fuzz"),
        }),
        "Dielectric" => Box::new(Dielectric {
            refraction_index: get_float(node, "refraction_index"),
        }),
        _ => panic!("Unknown object type {}", node.name().value()),
    }
}

fn parse_sphere(node: &KdlNode) -> Sphere {
    let radius = get_float(node, "radius");
    let mat = parse_mat(&node.children().unwrap().get("mat").unwrap());
    if node.children().unwrap().get_arg("center").is_some() {
        Sphere::stationary(get_vec(node, "center"), radius, mat)
    } else {
        Sphere::moving(
            get_vec(node, "center1"),
            get_vec(node, "center2"),
            radius,
            mat,
        )
    }
}

fn parse_group(kdl: &KdlDocument) -> Box<dyn Object> {
    let objects = kdl
        .nodes()
        .iter()
        .map(|node| match node.name().value() {
            "Sphere" => Box::new(parse_sphere(node)) as Box<dyn Object>,
            _ => panic!("Unknown object type {}", node.name().value()),
        })
        .collect();
    BVH::new(objects)
}

fn get_world(kdl: &KdlDocument) -> Box<dyn Object> {
    parse_group(kdl.get("World").unwrap().children().unwrap())
}

pub fn load_scene(path: String) -> Result<Scene, Error> {
    let scene_kdl = KdlDocument::parse_v2(&fs::read_to_string(path)?)?;

    Ok(Scene::new(get_camera(&scene_kdl), get_world(&scene_kdl)))
}
