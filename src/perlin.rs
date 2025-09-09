use std::array;

use rand::{rng, seq::SliceRandom};

use crate::{
    color::Color,
    math::{Point3, Vec2, Vec3},
    rgb,
    texture::Texture,
    vec3,
};

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
    scale: f64,
}

impl Perlin {
    pub fn new(scale: f64) -> Self {
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
            scale,
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
}

impl Texture for Perlin {
    fn sample(&self, uv: Vec2, p: Point3) -> Color {
        rgb!(0.5, 0.5, 0.5) * (1.0 + self.noise(&(p * self.scale)))
    }
}
