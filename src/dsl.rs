use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub trait Expr<TValue, TFunctor>: Serialize + for<'a> Deserialize<'a> {
    fn analyze_with(&self, analyzer: &mut impl Analyzer<TValue, TFunctor>);

    fn apply<TExpr>(self, functor: TFunctor, other: TExpr) -> Apply<TFunctor, Self, TExpr>
    where
        Self: Sized,
        TExpr: Expr<TValue, TFunctor>,
    {
        Apply(functor, self, other)
    }
}

pub trait Analyzer<TValue, TFunctor> {
    fn value(&mut self, x: &TValue);

    fn ident(&mut self) {}

    fn apply(
        &mut self,
        functor: &TFunctor,
        left: &impl Expr<TValue, TFunctor>,
        right: &impl Expr<TValue, TFunctor>,
    );
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Ident<T, F>(PhantomData<T>, PhantomData<F>);

impl<T, F> Ident<T, F> {
    pub fn new() -> Ident<T, F> {
        Ident(PhantomData, PhantomData)
    }
}

impl<T, F> Expr<T, F> for Ident<T, F> {
    fn analyze_with(&self, analyzer: &mut impl Analyzer<T, F>) {
        analyzer.ident()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Value<T, F>(T, PhantomData<F>);

impl<T, F> Value<T, F> {
    pub fn new(value: T) -> Value<T, F> {
        Value(value, PhantomData)
    }
}

impl<T, F> Expr<T, F> for Value<T, F>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    fn analyze_with(&self, visitor: &mut impl Analyzer<T, F>) {
        visitor.value(&self.0)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Apply<TFunctor, TLeft, TRight>(TFunctor, TLeft, TRight);

impl<TValue, TFunctor, TLeft, TRight> Expr<TValue, TFunctor> for Apply<TFunctor, TLeft, TRight>
where
    TLeft: Expr<TValue, TFunctor> + Serialize + for<'de> Deserialize<'de>,
    TRight: Expr<TValue, TFunctor> + Serialize + for<'de> Deserialize<'de>,
    TFunctor: Serialize + for<'de> Deserialize<'de>,
{
    fn analyze_with(&self, analyzer: &mut impl Analyzer<TValue, TFunctor>) {
        analyzer.apply(&self.0, &self.1, &self.2)
    }
}
