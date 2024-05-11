use crate::dsl::*;
use std::{collections::HashMap, fmt::Display, ops::Neg};

pub fn def(symbol: &str) -> Value<Variable, Operations> {
    Value::new(Variable::Symbol(symbol.to_string()))
}

pub fn cons(x: i32) -> Value<Variable, Operations> {
    Value::new(Variable::Value(x))
}

pub enum Variable {
    Value(i32),
    Symbol(String),
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lit = match self {
            Variable::Value(x) => x.to_string(),
            Variable::Symbol(x) => x.to_string(),
        };
        lit.fmt(f)
    }
}

pub enum Operations {
    Pow,
    Add,
    Neg,
}

pub trait Algebra {
    fn pow(self, x: impl Expr<Variable, Operations>) -> impl Expr<Variable, Operations>;

    fn add(self, x: impl Expr<Variable, Operations>) -> impl Expr<Variable, Operations>;

    fn neg(self) -> impl Expr<Variable, Operations>;

    fn sub(self, x: impl Expr<Variable, Operations>) -> impl Expr<Variable, Operations>
    where
        Self: Sized,
    {
        self.add(x.neg())
    }
}

impl<T> Algebra for T
where
    T: Expr<Variable, Operations>,
{
    fn pow(self, x: impl Expr<Variable, Operations>) -> impl Expr<Variable, Operations> {
        self.apply(Operations::Pow, x)
    }

    fn add(self, x: impl Expr<Variable, Operations>) -> impl Expr<Variable, Operations> {
        self.apply(Operations::Add, x)
    }

    fn neg(self) -> impl Expr<Variable, Operations> {
        self.apply(Operations::Neg, Ident::new())
    }
}

pub struct Calcurator {
    res: i32,
    symbols: HashMap<String, i32>,
}

impl Calcurator {
    fn new(symbols: HashMap<String, i32>) -> Calcurator {
        Calcurator { res: 0, symbols }
    }

    pub fn calc<T>(symbols: HashMap<String, i32>, expr: &T) -> i32
    where
        T: Expr<Variable, Operations>,
    {
        let mut this = Calcurator::new(symbols);
        expr.analyze_with(&mut this);
        this.res
    }
}

impl Analyzer<Variable, Operations> for Calcurator {
    fn value(&mut self, x: &Variable) {
        self.res = match x {
            Variable::Value(v) => *v,
            Variable::Symbol(s) => self.symbols[s],
        };
    }

    fn apply<TLeft, TRight>(&mut self, functor: &Operations, left: &TLeft, right: &TRight)
    where
        TLeft: Expr<Variable, Operations>,
        TRight: Expr<Variable, Operations>,
    {
        let mut left_a = Self::new(self.symbols.clone());
        let mut right_a = Self::new(self.symbols.clone());

        left.analyze_with(&mut left_a);
        right.analyze_with(&mut right_a);

        match functor {
            Operations::Pow => self.res = left_a.res.pow(right_a.res as u32),
            Operations::Add => self.res = left_a.res + right_a.res,
            Operations::Neg => self.res = left_a.res.neg(),
        }
    }
}

pub struct Displayer(String);

impl Displayer {
    pub fn to_string<T>(expr: &T) -> String
    where
        T: Expr<Variable, Operations>,
    {
        let mut this = Self(String::new());
        expr.analyze_with(&mut this);
        this.0
    }
}

impl Analyzer<Variable, Operations> for Displayer {
    fn value(&mut self, x: &Variable) {
        self.0 = format!("{}", x)
    }

    fn apply<TLeft, TRight>(&mut self, functor: &Operations, left: &TLeft, right: &TRight)
    where
        TLeft: Expr<Variable, Operations>,
        TRight: Expr<Variable, Operations>,
    {
        let mut left_a = Self(String::new());
        let mut right_a = Self(String::new());

        left.analyze_with(&mut left_a);
        right.analyze_with(&mut right_a);

        match functor {
            Operations::Pow => self.0 = format!("({} ^ {})", left_a.0, right_a.0),
            Operations::Add => self.0 = format!("({} + {})", left_a.0, right_a.0),
            Operations::Neg => self.0 = format!("(-{})", left_a.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algebra() {
        let expr = def("a")
            .add(def("b").pow(cons(3)))
            .pow(cons(2))
            .add(def("c").pow(def("x")))
            .neg();

        let res = Calcurator::calc(
            vec![
                ("a".to_string(), 2),
                ("b".to_string(), 2),
                ("c".to_string(), 2),
                ("x".to_string(), 2),
            ]
            .into_iter()
            .collect(),
            &expr,
        );
        assert_eq!(-104, res);

        assert_eq!(
            "(-(((a + (b ^ 3)) ^ 2) + (c ^ x)))".to_string(),
            Displayer::to_string(&expr)
        );
    }
}
