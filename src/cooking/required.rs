use super::*;
use std::vec;

#[derive(Debug, Clone)]
pub struct Required<T>(Vec<Ingredient<T>>);

impl<T> Required<T> {
    pub fn new() -> Required<T> {
        Required(vec![])
    }
}

impl<T> IntoIterator for Required<T> {
    type Item = Ingredient<T>;

    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> CookingAnalyzer<T> for Required<T>
where
    T: Clone,
{
    fn prepare(&mut self, x: &Ingredient<T>) {
        self.0.push(Ingredient {
            amount: x.amount,
            kind: x.kind.to_owned(),
        })
    }

    fn join(&mut self, mut left: Self, mut right: Self) {
        self.0.append(left.0.as_mut());
        self.0.append(right.0.as_mut());
    }

    fn split(&self) -> Self {
        Self(vec![])
    }

    fn process(&mut self, _p: &Processes) {}
}
