#[derive(Copy, Clone)]
pub enum Parent {
    Node(usize), // parent node in the tree
    Path(usize), // path to the root in the forest
    Root,        // root of the tree
}

pub struct Path {
    pub max_weight_idx: usize,
    pub max_weight: f64,
}

impl Path {
    pub fn new(max_weight_idx: usize, max_weight: f64) -> Self {
        Path {
            max_weight_idx,
            max_weight,
        }
    }

    pub fn update(&mut self, idx: usize, weight: f64) {
        if weight > self.max_weight {
            self.max_weight = weight;
            self.max_weight_idx = idx;
        }
    }
}

pub struct Node {
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub parent: Parent,
    pub flipped: bool,
    // path aggregates
    pub weight: f64,
    pub path: Path,
}

impl Node {
    pub fn new(idx: usize, weight: f64) -> Self {
        Node {
            left: None,
            right: None,
            parent: Parent::Root,
            flipped: false,
            weight: 0.0,
            path: Path::new(idx, weight),
        }
    }
}
