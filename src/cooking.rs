mod required;
mod tree_parser;

use crate::dsl::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Recipe<E, T> {
    inner: E,
    to_make: T,
}

impl<E, T> Recipe<E, T>
where
    E: Expr<Ingredient<T>, Operations> + Serialize,
    T: Clone,
{
    pub fn new(expr: impl Fn() -> E, to_make: T) -> Self {
        Recipe {
            inner: expr(),
            to_make,
        }
    }

    pub fn parse(&self) -> tree_parser::Tree<Ingredient<T>> {
        tree_parser::parse(&self.inner)
    }

    pub fn needed(&self) -> impl Iterator<Item = Ingredient<T>> {
        let mut a = required::Required::new();
        self.inner.analyze_with(&mut a);
        a.into_iter()
    }

    pub fn into_expr(self) -> E {
        self.inner
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Amount {
    Pcs(f32),
    Tsp(f32),
    Gram(f32),
    MilliLitter(f32),
    Pinch,
}

impl Amount {
    pub fn of<T>(self, x: T) -> impl Expr<Ingredient<T>, Operations>
    where
        T: Serialize + for<'de> Deserialize<'de>,
    {
        prepare(x, self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ingredient<T> {
    kind: T,
    amount: Amount,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Operations {
    Processes(Processes),
    Join,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Processes {
    Cuts { shape: String },
    Stakes { until: f32 },
    Fries { until: f32 },
    Boils { until: f32 },
    Stew { until: f32 },
    Waits { until: f32 },
}

pub fn prepare<T>(what: T, amount: Amount) -> impl Expr<Ingredient<T>, Operations>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    Value::new(Ingredient { kind: what, amount })
}

pub trait Cooking<T> {
    fn apply_process(self, p: Processes) -> impl Expr<Ingredient<T>, Operations>;

    fn joins(
        self,
        other: impl Expr<Ingredient<T>, Operations>,
    ) -> impl Expr<Ingredient<T>, Operations>;
}

pub trait CookingExpr<T>: Cooking<T> {
    fn cuts(self, shape: &str) -> impl Expr<Ingredient<T>, Operations>
    where
        Self: Sized,
    {
        self.apply_process(Processes::Cuts {
            shape: shape.to_string(),
        })
    }

    fn stakes(self, until: f32) -> impl Expr<Ingredient<T>, Operations>
    where
        Self: Sized,
    {
        self.apply_process(Processes::Stakes { until })
    }

    fn stakes_with(
        self,
        other: impl Expr<Ingredient<T>, Operations>,
        until: f32,
    ) -> impl Expr<Ingredient<T>, Operations>
    where
        Self: Sized,
    {
        self.joins(other).stakes(until)
    }

    fn boils(self, until: f32) -> impl Expr<Ingredient<T>, Operations>
    where
        Self: Sized,
    {
        self.apply_process(Processes::Boils { until })
    }

    fn stew(self, until: f32) -> impl Expr<Ingredient<T>, Operations>
    where
        Self: Sized,
    {
        self.apply_process(Processes::Stew { until })
    }

    fn stew_with(
        self,
        other: impl Expr<Ingredient<T>, Operations>,
        until: f32,
    ) -> impl Expr<Ingredient<T>, Operations>
    where
        Self: Sized,
    {
        self.joins(other).stew(until)
    }
}

impl<F, T> Cooking<T> for F
where
    F: Expr<Ingredient<T>, Operations>,
{
    fn joins(
        self,
        other: impl Expr<Ingredient<T>, Operations>,
    ) -> impl Expr<Ingredient<T>, Operations> {
        self.apply(Operations::Join, other)
    }

    fn apply_process(self, p: Processes) -> impl Expr<Ingredient<T>, Operations> {
        self.apply(Operations::Processes(p), Ident::new())
    }
}

impl<F, T> CookingExpr<T> for F where F: Cooking<T> {}

pub trait CookingAnalyzer<T> {
    fn prepare(&mut self, x: &Ingredient<T>);
    fn process(&mut self, p: &Processes);
    fn join(&mut self, left: Self, right: Self);
    fn split(&self) -> Self;
}

impl<T, V> Analyzer<Ingredient<V>, Operations> for T
where
    T: CookingAnalyzer<V>,
{
    fn value(&mut self, x: &Ingredient<V>) {
        self.prepare(x)
    }

    fn apply(
        &mut self,
        functor: &Operations,
        left: &impl Expr<Ingredient<V>, Operations>,
        right: &impl Expr<Ingredient<V>, Operations>,
    ) {
        match functor {
            Operations::Join => {
                let (mut l, mut r) = (self.split(), self.split());
                left.analyze_with(&mut l);
                right.analyze_with(&mut r);
                self.join(l, r);
            }
            Operations::Processes(x) => {
                left.analyze_with(self);
                self.process(x);
            }
        }
    }
}
