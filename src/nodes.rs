use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub probability: f32,
    pub kind: NodeKind,
}

impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.probability.partial_cmp(&self.probability).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug, PartialEq, PartialOrd)]
pub enum NodeKind {
    Internal { left: Box<Node>, right: Box<Node> },
    Leaf { symbol: char },
}
