use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::sync::Arc;

use crate::background::{Background, BgExpr, Gradient};
use crate::bvh::BVH;
use crate::camera::Camera;
use crate::checker::Checker;
use crate::color::Color;
use crate::constant_medium::ConstantMedium;
use crate::diffuse_light::DiffuseLight;
use crate::group::Group;
use crate::image::Image;
use crate::material::Material;
use crate::math::{Vec2, Vec3};
use crate::object::Object;
use crate::perlin::Noise;
use crate::quad::Quad;
use crate::scene::Scene;
use crate::shapes::make_box;
use crate::solid_color::SolidColor;
use crate::sphere::Sphere;
use crate::texture::Texture;
use crate::transform::{RotateY, Translate};
use crate::{error, rgb, vec3};
use kdl::{KdlDocument, KdlError, KdlNode};
use miette::{Diagnostic, IntoDiagnostic, NamedSource, SourceSpan};

use crate::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal};

#[derive(thiserror::Error, Debug, Diagnostic)]
#[error("{msg:?}")]
#[diagnostic()]
pub struct LoadError {
    msg: String,
    #[label]
    span: Option<SourceSpan>,
}

impl LoadError {
    fn new(msg: &str, node: &KdlNode) -> Self {
        Self {
            msg: msg.into(),
            span: Some(node.span()),
        }
    }

    fn obj(obj: &str, node: &KdlNode) -> Self {
        Self {
            msg: format!("Failed to load {}", obj),
            span: Some(node.span()),
        }
    }

    fn err<E>(obj: &str, err: E, node: &KdlNode) -> Self
    where
        E: Error,
    {
        Self {
            msg: format!("Failed to load {}, {}", obj, err.to_string()),
            span: Some(node.span()),
        }
    }
}

impl From<KdlError> for LoadError {
    fn from(value: KdlError) -> Self {
        Self {
            msg: format!("Failed to load Scene: {}", value.to_string()),
            span: Some(value.diagnostics.first().unwrap().span),
        }
    }
}

impl From<std::io::Error> for LoadError {
    fn from(value: std::io::Error) -> Self {
        Self {
            msg: value.to_string(),
            span: None,
        }
    }
}

impl From<error::Error> for LoadError {
    fn from(value: error::Error) -> Self {
        Self {
            msg: value.to_string(),
            span: None,
        }
    }
}

type LoadResult<T = ()> = Result<T, LoadError>;

fn get_vec(node: &KdlNode, key: &str) -> LoadResult<Vec3> {
    get_vec_at(&node.children().unwrap().get(key).unwrap(), 0)
}

fn get_vec_at(node: &KdlNode, i: usize) -> LoadResult<Vec3> {
    match (
        node.get(i).and_then(|x| x.as_float()),
        node.get(i + 1).and_then(|y| y.as_float()),
        node.get(i + 2).and_then(|z| z.as_float()),
    ) {
        (Some(x), Some(y), Some(z)) => Ok(vec3!(x, y, z)),
        _ => Err(LoadError::obj("Vec3", node)),
    }
}

fn get_vec2(node: &KdlNode, key: &str) -> LoadResult<Vec2> {
    get_vec2_at(&node.children().unwrap().get(key).unwrap(), 0)
}

fn get_vec2_at(node: &KdlNode, i: usize) -> LoadResult<Vec2> {
    match (
        node.get(i).and_then(|x| x.as_float()),
        node.get(i + 1).and_then(|y| y.as_float()),
    ) {
        (Some(x), Some(y)) => Ok(Vec2::new(x, y)),
        _ => Err(LoadError::obj("Vec2", node)),
    }
}

fn get_float(node: &KdlNode, key: &str) -> LoadResult<f64> {
    node.children()
        .and_then(|c| c.get_arg(key))
        .and_then(|a| a.as_float())
        .ok_or_else(|| LoadError::obj("Float", node))
}

fn get_int(node: &KdlNode, key: &str) -> LoadResult<i128> {
    node.children()
        .and_then(|c| c.get_arg(key))
        .and_then(|a| a.as_integer())
        .ok_or_else(|| LoadError::obj("Integer", node))
}

