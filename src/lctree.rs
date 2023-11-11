use crate::{
    path::{FindMax, Path},
    splay::Forest,
};

pub struct LinkCutTree<P: Path> {
    forest: Forest<P>,
}

/// # Link-cut-tree.
/// A self-balancing data structure to maintain a dynamic forest of (un)rooted trees
/// under the following operations that take `O(logn)` amortized time:
/// - `link(v, w)`: creates an edge between nodes `v` and `w`.
/// - `cut(v, w)`: removes the edge between nodes `v` and `w`.
/// - `connected(v, w)`: returns `true` if nodes `v` and `w` are in the same tree.
/// - `path(v, w)`: performs calculations on a path between nodes `v` and `w`.
///
/// # Examples
///
/// ```
/// use lctree::LinkCutTree;
///
/// // We form a link-cut tree for the following forest:
/// // (the numbers in parentheses are the weights of the nodes):
/// //            a(9)
/// //           /    \
/// //         b(1)    e(2)
/// //        /   \      \
/// //      c(8)  d(10)   f(4)
/// let mut lctree = LinkCutTree::default();
/// let a = lctree.make_tree(9.);
/// let b = lctree.make_tree(1.);
/// let c = lctree.make_tree(8.);
/// let d = lctree.make_tree(10.);
/// let e = lctree.make_tree(2.);
/// let f = lctree.make_tree(4.);
///
/// lctree.link(b, a);
/// lctree.link(c, b);
/// lctree.link(d, b);
/// lctree.link(e, a);
/// lctree.link(f, e);
///
/// // Checking connectivity:
/// assert!(lctree.connected(c, f)); // connected
///
/// // Path aggregation:
/// // We find the node with max weight on the path between c to f,
/// // where a has the maximum weight of 9.0:
/// let heaviest_node = lctree.path(c, f);
/// assert_eq!(heaviest_node.idx, a);
/// assert_eq!(heaviest_node.weight, 9.0);
///
/// // We cut node e from its parent a:
/// lctree.cut(e, a);
///
/// // The forest should now look like this:
/// //            a(9)
/// //           /    
/// //         b(1)      e(2)
/// //        /   \        \
/// //      c(8)  d(10)    f(4)
///
/// // We check connectivity again:
/// assert!(!lctree.connected(c, f)); // not connected anymore
/// ```
impl<P: Path> LinkCutTree<P> {
    /// Creates a new empty link-cut tree.
    #[must_use]
    pub fn new() -> Self {
        Self {
            forest: Forest::new(),
        }
    }

    /// Creates a new tree with a single node with the given weight and returns its id.
    /// If possible, reuses the space of a deleted node and returns its id.
    ///
    /// # Examples
    /// ```
    /// use lctree::LinkCutTree;
    ///
    /// let mut lctree = LinkCutTree::default();
    /// let alice = lctree.make_tree(0.0);
    /// let bob = lctree.make_tree(1.0);
    /// let clay = lctree.make_tree(2.0);
    /// assert_eq!([alice, bob, clay], [0, 1, 2]);
    ///
    /// // Remove bob's tree from the forest
    /// lctree.remove_tree(bob);
    ///
    /// // Reuse the space of bob's tree (which was removed) to create a new tree:
    /// let david = lctree.make_tree(4.0);
    /// assert_eq!(david, bob);
    /// ```
    pub fn make_tree(&mut self, weight: f64) -> usize {
        self.forest.create_node(weight)
    }

    /// Extends the forest with n new single-noded trees for the given weights.
    ///
    /// # Examples
    ///
    /// ```
    /// use lctree::LinkCutTree;
    ///
    /// let weights = vec![1.0, 2.0, 3.0];
    /// let mut lctree = LinkCutTree::default();
    /// let trees_ids = lctree.extend_forest(&weights);
    /// assert_eq!(trees_ids, vec![0, 1, 2]);
    /// ```
    #[must_use]
    pub fn extend_forest(&mut self, weights: &[f64]) -> Vec<usize> {
        weights
            .iter()
            .map(|&weight| self.make_tree(weight))
            .collect()
    }

    /// Delete a tree with a single node with the given id.
    ///
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

