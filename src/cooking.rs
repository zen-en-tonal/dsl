mod instraction;
mod required;

use self::instraction::Nodes;
use crate::dsl::*;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, ops::Mul};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe<E, T> {
    inner: E,
    to_make: T,
}

impl<E, T> Recipe<E, T>
where
    E: Expr<Ingredient<T>, Process> + Serialize,
    T: Display + Clone,
{
    pub fn new(expr: E, to_make: T) -> Self {
        Recipe {
            inner: expr,
            to_make,
        }
    }

    pub fn instraction(&self) -> Nodes {
        let mut a = instraction::Instraction::new();
        self.inner.analyze_with(&mut a);
        a.into_groups()
    }

    pub fn needed(&self) -> impl Iterator<Item = Ingredient<T>> {
        let mut a = required::Required::new();
        self.inner.analyze_with(&mut a);
        a.into_iter()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Amount {
    Pcs(f32),
    Tsp(f32),
    Gram(f32),
    MilliLitter(f32),
    Pinch,
}

impl Amount {
    pub fn of<T>(self, x: T) -> impl Expr<Ingredient<T>, Process>
    where
        T: Serialize + for<'de> Deserialize<'de>,
    {
        prepare(x, self)
    }
}

impl Mul<f32> for Amount {
    type Output = Amount;

    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Amount::Pcs(x) => Amount::Pcs(x * rhs),
            Amount::Tsp(x) => Amount::Tsp(x * rhs),
            Amount::Gram(x) => Amount::Gram(x * rhs),
            Amount::MilliLitter(x) => Amount::MilliLitter(x * rhs),
            Amount::Pinch => Amount::Pinch,
        }
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lit = match self {
            Amount::Pcs(x) => format!("{} [pcs]", x),
            Amount::Tsp(x) => format!("{} [tsp]", x),
            Amount::Gram(x) => format!("{} [g]", x),
            Amount::MilliLitter(x) => format!("{} [ml]", x),
            Amount::Pinch => format!("pinch"),
        };
        write!(f, "{}", lit)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient<T> {
    kind: T,
    amount: Amount,
}

impl<T> Display for Ingredient<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} of {}(s)", self.amount, self.kind)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Process {
    Cuts(String),
    Stakes(f32),
    Fries(f32),
    Boils(f32),
    Stew(f32),
    Waits(f32),
    Join,
}

pub fn prepare<T>(what: T, amount: Amount) -> impl Expr<Ingredient<T>, Process>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    Value::new(Ingredient { kind: what, amount })
}

pub trait Cooking<T> {
    fn cuts(self, shape: &str) -> impl Expr<Ingredient<T>, Process>;
    fn stakes(self, until: f32) -> impl Expr<Ingredient<T>, Process>;
    fn boils(self, until: f32) -> impl Expr<Ingredient<T>, Process>;
    fn stew(self, until: f32) -> impl Expr<Ingredient<T>, Process>;
    fn joins(self, other: impl Expr<Ingredient<T>, Process>) -> impl Expr<Ingredient<T>, Process>;
}

impl<F, T> Cooking<T> for F
where
    F: Expr<Ingredient<T>, Process>,
{
    fn cuts(self, shape: &str) -> impl Expr<Ingredient<T>, Process> {
        self.apply(Process::Cuts(shape.to_string()), Ident::new())
    }

    fn stakes(self, until: f32) -> impl Expr<Ingredient<T>, Process> {
        self.apply(Process::Stakes(until), Ident::new())
    }

    fn boils(self, until: f32) -> impl Expr<Ingredient<T>, Process> {
        self.apply(Process::Boils(until), Ident::new())
    }

    fn joins(self, other: impl Expr<Ingredient<T>, Process>) -> impl Expr<Ingredient<T>, Process> {
        self.apply(Process::Join, other)
    }

    fn stew(self, until: f32) -> impl Expr<Ingredient<T>, Process> {
        self.apply(Process::Stew(until), Ident::new())
    }
}

pub trait CookingAnalyzer<T> {
    fn prepare(&mut self, x: &Ingredient<T>);
    fn cut(&mut self, shape: &str);
    fn stake(&mut self, until: f32);
    fn stew(&mut self, until: f32);
    fn join(&mut self, left: Self, right: Self);
    fn split(&self) -> Self;
}

impl<T, V> Analyzer<Ingredient<V>, Process> for T
where
    T: CookingAnalyzer<V>,
{
    fn value(&mut self, x: &Ingredient<V>) {
        self.prepare(x)
    }

    fn apply(
        &mut self,
        functor: &Process,
        left: &impl Expr<Ingredient<V>, Process>,
        right: &impl Expr<Ingredient<V>, Process>,
    ) {
        match functor {
            Process::Cuts(shape) => {
                left.analyze_with(self);
                self.cut(&shape);
            }
            Process::Stakes(until) => {
                left.analyze_with(self);
                self.stake(*until);
            }
            Process::Fries(_) => todo!(),
            Process::Boils(_) => todo!(),
            Process::Stew(until) => {
                left.analyze_with(self);
                self.stew(*until);
            }
            Process::Waits(_) => todo!(),
            Process::Join => {
                let mut l = self.split();
                left.analyze_with(&mut l);

                let mut r = self.split();
                right.analyze_with(&mut r);

                self.join(l, r);
            }
        }
    }
}
