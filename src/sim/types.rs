use std::collections::BTreeSet;
use std::hash::Hash;

pub trait Node: Clone + Eq + Hash + Ord {
    fn neighbors(&self) -> Vec<Self>;
}

pub trait Bounds<N: Node> {
    fn transform(&self, node: N) -> Option<N>;
}

pub trait Rules<N: Node> {
    fn generate(&self, state: &BTreeSet<N>, node: &N, count: u8) -> Option<N>;
}
