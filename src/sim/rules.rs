use super::types::{Node, Rules};
use std::collections::BTreeSet;

pub struct ConwayRules {}

impl<N> Rules<N> for ConwayRules
where
    N: Node,
{
    fn generate(&self, state: &BTreeSet<N>, node: &N, count: u8) -> Option<N> {
        match (count == 3) || (count == 2 && state.contains(node)) {
            true => Some(node.clone()),
            false => None,
        }
    }
}