fn parse_checker_tex(node: &KdlNode, key: &str) -> LoadResult<Arc<dyn Texture>> {
    match node.children().and_then(|c| c.get(key)) {
        Some(tnode) => {
            if tnode.get(0).is_some_and(|n| n.is_string()) {
                parse_tex(&tnode)
            } else {
                Ok(Arc::new(SolidColor(get_vec(node, key)?)))
            }
        }
        None => Err(LoadError::obj("Checker", node)),
    }
}

fn parse_checker(node: &KdlNode) -> LoadResult<Checker> {
    Ok(Checker::new(
        get_float(node, "scale")?,
        &parse_checker_tex(node, "even")?,
        &parse_checker_tex(node, "odd")?,
    ))
}

fn parse_solid(node: &KdlNode) -> LoadResult<SolidColor> {
    match (
        node.get(1).and_then(|r| r.as_float()),
        node.get(2).and_then(|g| g.as_float()),
        node.get(3).and_then(|b| b.as_float()),
    ) {
        (Some(r), Some(g), Some(b)) => Ok(SolidColor(rgb!(r, g, b))),
        _ => Err(LoadError::obj("Solid", node)),
    }
}

fn parse_image(node: &KdlNode) -> LoadResult<Image> {
    if let Some(path) = node.get(1).and_then(|a| a.as_string()) {
        Image::load(path).or_else(|err| Err(LoadError::err("Image", err, node)))
    } else {
        Err(LoadError::obj("Image", node))
    }
}

fn parse_noise(node: &KdlNode) -> LoadResult<Noise> {
    if let Some(expr) = node.get(1).and_then(|a| a.as_string()) {
        Noise::parse(expr).or_else(|err| {
            Err(LoadError::new(
                format!("Failed to load Noise: {}", err.to_string()).as_str(),
                node,
            ))
        })
    } else {
        Err(LoadError::obj("Noise", node))
    }
}

fn parse_tex(node: &KdlNode) -> LoadResult<Arc<dyn Texture>> {
    match node.get(0).and_then(|a| a.as_string()) {
        Some("Solid") => Ok(Arc::new(parse_solid(node)?)),
        Some("Checker") => Ok(Arc::new(parse_checker(node)?)),
        Some("Image") => Ok(Arc::new(parse_image(node)?)),
        Some("Noise") => Ok(Arc::new(parse_noise(node)?)),
        _ => Err(LoadError::obj("Texture", node)),
    }
}

fn parse_gradient(node: &KdlNode) -> LoadResult<Gradient> {
    Ok(Gradient::new(
        get_vec(node, "top")?,
        get_vec(node, "bottom")?,
    ))
}

fn parse_bg_expr(node: &KdlNode) -> LoadResult<BgExpr> {
    if let Some(expr) = node.get(1).and_then(|a| a.as_string()) {
        Ok(BgExpr::new(expr.replace("\n", " "))?)
    } else {
        Err(LoadError::obj("BgExpr", node))
    }
}

#[derive(Default)]
struct KdlLoader {
    doc: KdlDocument,
    textures: HashMap<String, Arc<dyn Texture>>,
    materials: HashMap<String, Arc<dyn Material>>,
}

impl KdlLoader {
    fn load(path: String) -> miette::Result<Scene> {
        let source = fs::read_to_string(&path).into_diagnostic()?;
        let doc = KdlDocument::parse_v2(&source)?;

        let (camera, world, background) = Self::load_scene_parts(doc)
            .map_err(|err| err.with_source_code(NamedSource::new(path, source)))?;

        Scene::new(camera, world, background).into_diagnostic()
    }

    fn load_scene_parts(
        doc: KdlDocument,
    ) -> miette::Result<(Camera, Box<dyn Object>, Option<Box<dyn Background>>)> {
        let mut loader = KdlLoader {
            doc,
            ..Default::default()
        };

        loader.load_textures()?;
        loader.load_materials()?;
        let world = loader.parse_world()?;
        let camera = loader.parse_camera()?;
        let background = loader.parse_background()?;

        Ok((camera, world, background))
    }

