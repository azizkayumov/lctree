use lctree::{LinkCutTree, Path};

#[derive(Copy, Clone)]
pub struct FindXor {
    pub xor: u64,
}

impl Path for FindXor {
    fn default(weight: f64, _: usize) -> Self {
        FindXor { xor: weight as u64 }
    }

    fn aggregate(&mut self, other: Self) {
        self.xor ^= other.xor;
    }
}

#[test]
pub fn custom_path_aggregation() {
    // We form a link-cut tree from the following rooted tree
    // (the numbers in parentheses are the weights of the nodes):
    //           a(9)
    //           /  \
    //         b(1)  e(2)
    //        /   \    \
    //      c(8)  d(10)  f(4)
    let mut lctree: LinkCutTree<FindXor> = LinkCutTree::new();
    let a = lctree.make_tree(9.);
    let b = lctree.make_tree(1.);
    let c = lctree.make_tree(8.);
    let d = lctree.make_tree(10.);
    let e = lctree.make_tree(2.);
    let f = lctree.make_tree(4.);

    lctree.link(b, a);
    lctree.link(c, b);
    lctree.link(d, b);
    lctree.link(e, a);
    lctree.link(f, e);

    // We find the xor of the weights on the path between c to f,
    let result = lctree.path(c, f);
    assert_eq!(result.xor, 8 ^ 1 ^ 9 ^ 2 ^ 4);
}
