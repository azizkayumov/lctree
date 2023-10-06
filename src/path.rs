use crate::node::Node;

pub trait Path {
    fn aggregate(&mut self, other: &Node);
}

pub struct FindMax {
    pub max_weight_idx: usize,
    pub max_weight: f64,
}

impl Path for FindMax {
    fn aggregate(&mut self, other: &Node) {
        if other.weight > self.max_weight {
            self.max_weight = other.weight;
            self.max_weight_idx = other.idx;
        }
    }
}

pub struct FindMin {
    pub min_weight_idx: usize,
    pub min_weight: f64,
}

impl Path for FindMin {
    fn aggregate(&mut self, other: &Node) {
        if other.weight < self.min_weight {
            self.min_weight = other.weight;
            self.min_weight_idx = other.idx;
        }
    }
}

pub struct FindSum {
    pub sum: f64,
}

impl Path for FindSum {
    fn aggregate(&mut self, other: &Node) {
        self.sum += other.weight;
    }
}

pub fn update_max(forest: &mut [Node], node_idx: usize) {
    let mut max_idx = node_idx;
    if let Some(left_child) = forest[node_idx].left {
        let left_max = forest[left_child].max_weight_idx;
        if forest[left_max].weight > forest[max_idx].weight {
            max_idx = left_max;
        }
    }
    if let Some(right_child) = forest[node_idx].right {
        let right_max = forest[right_child].max_weight_idx;
        if forest[right_max].weight > forest[max_idx].weight {
            max_idx = right_max;
        }
    }
    forest[node_idx].max_weight_idx = max_idx;
}
