pub enum Process<T> {
    Add(Box<Add<T>>),
    Then(Box<Then<T>>),
    Finaly(Box<Finaly<T>>),
}
pub struct Add<T>(Process<T>, Process<T>);

pub struct Then<T>(Process<T>);

pub struct Finaly<T>(Process<T>, T);

#[cfg(test)]
mod tests {
    use crate::block_diagram::start;

    #[test]
    fn sample() {
        let a = start(0, "prepare a").then((|x| x + 1, "plus one"));
        let b = start(1, "prepare b");
        let y = a.marge(b).then((|c| c * 10, "mul 10"));

        let y = y.into_iter();

        assert_eq!(y.next(), Some("prepare a"));
        assert_eq!(y.next(), Some("plus one"));
        assert_eq!(y.next(), Some("prepare b"));
        assert_eq!(y.next(), Some("a add b"));
        assert_eq!(y.next(), Some("mul 10"));
        assert_eq!(y.next(), None);
    }
}
