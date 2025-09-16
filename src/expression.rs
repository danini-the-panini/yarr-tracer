use std::collections::HashMap;

use exmex::{
    ArrayType, ExError, Express, FlatEx, MakeOperators, Operator, Val, ValMatcher, ValOpsFactory,
};

use crate::{
    math::{Vec2, Vec3, Vector},
    perlin::PERLIN,
    vec3,
};

impl From<Val> for Vector<4> {
    fn from(val: Val) -> Self {
        match val {
            Val::Array(v) => Vector(v.into_inner().unwrap_or_default()),
            Val::Float(v) => Vector::repeat(v),
            Val::Int(v) => Vector::repeat(v as f64),
            _ => Vector::repeat(0.0),
        }
    }
}

impl From<Val> for Vector<3> {
    fn from(val: Val) -> Self {
        match val {
            Val::Array(v) => vec3!(v[0], v[1], v[2]),
            Val::Float(v) => Vector::repeat(v),
            Val::Int(v) => Vector::repeat(v as f64),
            _ => Vector::repeat(0.0),
        }
    }
}

impl From<Val> for Vector<2> {
    fn from(val: Val) -> Self {
        match val {
            Val::Array(v) => Vec2::new(v[0], v[1]),
            Val::Float(v) => Vector::repeat(v),
            Val::Int(v) => Vector::repeat(v as f64),
            _ => Vector::repeat(0.0),
        }
    }
}

impl<const N: usize> Into<Val> for Vector<N> {
    fn into(self) -> Val {
        Val::Array(ArrayType::from_slice(&self.0))
    }
}

impl<const N: usize> Into<Val> for &Vector<N> {
    fn into(self) -> Val {
        Val::Array(ArrayType::from_slice(&self.0))
    }
}

#[derive(Clone, Debug)]
struct NoiseOpsFactory;
impl MakeOperators<Val> for NoiseOpsFactory {
    fn make<'a>() -> Vec<Operator<'a, Val>> {
        let mut ops = ValOpsFactory::make();
        ops.push(Operator::make_unary("noise", |a| {
            Val::Float(PERLIN.noise(&a.into()))
        }));
        ops.push(Operator::make_bin(
            "turb",
            exmex::BinOp {
                apply: |a, b| Val::Float(PERLIN.turb(&a.into(), b.to_int().unwrap_or_default())),
                prio: 0,
                is_commutative: false,
            },
        ));
        ops
    }
}

type FlatExNoise = FlatEx<Val, NoiseOpsFactory, ValMatcher>;

pub struct Expression(FlatExNoise);

impl Expression {
    pub fn parse(expr: &str) -> Result<Self, ExError> {
        let expr = FlatExNoise::parse(expr)?;
        Ok(Self(expr))
    }

    pub fn eval(&self, vars: HashMap<&str, Val>) -> Result<Val, ExError> {
        let var_names = self.0.var_names();
        let mut pairs: Vec<(&str, Val)> = vars
            .into_iter()
            .filter(|(k, _)| var_names.contains(&k.to_string()))
            .collect();
        pairs.sort_unstable_by_key(|(k, _)| *k);
        let vars: Vec<Val> = pairs.into_iter().map(|(_, v)| v).collect();
        self.0.eval_vec(vars)
    }
}
