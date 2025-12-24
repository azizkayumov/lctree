pub trait Path: Copy + Clone {
    fn default(weight: f64, index: usize) -> Self;
    fn aggregate(&mut self, other: Self);
}

#[derive(Copy, Clone)]
pub struct FindMax {
    pub idx: usize,
    pub weight: f64,
}

impl Path for FindMax {
    fn default(weight: f64, index: usize) -> Self {
        FindMax { idx: index, weight }
    }

    fn aggregate(&mut self, other: Self) {
        if other.weight > self.weight {
            self.weight = other.weight;
            self.idx = other.idx;
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

#[derive(Copy, Clone)]
pub struct FindXor {
    pub xor: u64,
}

impl Path for FindXor {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn default(weight: f64, _: usize) -> Self {
        FindXor { xor: weight as u64 }
    }

    fn aggregate(&mut self, other: Self) {
        self.xor ^= other.xor;
    }
}
