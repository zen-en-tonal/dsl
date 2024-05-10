use std::marker::PhantomData;

pub trait Expr<TValue, TFunctor> {
    fn analyze_with(&self, analyzer: &mut impl Analyzer<TValue, TFunctor>);

    fn add<TExpr, F>(self, other: TExpr) -> Add<Self, TExpr>
    where
        Self: Sized,
        TExpr: Expr<TValue, F>,
    {
        Add(self, other)
    }

    fn apply(self, functor: TFunctor) -> Apply<TFunctor, Self>
    where
        Self: Sized,
    {
        Apply(functor, self)
    }
}

pub trait Analyzer<TValue, TFunctor> {
    fn value(&mut self, x: &TValue);

    fn add<TLeft, TRight>(&mut self, left: &TLeft, right: &TRight)
    where
        TLeft: Expr<TValue, TFunctor>,
        TRight: Expr<TValue, TFunctor>;

    fn apply<TExpr>(&mut self, functor: &TFunctor, value: &TExpr)
    where
        TExpr: Expr<TValue, TFunctor>;
}

pub struct Value<T, F>(T, PhantomData<F>);

impl<T, F> Value<T, F> {
    pub fn new(value: T) -> Value<T, F> {
        Value(value, PhantomData)
    }
}

impl<T, F> Expr<T, F> for Value<T, F> {
    fn analyze_with(&self, visitor: &mut impl Analyzer<T, F>) {
        visitor.value(&self.0)
    }
}

pub struct Add<E1, E2>(E1, E2);

impl<T, F, E1, E2> Expr<T, F> for Add<E1, E2>
where
    E1: Expr<T, F>,
    E2: Expr<T, F>,
{
    fn analyze_with(&self, visitor: &mut impl Analyzer<T, F>) {
        visitor.add(&self.0, &self.1)
    }
}

pub struct Apply<TFunctor, TExpr>(TFunctor, TExpr);

impl<TValue, TFunctor, TExpr> Expr<TValue, TFunctor> for Apply<TFunctor, TExpr>
where
    TExpr: Expr<TValue, TFunctor>,
{
    fn analyze_with(&self, analyzer: &mut impl Analyzer<TValue, TFunctor>) {
        analyzer.apply(&self.0, &self.1)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{Analyzer, Expr, Value};

    enum NumericFunctor {
        Pow(i32),
    }

    struct Calcurator {
        res: i32,
        symbols: HashMap<String, i32>,
    }

    impl Analyzer<String, NumericFunctor> for Calcurator {
        fn value(&mut self, x: &String) {
            self.res = self.symbols[x];
        }

        fn add<TLeft, TRight>(&mut self, left: &TLeft, right: &TRight)
        where
            TLeft: Expr<String, NumericFunctor>,
            TRight: Expr<String, NumericFunctor>,
        {
            let mut left_a = Self {
                res: 0,
                symbols: self.symbols.clone(),
            };
            let mut right_a = Self {
                res: 0,
                symbols: self.symbols.clone(),
            };

            left.analyze_with(&mut left_a);
            right.analyze_with(&mut right_a);

            self.res = left_a.res + right_a.res
        }

        fn apply<TExpr>(&mut self, functor: &NumericFunctor, value: &TExpr)
        where
            TExpr: Expr<String, NumericFunctor>,
        {
            let mut a = Self {
                res: 0,
                symbols: self.symbols.clone(),
            };
            value.analyze_with(&mut a);
            match functor {
                NumericFunctor::Pow(x) => self.res = a.res.pow(*x as u32),
            }
        }
    }

    #[test]
    fn i32_calc() {
        // (a + b ** 3 ) ** 2
        let expr = Value::<String, NumericFunctor>::new("a".to_string())
            .add(Value::new("b".to_string()).apply(NumericFunctor::Pow(3)))
            .apply(NumericFunctor::Pow(2));

        let mut visitor = Calcurator {
            res: 0,
            symbols: vec![("a".to_string(), 2), ("b".to_string(), 2)]
                .into_iter()
                .collect(),
        };
        expr.analyze_with(&mut visitor);

        assert_eq!(100, visitor.res);
    }
}
