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
