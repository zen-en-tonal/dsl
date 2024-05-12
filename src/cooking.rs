use serde::{Deserialize, Serialize};

use crate::dsl::*;
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

    pub fn instraction(&self) -> Vec<String> {
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

struct Instraction(Vec<String>);

impl Instraction {
    fn new() -> Instraction {
        Instraction(vec![])
    }
}

impl<T> Analyzer<Ingredient<T>, Process> for Instraction
where
    T: Display,
{
    fn value(&mut self, x: &Ingredient<T>) {
        self.0.push(format!("prepare {}", x))
    }

    fn apply<TLeft, TRight>(&mut self, functor: &Process, left: &TLeft, right: &TRight)
    where
        TLeft: Expr<Ingredient<T>, Process>,
        TRight: Expr<Ingredient<T>, Process>,
    {
        let mut left_a = Self::new();
        let mut right_a = Self::new();

        left.analyze_with(&mut left_a);
        right.analyze_with(&mut right_a);

        match functor {
            Process::Cuts(shapes) => {
                self.0.append(&mut left_a.0);
                self.0.push(format!("cuts it like {}", shapes));
            }
            Process::Stakes(fo) => {
                self.0.append(&mut left_a.0);
                self.0.push(format!("stakes it for {} min", fo));
            }
            Process::Fries(_) => todo!(),
            Process::Boils(_) => todo!(),
            Process::Waits(_) => todo!(),
            Process::Stew(f) => {
                self.0.append(&mut left_a.0);
                self.0.push(format!("stews it for {} min", f));
            }
            Process::Add => {
                self.0.append(
                    &mut left_a
                        .0
                        .into_iter()
                        .map(|x| format!("| {}", x))
                        .collect::<Vec<String>>(),
                );
                self.0.append(
                    &mut right_a
                        .0
                        .into_iter()
                        .map(|x| format!("| {}", x))
                        .collect::<Vec<String>>(),
                );
            }
        }
    }
}

struct Required<T>(Vec<Ingredient<T>>, f32);

impl<T> Analyzer<Ingredient<T>, Process> for Required<T>
where
    T: Clone,
{
    fn value(&mut self, x: &Ingredient<T>) {
        self.0.push(Ingredient {
            amount: x.amount * self.1,
            kind: x.kind.to_owned(),
        })
    }

    fn apply<TLeft, TRight>(&mut self, _functor: &Process, left: &TLeft, right: &TRight)
    where
        TLeft: Expr<Ingredient<T>, Process>,
        TRight: Expr<Ingredient<T>, Process>,
    {
        let mut left_a = Self(vec![], self.1);
        let mut right_a = Self(vec![], self.1);

        left.analyze_with(&mut left_a);
        right.analyze_with(&mut right_a);

        self.0.append(&mut left_a.0);
        self.0.append(&mut right_a.0);
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

        println!("{:#?}", recipe.instraction());

        for i in recipe.needed(2.) {
            println!("{}", i);
        }

        let json = serde_json::to_string_pretty(&recipe).unwrap();
        println!("{}", json)
    }
}
