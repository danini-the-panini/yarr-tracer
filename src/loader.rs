use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

use crate::bvh::BVH;
use crate::checker::Checker;
use crate::image::Image;
use crate::material::Material;
use crate::math::Vec3;
use crate::object::Object;
use crate::perlin::Noise;
use crate::quad::Quad;
use crate::scene::Scene;
use crate::solid_color::SolidColor;
use crate::sphere::Sphere;
use crate::texture::Texture;
use crate::vec3;
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
    vec3!(values[0], values[1], values[2])
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

fn parse_checker_tex(node: &KdlNode, key: &str) -> Arc<dyn Texture> {
    let tnode = node.children().unwrap().get(key).unwrap();
    if tnode.get(0).unwrap().is_string() {
        parse_tex(&tnode)
    } else {
        Arc::new(SolidColor(get_vec(node, key)))
    }
}

fn parse_checker(node: &KdlNode) -> Checker {
    let scale = get_float(node, "scale");
    Checker::new(
        scale,
        &parse_checker_tex(node, "even"),
        &parse_checker_tex(node, "odd"),
    )
}

fn parse_tex(node: &KdlNode) -> Arc<dyn Texture> {
    match node.get(0).unwrap().as_string().unwrap() {
        "Solid" => Arc::new(SolidColor(vec3!(
            node.get(1).unwrap().as_float().unwrap(),
            node.get(2).unwrap().as_float().unwrap(),
            node.get(3).unwrap().as_float().unwrap()
        ))),
        "Checker" => Arc::new(parse_checker(node)),
        "Image" => Arc::new(Image::load(node.get(1).unwrap().as_string().unwrap()).unwrap()),
        "Noise" => Arc::new(Noise::parse(node.get(1).unwrap().as_string().unwrap()).unwrap()),
        _ => panic!("Unknown texture type {}", node.name().value()),
    }
}

#[derive(Default)]
struct KdlLoader {
    doc: KdlDocument,
    textures: HashMap<String, Arc<dyn Texture>>,
    materials: HashMap<String, Arc<dyn Material>>,
}

impl KdlLoader {
    fn load(path: String) -> Result<Scene, Error> {
        let mut loader = KdlLoader {
            doc: KdlDocument::parse_v2(&fs::read_to_string(path)?)?,
            ..Default::default()
        };

        loader.load_textures();
        loader.load_materials();
        let world = loader.parse_world();
        let camera = loader.parse_camera();

        Ok(Scene::new(camera, world))
    }

    fn parse_world(&self) -> Box<dyn Object> {
        self.parse_group(self.doc.get("World").unwrap().children().unwrap())
    }

    fn get_tex(&self, node: &KdlNode) -> Arc<dyn Texture> {
        match node.get(0).unwrap().as_string().unwrap() {
            "Solid" | "Image" | "Noise" => parse_tex(node),
            name => Arc::clone(self.textures.get(name).unwrap()),
        }
    }

    fn parse_lambert(&self, node: &KdlNode) -> Lambertian {
        if node.children().unwrap().get("albedo").is_some() {
            Lambertian::solid(get_vec(node, "albedo"))
        } else {
            Lambertian {
                tex: self.get_tex(&node.children().unwrap().get("tex").unwrap()),
            }
        }
    }

    fn parse_metal(&self, node: &KdlNode) -> Metal {
        let fuzz = get_float(node, "fuzz");
        if node.children().unwrap().get("albedo").is_some() {
            Metal::solid(get_vec(node, "albedo"), fuzz)
        } else {
            Metal {
                tex: self.get_tex(&node.children().unwrap().get("tex").unwrap()),
                fuzz,
            }
        }
    }

    fn parse_mat(&self, node: &KdlNode) -> Arc<dyn Material> {
        match node.get(0).unwrap().as_string().unwrap() {
            "Lambertian" => Arc::new(self.parse_lambert(node)),
            "Metal" => Arc::new(self.parse_metal(node)),
            "Dielectric" => Arc::new(Dielectric {
                refraction_index: get_float(node, "refraction_index"),
            }),
            _ => panic!("Unknown object type {}", node.name().value()),
        }
    }

    fn get_mat(&self, node: &KdlNode) -> Arc<dyn Material> {
        match node.get(0).unwrap().as_string().unwrap() {
            "Lambertian" | "Metal" | "Dielectric" => self.parse_mat(node),
            name => Arc::clone(self.materials.get(name).unwrap()),
        }
    }

    fn parse_quad(&self, node: &KdlNode) -> Box<dyn Object> {
        let q = get_vec(node, "q");
        let u = get_vec(node, "u");
        let v = get_vec(node, "v");
        let mat = self.get_mat(&node.children().unwrap().get("mat").unwrap());
        Box::new(Quad::new(q, u, v, &mat))
    }

    fn parse_sphere(&self, node: &KdlNode) -> Box<dyn Object> {
        let radius = get_float(node, "radius");
        let mat = self.get_mat(&node.children().unwrap().get("mat").unwrap());
        Box::new(if node.children().unwrap().get_arg("center").is_some() {
            Sphere::stationary(get_vec(node, "center"), radius, &mat)
        } else {
            Sphere::moving(
                get_vec(node, "center1"),
                get_vec(node, "center2"),
                radius,
                &mat,
            )
        })
    }

    fn parse_group(&self, kdl: &KdlDocument) -> Box<dyn Object> {
        let objects = kdl
            .nodes()
            .iter()
            .map(|node| match node.name().value() {
                "Sphere" => self.parse_sphere(node),
                "Quad" => self.parse_quad(node),
                _ => panic!("Unknown object type {}", node.name().value()),
            })
            .collect();
        BVH::new(objects)
    }

    fn parse_camera(&self) -> Camera {
        let camera = self.doc.get("Camera").unwrap();
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

    fn load_textures(&mut self) {
        if let Some(node) = self.doc.get("Textures") {
            self.textures = node
                .children()
                .unwrap()
                .nodes()
                .iter()
                .map(|tnode| (tnode.name().value().to_string(), parse_tex(tnode)))
                .collect();
        }
    }

    fn load_materials(&mut self) {
        if let Some(node) = self.doc.get("Materials") {
            self.materials = node
                .children()
                .unwrap()
                .nodes()
                .iter()
                .map(|tnode| (tnode.name().value().to_string(), self.parse_mat(tnode)))
                .collect();
        }
    }
}

pub fn load_scene(path: String) -> Result<Scene, Error> {
    KdlLoader::load(path)
}
