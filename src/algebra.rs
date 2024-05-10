use crate::dsl::*;
use std::{collections::HashMap, fmt::Display};

pub fn def(symbol: &str) -> Value<Variable, Operations> {
    Value::new(var(symbol))
}

pub fn var(symbol: &str) -> Variable {
    Variable::Symbol(symbol.to_string())
}

pub fn cons(x: i32) -> Variable {
    Variable::Value(x)
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
    Pow(Variable),
}

pub trait Algebra {
    fn pow(self, x: Variable) -> Apply<Operations, Self>
    where
        Self: Sized;
}

impl<T> Algebra for T
where
    T: Expr<Variable, Operations>,
{
    fn pow(self, x: Variable) -> Apply<Operations, Self>
    where
        Self: Sized,
    {
        self.apply(Operations::Pow(x))
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

    fn add<TLeft, TRight>(&mut self, left: &TLeft, right: &TRight)
    where
        TLeft: Expr<Variable, Operations>,
        TRight: Expr<Variable, Operations>,
    {
        let mut left_a = Self::new(self.symbols.clone());
        let mut right_a = Self::new(self.symbols.clone());

        left.analyze_with(&mut left_a);
        right.analyze_with(&mut right_a);

        self.res = left_a.res + right_a.res
    }

    fn apply<TExpr>(&mut self, functor: &Operations, value: &TExpr)
    where
        TExpr: Expr<Variable, Operations>,
    {
        let mut a = Self::new(self.symbols.clone());
        value.analyze_with(&mut a);
        match functor {
            Operations::Pow(x) => match x {
                Variable::Value(v) => self.res = a.res.pow(*v as u32),
                Variable::Symbol(s) => self.res = a.res.pow(self.symbols[s] as u32),
            },
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

    fn add<TLeft, TRight>(&mut self, left: &TLeft, right: &TRight)
    where
        TLeft: Expr<Variable, Operations>,
        TRight: Expr<Variable, Operations>,
    {
        let mut left_a = Self(String::new());
        let mut right_a = Self(String::new());

        left.analyze_with(&mut left_a);
        right.analyze_with(&mut right_a);

        self.0 = format!("({} + {})", left_a.0, right_a.0)
    }

    fn apply<TExpr>(&mut self, functor: &Operations, value: &TExpr)
    where
        TExpr: Expr<Variable, Operations>,
    {
        let mut a = Self(String::new());
        value.analyze_with(&mut a);
        match functor {
            Operations::Pow(x) => match x {
                Variable::Value(x) => self.0 = format!("({} ^ {})", a.0, *x),
                Variable::Symbol(x) => self.0 = format!("({} ^ {})", a.0, x),
            },
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
            .add(def("c").pow(var("x")));

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
        assert_eq!(104, res);

        assert_eq!(
            "(((a + (b ^ 3)) ^ 2) + (c ^ x))".to_string(),
            Displayer::to_string(&expr)
        );
    }
}
