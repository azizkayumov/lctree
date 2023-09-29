#[derive(Copy, Clone)]
pub enum Parent {
    Node(usize), // parent node in the tree
    Path(usize), // path to the root in the forest
    Root,        // root of the tree
}

pub struct Node {
    pub idx: usize,
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub parent: Parent,
    pub flipped: bool,
    pub weight: f64, // for path aggregation
}

impl Node {
    pub fn new(idx: usize, weight: f64) -> Self {
        Node {
            idx,
            left: None,
            right: None,
            parent: Parent::Root,
            flipped: false,
            weight,
        }
    }
}