    /// Checks if two nodes are connected (i.e. in the same tree).
    ///
    /// # Examples
    /// ```
    /// use lctree::LinkCutTree;
    ///
    /// let mut lctree = LinkCutTree::default();
    /// let alice = lctree.make_tree(0.0);
    /// let bob = lctree.make_tree(1.0);
    /// assert!(!lctree.connected(alice, bob)); // not connected yet
    ///
    /// lctree.link(alice, bob);
    /// assert!(lctree.connected(alice, bob)); // now connected
    /// ```
    pub fn connected(&mut self, v: usize, w: usize) -> bool {
        self.reroot(v); // v is now the root of the tree
        self.access(w);
        // if access(w) messed with the root of the tree, then v and w are connected:
        self.forest.parent_of(v).is_some() || v == w
    }

    /// Merges two trees into a single tree.
    ///
    /// # Examples
    /// ```
    /// use lctree::LinkCutTree;
    ///
    /// let mut lctree = LinkCutTree::default();
    /// let alice = lctree.make_tree(0.0);
    /// let bob = lctree.make_tree(1.0);
    /// let clay = lctree.make_tree(2.0);
    ///
    /// lctree.link(alice, bob);
    /// lctree.link(bob, clay);
    /// assert!(lctree.connected(alice, clay));
    /// ```
    pub fn link(&mut self, v: usize, w: usize) {
        if self.connected(v, w) {
            return;
        }
        // v is the root of its represented tree:
        self.forest.set_left(v, w);
    }

    /// Cuts the link between two nodes (if it exists)
    ///
    /// # Examples
    /// ```
    /// use lctree::LinkCutTree;
    ///
    /// let mut lctree = LinkCutTree::default();
    /// let alice = lctree.make_tree(0.0);
    /// let bob = lctree.make_tree(1.0);
    /// assert!(!lctree.connected(alice, bob)); // not connected yet
    ///
    /// lctree.link(alice, bob);
    /// assert!(lctree.connected(alice, bob)); // now connected
    ///
    /// lctree.cut(alice, bob);
    /// assert!(!lctree.connected(alice, bob)); // not connected again
    /// ```
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

    /// Performs path aggregation on a path between two nodes (if they are connected)
    ///
    /// # Examples
    /// ```
    /// use lctree::{LinkCutTree, FindMax};
    ///
    /// let mut lctree: LinkCutTree<FindMax> = LinkCutTree::new();
    /// let alice = lctree.make_tree(0.0);
    /// let bob = lctree.make_tree(10.0);
    /// let clay = lctree.make_tree(1.0);
    /// let dave = lctree.make_tree(2.0);
    ///
    /// // Form a path from Alice to Dave:
    /// lctree.link(alice, bob);
    /// lctree.link(bob, clay);
    /// lctree.link(clay, dave);
    ///
    /// // Find the richest guy in the path from Alice to Dave:
    /// let richest_guy = lctree.path(alice, dave);
    /// assert_eq!(richest_guy.idx, bob);
    /// assert_eq!(richest_guy.weight, 10.0);
    /// ```
    pub fn path(&mut self, v: usize, w: usize) -> P {
        if !self.connected(v, w) {
            return P::default(f64::INFINITY, usize::MAX);
        }
        self.forest.aggregated_path_of(w)
    }

    /// Finds the root of the tree that the query node is in.
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
        assert_eq!(lctree.path(4, 5).idx, 0);
        assert_eq!(lctree.path(3, 6).idx, 0);
        assert_eq!(lctree.path(2, 7).idx, 0);
        assert_eq!(lctree.path(1, 8).idx, 0);
        assert_eq!(lctree.path(0, 9).idx, 0);
        assert_eq!(lctree.path(4, 3).idx, 2);
        assert_eq!(lctree.path(5, 7).idx, 6);
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
        assert_eq!(lctree.path(4, 5).idx, 1);
        assert_eq!(lctree.path(3, 6).idx, 3);
        assert_eq!(lctree.path(2, 7).idx, 1);
        assert_eq!(lctree.path(1, 8).idx, 1);
        assert_eq!(lctree.path(0, 9).idx, 5);
        assert_eq!(lctree.path(4, 3).idx, 3);
        assert_eq!(lctree.path(5, 7).idx, 5);
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

    #[test]
    pub fn test_extend_forest() {
        let weights = vec![1.0, 2.0, 3.0];
        let mut lctree = LinkCutTree::default();
        let trees_ids = lctree.extend_forest(&weights);
        assert_eq!(trees_ids, vec![0, 1, 2]);
    }

    #[test]
    #[should_panic]
    pub fn delete_tree() {
        let mut lctree = LinkCutTree::default();
        let alice = lctree.make_tree(0.0);
        let bob = lctree.make_tree(1.0);
        lctree.link(alice, bob);
        lctree.remove_tree(alice); // should panic
    }
}
