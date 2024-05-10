use std::fmt::Display;

pub trait Process<T>: Sized {
    fn acquires(&self) -> impl Iterator<Item = T>;

    fn releases(&self) -> impl Iterator<Item = T>;

    fn add<P>(self, other: P) -> Add<Self, P>
    where
        P: Process<T>,
    {
        Add(self, other)
    }

    fn then<F>(self, then: F, desc: &str) -> Then<Self, F>
    where
        F: Fn(T) -> T,
    {
        Then(self, then, desc.to_string())
    }

    fn finaly(self, final_thing: T) -> Finaly<Self, T> {
        Finaly(self, final_thing)
    }
}

pub struct Add<T, Q>(T, Q);

impl<T, P1, P2> Process<T> for Add<P1, P2>
where
    P1: Process<T>,
    P2: Process<T>,
{
    fn acquires(&self) -> impl Iterator<Item = T> {
        self.0.acquires().chain(self.1.acquires())
    }

    fn releases(&self) -> impl Iterator<Item = T> {
        self.0.releases().chain(self.1.releases())
    }
}

impl<P1, P2> Display for Add<P1, P2>
where
    P1: Display,
    P2: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} and {}", self.0, self.1)
    }
}

pub struct Then<P, F>(P, F, String);

impl<T, P1, F> Process<T> for Then<P1, F>
where
    P1: Process<T>,
    F: Fn(T) -> T,
{
    fn acquires(&self) -> impl Iterator<Item = T> {
        self.0.acquires()
    }

    fn releases(&self) -> impl Iterator<Item = T> {
        self.0.releases().map(|x| self.1(x))
    }
}

impl<P1, F> Display for Then<P1, F>
where
    P1: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}. then {}", self.0, self.2)
    }
}

pub struct Finaly<P, T>(P, T);

impl<P, T> Finaly<P, T>
where
    P: Process<T>,
    T: Clone,
{
    pub fn acquires(&self) -> Vec<T> {
        self.0.acquires().collect()
    }

    pub fn releases(&self) -> T {
        self.1.clone()
    }
}

impl<P, T> Display for Finaly<P, T>
where
    P: Display,
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}. finaly {}.", self.0, self.1)
    }
}
