use crate::{
    path::{FindMax, Path},
    splay::Forest,
};

pub struct LinkCutTree<P: Path> {
    forest: Forest<P>,
}

impl<P: Path> LinkCutTree<P> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            forest: Forest::new(),
        }
    }

    /// Creates a new tree with a single node with the given weight.
    /// Returns the id of the node.
    pub fn make_tree(&mut self, weight: f64) -> usize {
        self.forest.create_node(weight)
    }

    /// Delete a tree from the forest
    /// # Panics
    ///
    /// Panics if the tree contains more than one node.
    pub fn remove_tree(&mut self, idx: usize) {
        self.forest.delete_node(idx);
    }

    /// Constructs a path from a node to the root of the tree.
    fn access(&mut self, v: usize) {
        self.forest.splay(v);
        self.forest.remove_preferred_child(v);

        while let Some(path_idx) = self.forest.path_parent_of(v) {
            self.forest.splay(path_idx);
            self.forest.remove_preferred_child(path_idx);

            self.forest.set_right(path_idx, v);
            self.forest.splay(v); // just a rotation
        }
    }

    /// Makes v the root of its represented tree by flipping the path from v to the root.
    fn reroot(&mut self, v: usize) {
        self.access(v);
        self.forest.flip(v);
    }

    /// Checks if v and w are connected in the forest.
    pub fn connected(&mut self, v: usize, w: usize) -> bool {
        self.reroot(v); // v is now the root of the tree
        self.access(w);
        // if access(w) messed with the root of the tree, then v and w are connected:
        self.forest.parent_of(v).is_some() || v == w
    }

    /// Creates a link between two nodes in the forest (where w is the parent of v).
    pub fn link(&mut self, v: usize, w: usize) {
        if self.connected(v, w) {
            return;
        }
        // v is the root of its represented tree, so no need to check if it has a left child
        self.forest.set_left(v, w);
    }

    /// Cuts the link between nodes v and w (if it exists)
    pub fn cut(&mut self, v: usize, w: usize) {
        if !self.connected(v, w) {
            return;
        }
        // detach w from its parent (which is v)
        if let Some(left) = self.forest.left_of(w) {
            if left != v || self.forest.right_of(v).is_some() {
                // maybe this should be a panic?
                // eprintln!("Error: no link between {v} and {w}");
                return;
            }
            self.forest.cut_left(w);
        }
    }

    /// Performs path aggregation on a path between v and w (if they are connected)
    pub fn path(&mut self, v: usize, w: usize) -> P {
        if !self.connected(v, w) {
            return P::default(f64::INFINITY, usize::MAX);
        }
        self.forest.aggregated_path_of(w)
    }

    /// Finds the root of the tree that v is in.
    pub fn findroot(&mut self, v: usize) -> usize {
        self.access(v);
        let mut root = v;
        while let Some(left) = self.forest.left_of(root) {
            root = left;
        }
        self.forest.splay(root); // fast access to the root next time
        root
    }
}

