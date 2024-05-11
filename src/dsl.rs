use std::marker::PhantomData;

pub trait Expr<TValue, TFunctor> {
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

    fn apply<TLeft, TRight>(&mut self, functor: &TFunctor, left: &TLeft, right: &TRight)
    where
        TLeft: Expr<TValue, TFunctor>,
        TRight: Expr<TValue, TFunctor>;
}

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

pub struct Apply<TFunctor, TLeft, TRight>(TFunctor, TLeft, TRight);

impl<TValue, TFunctor, TLeft, TRight> Expr<TValue, TFunctor> for Apply<TFunctor, TLeft, TRight>
where
    TLeft: Expr<TValue, TFunctor>,
    TRight: Expr<TValue, TFunctor>,
{
    fn analyze_with(&self, analyzer: &mut impl Analyzer<TValue, TFunctor>) {
        analyzer.apply(&self.0, &self.1, &self.2)
    }
}
