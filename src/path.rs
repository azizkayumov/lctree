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