impl Default for LinkCutTree<FindMax> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{FindMin, FindSum, LinkCutTree};

    #[test]
    pub fn link_cut() {
        // We form a link-cut tree from the following rooted tree:
        //     0
        //    / \
        //   1   6
        //  / \   \
        // 2   3   7
        //    / \   \
        //   4   5   8
        //          /
        //         9
        let mut lctree = super::LinkCutTree::default();
        for i in 0..10 {
            lctree.make_tree(i as f64);
        }
        lctree.link(1, 0);
        lctree.link(2, 1);
        lctree.link(3, 1);
        lctree.link(4, 3);
        lctree.link(5, 3);
        lctree.link(6, 0);
        lctree.link(7, 6);
        lctree.link(8, 7);
        lctree.link(9, 8);

        // Checking connectivity:
        for i in 0..10 {
            for j in 0..10 {
                assert!(lctree.connected(i, j));
            }
        }

        // We cut node 6 from its parent 0:
        lctree.cut(6, 0);

        // The forest should now look like this:
        //         0
        //        /
        //       1        6
        //      / \        \
        //     2   3        7
        //        / \        \
        //       4   5        8
        //                   /
        //                  9

        // We check connectivity again for the two trees:
        for i in 0..6 {
            for j in 0..6 {
                assert!(lctree.connected(i, j));
            }
        }
        for i in 6..10 {
            for j in 6..10 {
                assert!(lctree.connected(i, j));
            }
        }
        for i in 0..6 {
            for j in 6..10 {
                assert!(!lctree.connected(i, j));
            }
        }
    }

    #[test]
    pub fn cut_non_existing_edge() {
        // We form a link-cut tree from the following rooted tree:
        //     0
        //    / \
        //   1   2
        //       |
        //       3
        let mut lctree = super::LinkCutTree::default();
        for i in 0..4 {
            lctree.make_tree(i as f64);
        }
        lctree.link(1, 0);
        lctree.link(2, 0);
        lctree.link(3, 2);

        // Try to cut non-existing edge:
        lctree.cut(1, 3); // should do nothing

        // They should still be connected:
        assert!(lctree.connected(1, 3));
    }

    #[test]
    pub fn findroot() {
        // We form a link-cut tree from the following rooted tree:
        //     0
        //    / \
        //   1   6
        //  / \   \
        // 2   3   7
        //    / \   \
        //   4   5   8
        //          /
        //         9
        let mut lctree = super::LinkCutTree::default();
        for i in 0..10 {
            lctree.make_tree(i as f64);
        }
        lctree.link(1, 0);
        lctree.link(2, 1);
        lctree.link(3, 1);
        lctree.link(4, 3);
        lctree.link(5, 3);
        lctree.link(6, 0);
        lctree.link(7, 6);
        lctree.link(8, 7);
        lctree.link(9, 8);

        // Checking findroot:
        for i in 0..10 {
            assert_eq!(lctree.findroot(i), 0);
        }

        // We cut node 6 from its parent 0:
        lctree.cut(6, 0);

        // the forest should now look like this:
        //         0
        //        /
        //       1        6
        //      / \        \
        //     2   3        7
        //        / \        \
        //       4   5        8
        //                   /
        //                  9

        // We check findroot again for the two trees:
        for i in 0..6 {
            assert_eq!(lctree.findroot(i), 0);
        }

        for i in 6..10 {
            assert_eq!(lctree.findroot(i), 6);
        }
    }

    #[test]
    pub fn reroot() {
        // We form a link-cut tree from the following rooted tree:
        //     0
        //    / \
        //   1   4
        //  / \
        // 2   3
        let mut lctree = super::LinkCutTree::default();
        for i in 0..5 {
            lctree.make_tree(i as f64);
        }
        lctree.link(1, 0);
        lctree.link(2, 1);
        lctree.link(3, 1);
        lctree.link(4, 0);

        // Checking findroot (which should be 0 for all nodes):
        for i in 0..5 {
            assert_eq!(lctree.findroot(i), 0);
        }

        // we make 1 the root of the tree:
        lctree.reroot(1);

        for i in 0..5 {
            assert_eq!(lctree.findroot(i), 1);
        }
    }

    #[test]
    pub fn findmax() {
        // We form a link-cut tree from the following rooted tree
        // (the numbers in parentheses are the weights of the nodes):
        //           0(9)
        //           /  \
        //         1(1)  5(2)
        //        /   \    \
        //      2(8)  3(0) 6(4)
        //      /             \
        //    4(6)            7(3)
        //                    /  \
        //                  8(7) 9(5)
        let mut lctree = super::LinkCutTree::default();
        let weights = [9.0, 1.0, 8.0, 0.0, 6.0, 2.0, 4.0, 3.0, 7.0, 5.0];
        for i in 0..weights.len() {
            lctree.make_tree(weights[i]);
        }
        lctree.link(1, 0);
        lctree.link(2, 1);
        lctree.link(3, 1);
        lctree.link(4, 2);
        lctree.link(5, 0);
        lctree.link(6, 5);
        lctree.link(7, 6);
        lctree.link(8, 7);
        lctree.link(9, 7);

        // We check the node index with max weight in the path from each node to the root:
        assert_eq!(lctree.path(4, 5).max_weight_idx, 0);
        assert_eq!(lctree.path(3, 6).max_weight_idx, 0);
        assert_eq!(lctree.path(2, 7).max_weight_idx, 0);
        assert_eq!(lctree.path(1, 8).max_weight_idx, 0);
        assert_eq!(lctree.path(0, 9).max_weight_idx, 0);
        assert_eq!(lctree.path(4, 3).max_weight_idx, 2);
        assert_eq!(lctree.path(5, 7).max_weight_idx, 6);
    }

    #[test]
    pub fn findmin() {
        // We form a link-cut tree from the following rooted tree
        // (the numbers in parentheses are the weights of the nodes):
        //           0(9)
        //           /  \
        //         1(1)  5(2)
        //        /   \    \
        //      2(8)  3(0) 6(4)
        //      /             \
        //    4(6)            7(3)
        //                    /  \
        //                  8(7) 9(5)
        let mut lctree: LinkCutTree<FindMin> = super::LinkCutTree::new();
        let weights = [9.0, 1.0, 8.0, 0.0, 6.0, 2.0, 4.0, 3.0, 7.0, 5.0];
        for i in 0..weights.len() {
            lctree.make_tree(weights[i]);
        }
        lctree.link(1, 0);
        lctree.link(2, 1);
        lctree.link(3, 1);
        lctree.link(4, 2);
        lctree.link(5, 0);
        lctree.link(6, 5);
        lctree.link(7, 6);
        lctree.link(8, 7);
        lctree.link(9, 7);

        // We check the node index with max weight in the path from each node to the root:
        assert_eq!(lctree.path(4, 5).min_weight_idx, 1);
        assert_eq!(lctree.path(3, 6).min_weight_idx, 3);
        assert_eq!(lctree.path(2, 7).min_weight_idx, 1);
        assert_eq!(lctree.path(1, 8).min_weight_idx, 1);
        assert_eq!(lctree.path(0, 9).min_weight_idx, 5);
        assert_eq!(lctree.path(4, 3).min_weight_idx, 3);
        assert_eq!(lctree.path(5, 7).min_weight_idx, 5);
    }

    #[test]
    pub fn findsum() {
        // We form a link-cut tree from the following rooted tree
        // (the numbers in parentheses are the weights of the nodes):
        //           0(9)
        //           /  \
        //         1(1)  5(2)
        //        /   \    \
        //      2(8)  3(0) 6(4)
        //      /             \
        //    4(6)            7(3)
        //                    /  \
        //                  8(7) 9(5)
        let mut lctree: LinkCutTree<FindSum> = super::LinkCutTree::new();
        let weights = [9.0, 1.0, 8.0, 0.0, 6.0, 2.0, 4.0, 3.0, 7.0, 5.0];
        for i in 0..weights.len() {
            lctree.make_tree(weights[i]);
        }
        lctree.link(1, 0);
        lctree.link(2, 1);
        lctree.link(3, 1);
        lctree.link(4, 2);
        lctree.link(5, 0);
        lctree.link(6, 5);
        lctree.link(7, 6);
        lctree.link(8, 7);
        lctree.link(9, 7);

        // We check the node index with max weight in the path from each node to the root:
        assert_eq!(lctree.path(4, 5).sum, 26.);
        assert_eq!(lctree.path(3, 6).sum, 16.);
        assert_eq!(lctree.path(2, 7).sum, 27.);
        assert_eq!(lctree.path(1, 8).sum, 26.);
        assert_eq!(lctree.path(0, 9).sum, 23.);
        assert_eq!(lctree.path(4, 3).sum, 15.);
        assert_eq!(lctree.path(5, 7).sum, 9.);
    }
}
