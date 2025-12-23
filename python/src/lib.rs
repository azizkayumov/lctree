use lctree::FindMax;
use lctree::LinkCutTree;
use pyo3::prelude::*;

#[pyclass(name = "LinkCutTree")]
pub struct LinkCutTreeWrapper {
    lctree: LinkCutTree<FindMax>,
}

#[pymethods]
impl LinkCutTreeWrapper {
    #[new]
    fn new() -> Self {
        let mut lctree = LinkCutTree::default();
        lctree.make_tree(0.0);
        LinkCutTreeWrapper { lctree }
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

#[pymodule]
fn _lctree(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<LinkCutTreeWrapper>()?;
    Ok(())
}
