use lctree::LinkCutTree;
use lctree::{FindMax, FindSum, FindXor};
use pyo3::prelude::*;

#[pyclass(name = "FindMaxTree")]
pub struct FindMaxTreeWrapper {
    lctree: LinkCutTree<FindMax>,
}

#[pymethods]
impl FindMaxTreeWrapper {
    #[new]
    fn new() -> Self {
        let lctree: LinkCutTree<FindMax> = LinkCutTree::new();
        FindMaxTreeWrapper { lctree }
    }

    fn make_tree(&mut self, value: f64) -> usize {
        self.lctree.make_tree(value)
    }

    fn link(&mut self, u: usize, v: usize) -> bool {
        self.lctree.link(u, v)
    }

    fn cut(&mut self, u: usize, v: usize) -> bool {
        self.lctree.cut(u, v)
    }

    fn connected(&mut self, u: usize, v: usize) -> bool {
        self.lctree.connected(u, v)
    }

    fn find_max(&mut self, u: usize, v: usize) -> (usize, f64) {
        let max = self.lctree.path(u, v);
        (max.idx, max.weight)
    }
}

#[pyclass(name = "FindSumTree")]
pub struct FindSumTreeWrapper {
    lctree: LinkCutTree<FindSum>,
}

#[pymethods]
impl FindSumTreeWrapper {
    #[new]
    fn new() -> Self {
        let lctree: LinkCutTree<FindSum> = LinkCutTree::new();
        FindSumTreeWrapper { lctree }
    }

    fn make_tree(&mut self, value: f64) -> usize {
        self.lctree.make_tree(value)
    }

    fn link(&mut self, u: usize, v: usize) -> bool {
        self.lctree.link(u, v)
    }

    fn cut(&mut self, u: usize, v: usize) -> bool {
        self.lctree.cut(u, v)
    }

    fn connected(&mut self, u: usize, v: usize) -> bool {
        self.lctree.connected(u, v)
    }

    fn find_sum(&mut self, u: usize, v: usize) -> f64 {
        self.lctree.path(u, v).sum
    }
}

#[pyclass(name = "FindXorTree")]
pub struct FindXorTreeWrapper {
    lctree: LinkCutTree<FindXor>,
}

#[pymethods]
impl FindXorTreeWrapper {
    #[new]
    fn new() -> Self {
        let lctree: LinkCutTree<FindXor> = LinkCutTree::new();
        FindXorTreeWrapper { lctree }
    }

    fn make_tree(&mut self, value: f64) -> usize {
        self.lctree.make_tree(value)
    }

    fn link(&mut self, u: usize, v: usize) -> bool {
        self.lctree.link(u, v)
    }

    fn cut(&mut self, u: usize, v: usize) -> bool {
        self.lctree.cut(u, v)
    }

    fn connected(&mut self, u: usize, v: usize) -> bool {
        self.lctree.connected(u, v)
    }

    fn find_xor(&mut self, u: usize, v: usize) -> u64 {
        self.lctree.path(u, v).xor
    }
}

#[pymodule]
fn lctree_rs(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FindMaxTreeWrapper>()?;
    m.add_class::<FindSumTreeWrapper>()?;
    m.add_class::<FindXorTreeWrapper>()?;
    Ok(())
}
