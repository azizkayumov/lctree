use crate::node::Node;

pub trait Path {
    fn default(node: &Node) -> Self;
    fn aggregate(&mut self, other: Self);
}

#[derive(Copy, Clone)]
pub struct FindMax {
    pub max_weight_idx: usize,
    pub max_weight: f64,
}

impl Path for FindMax {
    fn default(node: &Node) -> Self {
        FindMax {
            max_weight_idx: node.idx,
            max_weight: node.weight,
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

pub fn update_max(forest: &mut [Node], node_idx: usize) {
    let mut max_idx = node_idx;
    forest[node_idx].findmax = FindMax::default(&forest[node_idx]);

    if let Some(left_child) = forest[node_idx].left {
        let left_max = forest[left_child].max_weight_idx;
        if forest[left_max].weight > forest[max_idx].weight {
            max_idx = left_max;
        }
        forest[node_idx].max_weight_idx = max_idx;
        forest[node_idx].findmax.aggregate(forest[left_child].findmax)
    }
    if let Some(right_child) = forest[node_idx].right {
        let right_max = forest[right_child].max_weight_idx;
        if forest[right_max].weight > forest[max_idx].weight {
            max_idx = right_max;
        }
        forest[node_idx].findmax.aggregate(forest[right_child].findmax)
    }
    forest[node_idx].max_weight_idx = max_idx;
    assert_eq!(forest[node_idx].max_weight_idx, forest[node_idx].findmax.max_weight_idx);
}
