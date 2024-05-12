use super::*;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Nodes {
    Start(String),
    Next(String, Rc<Nodes>),
    Join(Rc<Nodes>, Rc<Nodes>),
}

#[derive(Debug, Clone)]
pub struct Instraction(Nodes);

impl Instraction {
    pub fn new() -> Instraction {
        Instraction(Nodes::Start(String::new()))
    }

    pub fn into_groups(self) -> Nodes {
        self.0
    }
}

impl<T> CookingAnalyzer<T> for Instraction
where
    T: Display,
{
    fn prepare(&mut self, x: &Ingredient<T>) {
        self.0 = Nodes::Start(format!("prepare {}", x));
    }

    fn cut(&mut self, shape: &str) {
        self.0 = Nodes::Next(format!("cut {}", shape), Rc::new(self.0.clone()));
    }

    fn stake(&mut self, until: f32) {
        self.0 = Nodes::Next(format!("stake {}", until), Rc::new(self.0.clone()));
    }

    fn stew(&mut self, until: f32) {
        self.0 = Nodes::Next(format!("stew {}", until), Rc::new(self.0.clone()));
    }

    fn join(&mut self, left: Self, right: Self) {
        self.0 = Nodes::Join(Rc::new(left.0), Rc::new(right.0));
    }

    fn split(&self) -> Self {
        Self::new()
    }
}
