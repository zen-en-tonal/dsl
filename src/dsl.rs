pub trait Expr<T> {
    fn analyze(&self, analyzer: &mut impl Analyzer<T>);

    fn add<E>(self, other: E) -> Add<Self, E>
    where
        Self: Sized,
        E: Expr<T>,
    {
        Add(self, other)
    }

    fn apply(self, functor: T) -> Apply<T, Self>
    where
        Self: Sized,
    {
        Apply(functor, self)
    }
}

pub trait Analyzer<TValue> {
    fn value(&mut self, x: &TValue);

    fn add<TLeft, TRight>(&mut self, left: &TLeft, right: &TRight)
    where
        TLeft: Expr<TValue>,
        TRight: Expr<TValue>;

    fn apply<TExpr>(&mut self, functor: &TValue, right: &TExpr)
    where
        TExpr: Expr<TValue>;
}

pub struct Value<T>(T);

impl<T> Expr<T> for Value<T> {
    fn analyze(&self, visitor: &mut impl Analyzer<T>) {
        visitor.value(&self.0)
    }
}

pub struct Add<E1, E2>(E1, E2);

impl<T, E1, E2> Expr<T> for Add<E1, E2>
where
    E1: Expr<T>,
    E2: Expr<T>,
{
    fn analyze(&self, visitor: &mut impl Analyzer<T>) {
        visitor.add(&self.0, &self.1)
    }
}

pub struct Apply<TValue, TExpr>(TValue, TExpr);

impl<TValue, TExpr> Expr<TValue> for Apply<TValue, TExpr>
where
    TExpr: Expr<TValue>,
{
    fn analyze(&self, analyzer: &mut impl Analyzer<TValue>) {
        analyzer.apply(&self.0, &self.1)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Add;

    use super::{Analyzer, Expr, Value};

    enum NumericCategory<T> {
        Value(T),
        Func(Box<dyn Fn(T) -> T>),
    }

    struct CalcVisitor<T>(T);

    impl<T> Analyzer<NumericCategory<T>> for CalcVisitor<T>
    where
        T: Default + Copy + Add<Output = T>,
    {
        fn value(&mut self, x: &NumericCategory<T>) {
            match x {
                NumericCategory::Value(v) => self.0 = *v,
                _ => {}
            }
        }

        fn add<TLeft, TRight>(&mut self, left: &TLeft, right: &TRight)
        where
            TLeft: Expr<NumericCategory<T>>,
            TRight: Expr<NumericCategory<T>>,
        {
            let mut left_vis = CalcVisitor(T::default());
            left.analyze(&mut left_vis);
            let mut right_vis = CalcVisitor(T::default());
            right.analyze(&mut right_vis);
            self.0 = left_vis.0 + right_vis.0;
        }

        fn apply<TExpr>(&mut self, functor: &NumericCategory<T>, right: &TExpr)
        where
            TExpr: Expr<NumericCategory<T>>,
        {
            let mut vis = Self(T::default());
            right.analyze(&mut vis);
            match functor {
                NumericCategory::Func(f) => self.0 = f(vis.0),
                _ => {}
            }
        }
    }

    fn define_func<T, F>(func: F) -> NumericCategory<T>
    where
        F: Fn(T) -> T + 'static,
    {
        NumericCategory::Func(Box::new(func))
    }

    fn define_value<T>(v: T) -> Value<NumericCategory<T>> {
        Value(NumericCategory::Value(v))
    }

    #[test]
    fn i32_calc() {
        let power_2 = define_func(|x| x * x);
        let expr = define_value(2).apply(power_2).add(define_value(1));

        let mut visitor = CalcVisitor(0);
        expr.analyze(&mut visitor);

        assert_eq!(5, visitor.0);
    }
}
