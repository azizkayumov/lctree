use crate::path::Path;

#[derive(Copy, Clone)]
pub enum Parent {
    Node(usize), // parent node in the tree
    Path(usize), // path to the root in the forest
    Root,        // root of the tree
}

pub struct Node<T: Path> {
    pub idx: usize,
    pub left: Option<usize>,
    pub right: Option<usize>,
    pub parent: Parent,
    pub flipped: bool,
    // for path aggregation:
    pub weight: f64,
    pub path: T,
    // for deletion (the number of edges connected to this node):
    pub degree: usize,
}

impl<T: Path> Node<T> {
    pub fn new(idx: usize, weight: f64) -> Self {
        Node {
            idx,
            left: None,
            right: None,
            parent: Parent::Root,
            flipped: false,
            weight,
            path: T::default(weight, idx),
            degree: 0,
        }
    }

    pub fn flip_children(&mut self) {
        std::mem::swap(&mut self.left, &mut self.right);
    }

    #[allow(dead_code)]
    pub fn to_str(&self) -> String {
        let parent = match self.parent {
            Parent::Node(idx) => format!("Node({idx})"),
            Parent::Path(idx) => format!("Path({idx})"),
            Parent::Root => "Root".to_string(),
        };
        format!(
            "Node {{ idx: {}, left: {:?}, right: {:?}, parent: {parent:?}}}",
            self.idx, self.left, self.right
        )
    }
}
