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

    pub fn instraction(&self) -> String {
        let mut a = Instraction::new();
        self.inner.analyze_with(&mut a);
        a.0
    }

    pub fn needed(&self, fact: f32) -> Vec<Ingredient<T>> {
        let mut a = Required(vec![], fact);
        self.inner.analyze_with(&mut a);
        a.0
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Amount {
    Pcs(f32),
    Tsp(f32),
    Gram(f32),
    MilliIllter(f32),
    Pinch,
}

impl Mul<f32> for Amount {
    type Output = Amount;

    fn mul(self, rhs: f32) -> Self::Output {
        match self {
            Amount::Pcs(x) => Amount::Pcs(x * rhs),
            Amount::Tsp(x) => Amount::Tsp(x * rhs),
            Amount::Gram(x) => Amount::Gram(x * rhs),
            Amount::MilliIllter(x) => Amount::MilliIllter(x * rhs),
            Amount::Pinch => todo!(),
        }
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lit = match self {
            Amount::Pcs(x) => format!("{} [pcs]", x),
            Amount::Tsp(x) => format!("{} [tsp]", x),
            Amount::Gram(x) => format!("{} [g]", x),
            Amount::MilliIllter(x) => format!("{} [ml]", x),
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
        write!(f, "{} of {}s", self.amount, self.kind)
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
    Add,
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
    fn adds(self, other: impl Expr<Ingredient<T>, Process>) -> impl Expr<Ingredient<T>, Process>;
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

    fn adds(self, other: impl Expr<Ingredient<T>, Process>) -> impl Expr<Ingredient<T>, Process> {
        self.apply(Process::Add, other)
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
    fn add(&mut self, left: Self, right: Self);
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
            Process::Add => {
                let mut l = self.split();
                left.analyze_with(&mut l);

                let mut r = self.split();
                right.analyze_with(&mut r);

                self.add(l, r);
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Instraction(String, i32);

impl Instraction {
    fn new() -> Instraction {
        Instraction(String::new(), 0)
    }
}

impl<T> CookingAnalyzer<T> for Instraction
where
    T: Display,
{
    fn prepare(&mut self, x: &Ingredient<T>) {
        println!("{}: prepare {}", self.1, x)
    }

    fn cut(&mut self, shape: &str) {
        println!("{}: cut like {}", self.1, shape)
    }

    fn stake(&mut self, until: f32) {
        println!("{}: stake until {}", self.1, until)
    }

    fn stew(&mut self, until: f32) {
        println!("{}: stew until {}", self.1, until)
    }

    fn add(&mut self, left: Self, right: Self) {
        println!("{}: add {}, {}", self.1, left.1, right.1)
    }

    fn split(&self) -> Self {
        Self(String::new(), self.1 + 1)
    }
}

#[derive(Debug, Clone)]
struct Required<T>(Vec<Ingredient<T>>, f32);

impl<T> Default for Required<T> {
    fn default() -> Self {
        Self(Default::default(), Default::default())
    }
}

impl<T> CookingAnalyzer<T> for Required<T>
where
    T: Clone,
{
    fn prepare(&mut self, x: &Ingredient<T>) {
        self.0.push(Ingredient {
            amount: x.amount * self.1,
            kind: x.kind.to_owned(),
        })
    }

    fn cut(&mut self, _shape: &str) {}

    fn stake(&mut self, _until: f32) {}

    fn stew(&mut self, _until: f32) {}

    fn add(&mut self, mut left: Self, mut right: Self) {
        self.0.append(left.0.as_mut());
        self.0.append(right.0.as_mut());
    }

    fn split(&self) -> Self {
        Self(vec![], self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_curry() {
        let recipe = Recipe::new(
            prepare("onion".to_string(), Amount::Pcs(1.))
                .cuts("dice")
                .stakes(3.)
                .adds(prepare("currot".to_string(), Amount::Pcs(1.)).cuts("block"))
                .adds(prepare("potato".to_string(), Amount::Pcs(1.)).cuts("block"))
                .stakes(3.)
                .adds(prepare("water".to_string(), Amount::MilliIllter(1000.)))
                .stew(15.)
                .adds(prepare("loux".to_string(), Amount::Pcs(1.)))
                .stew(3.),
            "curry".to_string(),
        );

        println!("{}", recipe.instraction());

        for i in recipe.needed(2.) {
            println!("{}", i);
        }
    }
}
