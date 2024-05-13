use super::*;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Tree<T> {
    Leaf(T),
    Branch(Processes, Rc<Tree<T>>),
    Node(Rc<Tree<T>>, Rc<Tree<T>>),
}

struct Parser<T>(Option<Tree<T>>);

impl<T> Parser<T> {
    fn new() -> Parser<T> {
        Parser(None)
    }
}

impl<T> CookingAnalyzer<T> for Parser<Ingredient<T>>
where
    T: Clone,
{
    fn prepare(&mut self, x: &Ingredient<T>) {
        self.0 = Some(Tree::Leaf(x.clone()))
    }

    fn join(&mut self, left: Self, right: Self) {
        self.0 = Some(Tree::Node(
            Rc::new(left.0.unwrap()),
            Rc::new(right.0.unwrap()),
        ));
    }

    fn process(&mut self, p: &Processes) {
        self.0 = Some(Tree::Branch(p.clone(), Rc::new(self.0.take().unwrap())))
    }

    fn split(&self) -> Self {
        Self::new()
    }
}

pub fn parse<T>(r: &impl Expr<Ingredient<T>, Operations>) -> Tree<Ingredient<T>>
where
    T: Clone,
{
    let mut a = Parser::new();
    r.analyze_with(&mut a);
    a.0.unwrap()
}
