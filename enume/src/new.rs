#[derive(Debug)]
pub struct Pair<F, S> {
    first: F,
    second: S,
}

impl<F, S> Pair<F, S> {
    pub fn new(first: F, second: S) -> Self {
        Pair { first, second }
    }

    pub fn swap(self) -> Pair<S, F> {
        Pair::new(self.second, self.first)
    }
}
