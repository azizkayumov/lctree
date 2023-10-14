pub trait Path: Copy + Clone {
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

#[derive(Copy, Clone)]
pub struct FindMin {
    pub min_weight_idx: usize,
    pub min_weight: f64,
}

impl Path for FindMin {
    fn default(weight: f64, index: usize) -> Self {
        FindMin {
            min_weight_idx: index,
            min_weight: weight,
        }
    }

    fn aggregate(&mut self, other: Self) {
        if other.min_weight < self.min_weight {
            self.min_weight = other.min_weight;
            self.min_weight_idx = other.min_weight_idx;
        }
    }
}

#[derive(Copy, Clone)]
pub struct FindSum {
    pub sum: f64,
}

impl Path for FindSum {
    fn default(weight: f64, _: usize) -> Self {
        FindSum { sum: weight }
    }

    fn aggregate(&mut self, other: Self) {
        self.sum += other.sum;
    }
}
