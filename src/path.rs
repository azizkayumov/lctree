use crate::node::Node;

pub trait Path {
    fn default(weight: f64, index: usize) -> Self;
    fn aggregate(&mut self, other: Self);
}

#[derive(Copy, Clone)]
pub struct FindMax {
    pub max_weight_idx: usize,
    pub max_weight: f64,
}

impl Path for FindMax {
    fn default(weight: f64, index: usize) -> Self {
        FindMax {
            max_weight_idx: index,
            max_weight: weight,
        }
    }

    fn aggregate(&mut self, other: Self) {
        if other.max_weight > self.max_weight {
            self.max_weight = other.max_weight;
            self.max_weight_idx = other.max_weight_idx;
        }
    }
}

pub struct FindMin {
    pub min_weight_idx: usize,
    pub min_weight: f64,
}

pub fn update<T: Path + Copy + Clone>(forest: &mut [Node<T>], node_idx: usize) {
    forest[node_idx].path = T::default(forest[node_idx].weight, node_idx);
    if let Some(left_child) = forest[node_idx].left {
        forest[node_idx].path.aggregate(forest[left_child].path);
    }
    if let Some(right_child) = forest[node_idx].right {
        forest[node_idx].path.aggregate(forest[right_child].path);
    }
}
