use super::types::{Bounds, Node, Rules};
use std::borrow::BorrowMut;
use std::collections::{BTreeSet, HashMap};

#[derive(Debug)]
pub struct Graph<N, B, R>
where
    N: Node,
    B: Bounds<N>,
    R: Rules<N>,
{
    pub generation: usize,
    pub state: BTreeSet<N>,
    pub bounds: B,
    pub rules: R,
}

impl<N, B, R> Iterator for Graph<N, B, R>
where
    N: Node,
    B: Bounds<N>,
    R: Rules<N>,
{
    type Item = (usize, BTreeSet<N>);
    fn next(&mut self) -> Option<Self::Item> {
        if self.state.is_empty() {
            return None;
        }

        *self.generation.borrow_mut() += 1;

        *self.state.borrow_mut() = self
            .state
            .iter()
            .flat_map(|node| node.neighbors())
            .filter_map(|node| self.bounds.transform(node))
            .fold(&mut HashMap::new(), |n_count, node| {
                *n_count.entry(node).or_insert(0) += 1;
                n_count
            })
            .iter()
            .filter_map(|(node, count)| self.rules.generate(&self.state, node, *count))
            .collect();

        Some((self.generation, self.state.clone()))
    }
}

impl<N, B, R> Graph<N, B, R>
where
    N: Node,
    B: Bounds<N>,
    R: Rules<N>,
{
    pub fn iter(&mut self) -> impl Iterator<Item = <Self as Iterator>::Item> + '_ {
        vec![(self.generation, self.state.clone())]
            .into_iter()
            .chain(self)
    }
}