    fn parse_world(&self) -> LoadResult<Box<dyn Object>> {
        if let Some(nodes) = self.doc.get("World").and_then(|n| n.children()) {
            Ok(BVH::new(self.parse_objects(nodes)?))
        } else {
            Ok(Box::new(Group::default()))
        }
    }

    fn get_tex(&self, node: &KdlNode) -> LoadResult<Arc<dyn Texture>> {
        match node.get(0).and_then(|a| a.as_string()) {
            Some("Solid" | "Image" | "Checker" | "Noise") => parse_tex(node),
            Some(name) => self
                .textures
                .get(name)
                .map(|tex| Arc::clone(tex))
                .ok_or_else(|| LoadError::new(format!("No such texture {}", name).as_str(), node)),
            None => Err(LoadError::obj("Texture", node)),
        }
    }

    fn parse_lambert(&self, node: &KdlNode) -> LoadResult<Lambertian> {
        if node.children().is_some_and(|c| c.get("albedo").is_some()) {
            Ok(Lambertian::solid(get_vec(node, "albedo")?))
        } else if let Some(tex) = node.children().and_then(|c| c.get("tex")) {
            Ok(Lambertian {
                tex: self.get_tex(&tex)?,
            })
        } else {
            Err(LoadError::obj("Lambertian", node))
        }
    }

    fn parse_metal(&self, node: &KdlNode) -> LoadResult<Metal> {
        let fuzz = get_float(node, "fuzz")?;
        if node.children().is_some_and(|c| c.get("albedo").is_some()) {
            Ok(Metal::solid(get_vec(node, "albedo")?, fuzz))
        } else if let Some(tex) = node.children().and_then(|c| c.get("tex")) {
            Ok(Metal {
                tex: self.get_tex(&tex)?,
                fuzz,
            })
        } else {
            Err(LoadError::obj("Metal", node))
        }
    }

    fn parse_diffuse_light(&self, node: &KdlNode) -> LoadResult<DiffuseLight> {
        if node.children().is_some_and(|c| c.get("albedo").is_some()) {
            Ok(DiffuseLight::solid(get_vec(node, "albedo")?))
        } else if let Some(tex) = node.children().and_then(|c| c.get("tex")) {
            Ok(DiffuseLight {
                tex: self.get_tex(&tex)?,
            })
        } else {
            Err(LoadError::obj("DiffuseLight", node))
        }
    }

    fn parse_mat(&self, node: &KdlNode) -> LoadResult<Arc<dyn Material>> {
        match node.get(0).and_then(|a| a.as_string()) {
            Some("Lambertian") => Ok(Arc::new(self.parse_lambert(node)?)),
            Some("Metal") => Ok(Arc::new(self.parse_metal(node)?)),
            Some("Dielectric") => Ok(Arc::new(Dielectric {
                refraction_index: get_float(node, "refraction_index")?,
            })),
            Some("DiffuseLight") => Ok(Arc::new(self.parse_diffuse_light(node)?)),
            Some(name) => Err(LoadError::new(
                format!("Unknown Material {}", name).as_str(),
                node,
            )),
            _ => Err(LoadError::obj("Materal", node)),
        }
    }

    fn get_mat(&self, node: &KdlNode) -> LoadResult<Arc<dyn Material>> {
        match node.get(0).and_then(|a| a.as_string()) {
            Some("Lambertian" | "Metal" | "Dielectric" | "DiffuseLight") => self.parse_mat(node),
            Some(name) => self
                .materials
                .get(name)
                .map(|mat| Arc::clone(mat))
                .ok_or_else(|| LoadError::new(format!("No such material {}", name).as_str(), node)),
            _ => Err(LoadError::obj("Material", node)),
        }
    }

