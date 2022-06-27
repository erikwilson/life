pub mod paint;

use super::types::{Bounds, Node};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GridNode(pub i16, pub i16);

impl Node for GridNode {
    fn neighbors(&self) -> Vec<Self> {
        vec![
            Self(self.0 - 1, self.1 - 1),
            Self(self.0 - 1, self.1),
            Self(self.0 - 1, self.1 + 1),
            Self(self.0, self.1 - 1),
            Self(self.0, self.1 + 1),
            Self(self.0 + 1, self.1 - 1),
            Self(self.0 + 1, self.1),
            Self(self.0 + 1, self.1 + 1),
        ]
    }
}

#[derive(Debug, Clone)]
pub struct Box {
    pub min_x: i16,
    pub min_y: i16,
    pub max_x: i16,
    pub max_y: i16,
}

#[derive(Debug, Clone)]
pub struct WrapBounds(pub Box);

impl Bounds<GridNode> for WrapBounds {
    fn transform(&self, mut node: GridNode) -> Option<GridNode> {
        if node.0 < self.0.min_x {
            node.0 = self.0.max_x;
        }
        if node.0 > self.0.max_x {
            node.0 = self.0.min_x;
        }
        if node.1 < self.0.min_y {
            node.1 = self.0.max_y;
        }
        if node.1 > self.0.max_y {
            node.1 = self.0.min_y;
        }
        Some(node)
    }
}

pub struct OutOfBounds(pub Box);

impl Bounds<GridNode> for OutOfBounds {
    fn transform(&self, node: GridNode) -> Option<GridNode> {
        match node.0 < self.0.min_x
            || node.0 > self.0.max_x
            || node.1 < self.0.min_y
            || node.1 > self.0.max_y
        {
            true => None,
            false => Some(node),
        }
    }
}
