use std::collections::HashMap;

use exmex::Val;

use crate::{
    color::Color,
    error::Error,
    expression::Expression,
    math::{Vec2, Vec3},
    rgb,
};

pub trait Background: Send + Sync {
    fn sample_bg(&self, dir: &Vec3) -> Color;
}

pub struct Gradient {
    top: Color,
    bottom: Color,
}

impl Gradient {
    pub fn new(top: Color, bottom: Color) -> Self {
        Self { top, bottom }
    }
}

impl Background for Gradient {
    fn sample_bg(&self, dir: &Vec3) -> Color {
        let a = 0.5 * (dir.y() + 1.0);
        (1.0 - a) * self.bottom + a * self.top
    }
}

impl Default for Gradient {
    fn default() -> Self {
        Self::new(rgb!(0.5, 0.7, 1.0), rgb!(1.0, 1.0, 1.0))
    }
}

pub struct BgExpr(Expression);

impl BgExpr {
    pub fn new(expr: String) -> Result<Self, Error> {
        Ok(Self(Expression::parse(expr.as_str())?))
    }
}

impl Background for BgExpr {
    fn sample_bg(&self, dir: &Vec3) -> Color {
        self.0
            .eval(HashMap::from([
                ("d", dir.into()),
                ("x", Val::Float(dir.x())),
                ("y", Val::Float(dir.y())),
                ("z", Val::Float(dir.z())),
            ]))
            .unwrap()
            .into()
    }
}

// pub struct ClearSky {
//     pub sun: Vec2,
//     pub scale: f64,
//     pub sun_color: Color,
//     pub bg: Gradient,
//     denom: f64,
// }

// impl ClearSky {
//     pub fn new(sun: Vec2, scale: f64, sun_color: Color, bg: Gradient) -> Self {
//         let sun = Vec2::new(sun.u().to_radians(), sun.v().to_radians());
//         let z = PI / 2.0 - sun.u();
//         dbg!(z);
//         let cos_z = z.cos();
//         dbg!(cos_z);
//         let denom =
//             (0.91 + 10.0 * (-3.0 * z).exp() + 0.45 * cos_z * cos_z) * (1.0 - (-0.32_f64).exp());
//         dbg!(denom);
//         Self {
//             sun,
//             scale,
//             sun_color,
//             bg,
//             denom,
//         }
//     }
// }

// fn safe_acos(c: f64) -> f64 {
//     if c < -1.0 {
//         PI
//     } else if c > 1.0 {
//         0.0
//     } else {
//         c.acos()
//     }
// }

// fn angle_between(t1: f64, p1: f64, t2: f64, p2: f64) -> f64 {
//     safe_acos(t1.sin() * t2.sin() * (p1 - p2).cos() + t1.cos() * t2.cos())
// }

// impl Background for ClearSky {
//     fn sample(&self, dir: &Vec3) -> Color {
//         let theta = PI + dir.z().atan2(dir.x());
//         let phi = safe_acos(dir.y());

//         let gamma = angle_between(self.sun.u(), self.sun.v(), theta, phi);
//         let cos_gamma = gamma.cos();
//         let num = (0.91 + 10.0 * (-3.0 * gamma).exp() + 0.45 * cos_gamma * cos_gamma)
//             * (1.0 - (-0.32 / theta.cos()).exp());
//         let sun = (self.scale * num / self.denom).max(0.0);
//         // self.bg.sample(dir) + sun * self.sun_color
//         sun * rgb!(1.0, 1.0, 1.0)
//     }
// }