    fn parse_quad(&self, node: &KdlNode) -> LoadResult<Box<dyn Object>> {
        let q = get_vec(node, "q")?;
        let u = get_vec(node, "u")?;
        let v = get_vec(node, "v")?;
        if let Some(n) = node.children().and_then(|c| c.get("mat")) {
            let mat = self.get_mat(&n)?;
            Ok(Box::new(Quad::new(q, u, v, &mat)))
        } else {
            Err(LoadError::obj("Quad", node))
        }
    }

    fn parse_sphere(&self, node: &KdlNode) -> LoadResult<Box<dyn Object>> {
        let radius = get_float(node, "radius")?;
        if let Some(n) = node.children().and_then(|c| c.get("mat")) {
            let mat = self.get_mat(&n)?;
            Ok(Box::new(
                if node
                    .children()
                    .is_some_and(|c| c.get_arg("center").is_some())
                {
                    Sphere::stationary(get_vec(node, "center")?, radius, &mat)
                } else {
                    Sphere::moving(
                        get_vec(node, "center1")?,
                        get_vec(node, "center2")?,
                        radius,
                        &mat,
                    )
                },
            ))
        } else {
            Err(LoadError::obj("Sphere", node))
        }
    }

    fn parse_box(&self, node: &KdlNode) -> LoadResult<Box<dyn Object>> {
        let a = get_vec(node, "a")?;
        let b = get_vec(node, "b")?;
        if let Some(n) = node.children().and_then(|c| c.get("mat")) {
            let mat = self.get_mat(&n)?;
            Ok(Box::new(make_box(a, b, mat)))
        } else {
            Err(LoadError::obj("Box", node))
        }
    }

    fn parse_translate(&self, node: &KdlNode) -> LoadResult<Box<dyn Object>> {
        let offset = get_vec(node, "offset")?;
        if let Some((ty, obj)) = node
            .children()
            .and_then(|c| c.get("obj"))
            .and_then(|obj| obj.get(0).and_then(|a| a.as_string().map(|ty| (ty, obj))))
        {
            let obj = self.parse_object(ty, obj)?;
            Ok(Box::new(Translate::new(obj, offset)))
        } else {
            Err(LoadError::obj("Translate", node))
        }
    }

    fn parse_rotate_y(&self, node: &KdlNode) -> LoadResult<Box<dyn Object>> {
        let angle = get_float(node, "angle")?;
        if let Some((ty, obj)) = node
            .children()
            .and_then(|c| c.get("obj"))
            .and_then(|obj| obj.get(0).and_then(|a| a.as_string().map(|ty| (ty, obj))))
        {
            let obj = self.parse_object(ty, obj)?;
            Ok(Box::new(RotateY::new(obj, angle)))
        } else {
            Err(LoadError::obj("Translate", node))
        }
    }

    fn parse_constant_medium(&self, node: &KdlNode) -> LoadResult<Box<dyn Object>> {
        let density = get_float(node, "density")?;
        let children = node.children();
        if let Some((ty, obj)) = children
            .and_then(|c| c.get("boundary"))
            .and_then(|obj| obj.get(0).and_then(|a| a.as_string().map(|ty| (ty, obj))))
        {
            let boundary = self.parse_object(ty, obj)?;
            if children.is_some_and(|c| c.get("color").is_some()) {
                Ok(Box::new(ConstantMedium::solid(
                    boundary,
                    density,
                    get_vec(node, "color")?,
                )))
            } else if let Some(tex) = children.and_then(|c| c.get("tex")) {
                let tex = self.get_tex(tex)?;
                Ok(Box::new(ConstantMedium::new(boundary, density, tex)))
            } else {
                Err(LoadError::obj("ConstantMedium", node))
            }
        } else {
            Err(LoadError::obj("ConstantMedium", node))
        }
    }

    fn parse_object(&self, name: &str, node: &KdlNode) -> LoadResult<Box<dyn Object>> {
        match name {
            "Group" => self.parse_group(node),
            "BVH" => self.parse_bvh(node),
            "Sphere" => self.parse_sphere(node),
            "Quad" => self.parse_quad(node),
            "Box" => self.parse_box(node),
            "Translate" => self.parse_translate(node),
            "RotateY" => self.parse_rotate_y(node),
            "ConstantMedium" => self.parse_constant_medium(node),
            _ => Err(LoadError::new(
                format!("Unknown object type {}", node.name().value()).as_str(),
                node,
            )),
        }
    }

