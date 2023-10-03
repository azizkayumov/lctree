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
    // for path aggregation:
    pub weight: f64,
    pub max_weight_idx: usize,
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
            max_weight_idx: idx,
        }
    }

    pub fn to_str(&self) -> String {
        let parent = match self.parent {
            Parent::Node(idx) => format!("Node({idx})"),
            Parent::Path(idx) => format!("Path({idx})"),
            Parent::Root => "Root".to_string(),
        };
        format!(
            "Node {{ idx: {}, left: {:?}, right: {:?}, parent: {:?}, max_weight_idx: {} }}",
            self.idx, self.left, self.right, parent, self.max_weight_idx
        )
    }
}
