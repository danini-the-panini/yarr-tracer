use exmex::Val;
use smallvec::smallvec;
use std::{array, collections::HashMap, sync::LazyLock};

use rand::{rng, seq::SliceRandom};

use crate::{
    color::Color,
    error,
    expression::Expression,
    math::{Point3, Vec2, Vec3},
    texture::Texture,
    vec3,
};

pub static PERLIN: LazyLock<Perlin> = LazyLock::new(|| Perlin::new());

const POINT_COUNT: usize = 256;

fn trilinear_interp(c: &[[[Vec3; 2]; 2]; 2], vec: &Vec3) -> f64 {
    let v = vec.smoothed();
    let mut accum = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let fi = i as f64;
                let fj = j as f64;
                let fk = k as f64;
                let weight = *vec - vec3!(fi, fj, fk);
                accum += (fi * v.x() + (1.0 - fi) * (1.0 - v.x()))
                    * (fj * v.y() + (1.0 - fj) * (1.0 - v.y()))
                    * (fk * v.z() + (1.0 - fk) * (1.0 - v.z()))
                    * c[i][j][k].dot(&weight);
            }
        }
    }

    accum
}

pub struct Perlin {
    randvec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut perm_x: [usize; POINT_COUNT] = array::from_fn(|i| i);
        perm_x.shuffle(&mut rng());
        let mut perm_y: [usize; POINT_COUNT] = array::from_fn(|j| j);
        perm_y.shuffle(&mut rng());
        let mut perm_z: [usize; POINT_COUNT] = array::from_fn(|k| k);
        perm_z.shuffle(&mut rng());
        Self {
            randvec: array::from_fn(|_| Vec3::random_in_range(-1.0, 1.0)),
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let fp = p.floored();
        let vec = *p - fp;
        let i = fp.x() as i32;
        let j = fp.y() as i32;
        let k = fp.z() as i32;

        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randvec[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]]
                }
            }
        }

        trilinear_interp(&c, &vec)
    }

    pub fn turb(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

pub struct Noise(Expression);

impl Noise {
    pub fn parse(expr: &str) -> Result<Self, error::Error> {
        let expr = Expression::parse(expr)?;
        Ok(Self(expr))
    }
}

impl Texture for Noise {
    fn sample_tex(&self, uv: &Vec2, p: &Point3) -> Color {
        self.0
            .eval(HashMap::from([
                ("p", p.into()),
                ("x", Val::Float(p.x())),
                ("y", Val::Float(p.y())),
                ("z", Val::Float(p.z())),
                ("uv", uv.into()),
                ("u", Val::Float(uv.u())),
                ("v", Val::Float(uv.v())),
            ]))
            .unwrap_or(Val::Array(smallvec![1.0, 0.0, 1.0]))
            .into()
    }
}