    fn parse_group(&self, node: &KdlNode) -> LoadResult<Box<dyn Object>> {
        if let Some(children) = node.children() {
            Ok(Box::new(Group::new(self.parse_objects(children)?)))
        } else {
            Ok(Box::new(Group::default()))
        }
    }

    fn parse_bvh(&self, node: &KdlNode) -> LoadResult<Box<dyn Object>> {
        if let Some(children) = node.children() {
            Ok(BVH::new(self.parse_objects(children)?))
        } else {
            Ok(Box::new(Group::default()))
        }
    }

    fn parse_objects(&self, kdl: &KdlDocument) -> LoadResult<Vec<Box<dyn Object>>> {
        kdl.nodes()
            .iter()
            .map(|node| self.parse_object(node.name().value(), node))
            .collect()
    }

    fn parse_camera(&self) -> LoadResult<Camera> {
        if let Some(camera) = self.doc.get("Camera") {
            Ok(Camera::new(
                get_int(&camera, "image_width")? as usize,
                get_int(&camera, "image_height")? as usize,
                get_float(&camera, "vfov")?,
                get_vec(&camera, "lookfrom")?,
                get_vec(&camera, "lookat")?,
                get_vec(&camera, "vup")?,
                get_float(&camera, "defocus_angle")?,
                get_float(&camera, "focus_dist")?,
                get_int(&camera, "samples")? as u32,
                get_int(&camera, "max_depth")? as u32,
            ))
        } else {
            Err(LoadError {
                msg: "Failed to load Camera".into(),
                span: None,
            })
        }
    }

    fn parse_background(&self) -> LoadResult<Option<Box<dyn Background>>> {
        if let Some(node) = self.doc.get("Background") {
            match node.get(0).and_then(|a| a.as_string()) {
                Some("Solid") => Ok(Some(Box::new(SolidColor(get_vec_at(&node, 1)?)))),
                Some("Gradient") => Ok(Some(Box::new(parse_gradient(&node)?))),
                // "ClearSky" => Box::new(ClearSky::new(
                //     get_vec2(&node, "sun"),
                //     get_float(&node, "scale"),
                //     get_vec(&node, "sun_color"),
                //     parse_gradient(node.children().unwrap().get("bg").unwrap()),
                // )),
                Some("Image") => {
                    parse_image(&node).map(|x| Some(Box::new(x) as Box<dyn Background>))
                }
                Some("Expression") => {
                    parse_bg_expr(&node).map(|x| Some(Box::new(x) as Box<dyn Background>))
                }
                Some(ty) => Err(LoadError::new(
                    format!("unknown background type {}", ty).as_str(),
                    node,
                )),
                None => Err(LoadError::obj("Background", node)),
            }
        } else {
            Ok(None)
        }
    }

    fn load_textures(&mut self) -> LoadResult {
        if let Some(nodes) = self
            .doc
            .get("Textures")
            .and_then(|t| t.children())
            .map(|c| c.nodes())
        {
            self.textures = nodes
                .iter()
                .map(|tnode| {
                    let name = tnode.name().value().to_string();
                    parse_tex(tnode).map(|tex| (name, tex))
                })
                .collect::<LoadResult<HashMap<String, Arc<dyn Texture>>>>()?;
        }
        Ok(())
    }

    fn load_materials(&mut self) -> LoadResult {
        if let Some(nodes) = self
            .doc
            .get("Materials")
            .and_then(|t| t.children())
            .map(|c| c.nodes())
        {
            self.materials = nodes
                .iter()
                .map(|mnode| {
                    let name = mnode.name().value().to_string();
                    self.parse_mat(mnode).map(|mat| (name, mat))
                })
                .collect::<LoadResult<HashMap<String, Arc<dyn Material>>>>()?;
        }
        Ok(())
    }
}

pub fn load_scene(path: String) -> miette::Result<Scene> {
    KdlLoader::load(path)
}
