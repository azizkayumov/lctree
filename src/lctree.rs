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
        v == w || self.findroot(v) == self.findroot(w)
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
    pub fn link(&mut self, v: usize, w: usize) -> bool {
        self.reroot(v);
        self.access(w);
        // if access(w) messed with the root of the tree, then v and w are connected:
        if self.forest.parent_of(v).is_some() || v == w {
            return false;
        }
        // v is the root of its represented tree:
        self.forest.set_left(v, w);
        true
    }

    /// Checks if two nodes are connected by a link
    /// (i.e. v is the parent of w or vice versa).
    ///
    /// # Examples
    /// ```
    /// use lctree::LinkCutTree;
    ///
    /// let mut lctree = LinkCutTree::default();
    /// let alice = lctree.make_tree(0.0);
    /// let bob = lctree.make_tree(0.0);
    /// let clay = lctree.make_tree(0.0);
    ///
    /// lctree.link(alice, bob);
    /// lctree.link(bob, clay);
    ///
    /// assert!(lctree.linked(alice, bob)); // alice and bob are connected by a link
    /// assert!(!lctree.linked(alice, clay)); // alice and clay are not connected by a link
    /// ```
    pub fn linked(&mut self, v: usize, w: usize) -> bool {
        self.reroot(v);
        self.access(w);
        self.forest.left_of(w) == Some(v) && self.forest.right_of(v).is_none()
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
    pub fn cut(&mut self, v: usize, w: usize) -> bool {
        if !self.linked(v, w) {
            return false;
        }
        self.forest.cut_left(w);
        true
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
        self.reroot(v);
        self.access(w);
        if self.forest.parent_of(v).is_none() && v != w {
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
    use crate::{FindSum, FindXor, LinkCutTree};

    #[test]
    pub fn link_cut() {
        // We form a link-cut tree from the following rooted tree:
        //     a
        //    / \
        //   b   e
        //  / \   \
        // c   d   f

        let mut lctree = super::LinkCutTree::default();
        let a = lctree.make_tree(0.0);
        let b = lctree.make_tree(0.0);
        let c = lctree.make_tree(0.0);
        let d = lctree.make_tree(0.0);
        let e = lctree.make_tree(0.0);
        let f = lctree.make_tree(0.0);

        lctree.link(b, a);
        lctree.link(c, b);
        lctree.link(d, b);
        lctree.link(e, a);
        lctree.link(f, e);

        // Checking connectivity:
        let nodes = [a, b, c, d, e, f];
        for i in nodes {
            for j in nodes {
                assert!(lctree.connected(i, j));
            }
        }

        // We cut node e from its parent a:
        lctree.cut(e, a);

        // The forest should now look like this:
        //     a
        //    /
        //   b      e
        //  / \      \
        // c   d      f

        // We check connectivity again for the two trees:
        let left_tree = [a, b, c, d];
        let right_tree = [e, f];
        for i in left_tree {
            for j in left_tree {
                assert!(lctree.connected(i, j));
            }
        }
        for i in right_tree {
            for j in right_tree {
                assert!(lctree.connected(i, j));
            }
        }
        for left in left_tree {
            for right in right_tree {
                assert!(!lctree.connected(left, right));
            }
        }
    }

    #[test]
    pub fn connected_so_no_need_to_link() {
        let mut lctree = super::LinkCutTree::default();
        let alice = lctree.make_tree(0.0);
        let bob = lctree.make_tree(10.0);
        let clay = lctree.make_tree(2.0);
        lctree.link(alice, bob);
        lctree.link(bob, clay);
        // Try to link two nodes that are already connected:
        assert!(!lctree.link(alice, clay));
    }

    #[test]
    pub fn connected_but_no_edge_to_cut() {
        let mut lctree = super::LinkCutTree::default();
        let alice = lctree.make_tree(0.0);
        let bob = lctree.make_tree(10.0);
        let clay = lctree.make_tree(2.0);
        lctree.link(alice, bob);
        lctree.link(bob, clay);
        // Try to cut an edge that doesn't exist:
        assert!(!lctree.cut(alice, clay));
    }

    #[test]
    pub fn linked() {
        let mut lctree = super::LinkCutTree::default();
        let alice = lctree.make_tree(0.0);
        let bob = lctree.make_tree(0.0);
        let clay = lctree.make_tree(0.0);

        lctree.link(alice, bob);
        lctree.link(bob, clay);

        assert!(lctree.linked(alice, bob));
        assert!(lctree.linked(bob, clay));
        // alice and clay are not connected by a link
        assert!(!lctree.linked(alice, clay));
    }

    #[test]
    pub fn findroot() {
        // We form a link-cut tree from the following rooted tree:
        //     a
        //    / \
        //   b   e
        //  / \   \
        // c   d   f
        let mut lctree = super::LinkCutTree::default();
        let a = lctree.make_tree(0.0);
        let b = lctree.make_tree(0.0);
        let c = lctree.make_tree(0.0);
        let d = lctree.make_tree(0.0);
        let e = lctree.make_tree(0.0);
        let f = lctree.make_tree(0.0);
        lctree.link(b, a);
        lctree.link(c, b);
        lctree.link(d, b);
        lctree.link(e, a);
        lctree.link(f, e);

        // Checking findroot:
        let nodes = [a, b, c, d, e, f];
        for i in nodes {
            assert_eq!(lctree.findroot(i), a);
        }

        // We cut node e from its parent a:
        lctree.cut(e, a);

        // The forest should now look like this:
        //     a
        //    /
        //   b      e
        //  / \      \
        // c   d      f

        // We check findroot again for the two trees:
        let left_tree = [a, b, c, d];
        for i in left_tree {
            assert_eq!(lctree.findroot(i), a);
        }

        let right_tree = [e, f];
        for i in right_tree {
            assert_eq!(lctree.findroot(i), e);
        }
    }

    #[test]
    pub fn reroot() {
        // We form a link-cut tree from the following rooted tree:
        //     a
        //    / \
        //   b   e
        //  / \   \
        // c   d   f
        let mut lctree = super::LinkCutTree::default();
        let a = lctree.make_tree(0.0);
        let b = lctree.make_tree(0.0);
        let c = lctree.make_tree(0.0);
        let d = lctree.make_tree(0.0);
        let e = lctree.make_tree(0.0);
        let f = lctree.make_tree(0.0);
        lctree.link(b, a);
        lctree.link(c, b);
        lctree.link(d, b);
        lctree.link(e, a);
        lctree.link(f, e);

        // Checking findroot (which should be a for all nodes):
        let nodes = [a, b, c, d, e, f];
        for i in nodes {
            assert_eq!(lctree.findroot(i), a);
        }

        // we make b the root of the tree:
        lctree.reroot(b);

        // The root of the tree should now be b:
        for i in nodes {
            assert_eq!(lctree.findroot(i), b);
        }
    }

    #[test]
    pub fn findmax() {
        // We form a link-cut tree from the following rooted tree
        // (the numbers in parentheses are the weights of the nodes):
        //         a(0)
        //        /    \
        //     b(10)   e(7)
        //     /   \     \
        //   c(3)  d(11)  f(2)
        let mut lctree = super::LinkCutTree::default();
        let a = lctree.make_tree(0.0);
        let b = lctree.make_tree(10.);
        let c = lctree.make_tree(3.);
        let d = lctree.make_tree(11.);
        let e = lctree.make_tree(7.);
        let f = lctree.make_tree(2.);

        lctree.link(b, a);
        lctree.link(c, b);
        lctree.link(d, b);
        lctree.link(e, a);
        lctree.link(f, e);

        // We check the node index with max weight in the path from each node to the root:
        assert_eq!(lctree.path(c, f).idx, b);
        assert_eq!(lctree.path(d, f).idx, d);
        assert_eq!(lctree.path(a, f).idx, e);
        assert_eq!(lctree.path(a, a).idx, a);
    }

    #[test]
    pub fn findxor() {
        // We form a link-cut tree from the following rooted tree
        // (the numbers in parentheses are the weights of the nodes):
        //         a(0)
        //        /    \
        //     b(10)   e(7)
        //     /   \     \
        //   c(3)  d(11)  f(2)
        let mut lctree: LinkCutTree<FindXor> = super::LinkCutTree::new();
        let a = lctree.make_tree(0.0);
        let b = lctree.make_tree(10.);
        let c = lctree.make_tree(3.);
        let d = lctree.make_tree(11.);
        let e = lctree.make_tree(7.);
        let f = lctree.make_tree(2.);

        lctree.link(b, a);
        lctree.link(c, b);
        lctree.link(d, b);
        lctree.link(e, a);
        lctree.link(f, e);

        // Checking the xor of weights on the path from each node to the root:
        let result = lctree.path(c, f);
        assert_eq!(result.xor, 3 ^ 10 ^ 0 ^ 7 ^ 2);
    }

    #[test]
    pub fn findsum() {
        // We form a link-cut tree from the following rooted tree
        // (the numbers in parentheses are the weights of the nodes):
        //         a(0)
        //        /    \
        //     b(10)   e(7)
        //     /   \     \
        //   c(3)  d(11)  f(2)
        let mut lctree: LinkCutTree<FindSum> = super::LinkCutTree::new();
        let a = lctree.make_tree(0.0);
        let b = lctree.make_tree(10.);
        let c = lctree.make_tree(3.);
        let d = lctree.make_tree(11.);
        let e = lctree.make_tree(7.);
        let f = lctree.make_tree(2.);

        lctree.link(b, a);
        lctree.link(c, b);
        lctree.link(d, b);
        lctree.link(e, a);
        lctree.link(f, e);

        // We check the node index with max weight in the path from each node to the root:
        assert_eq!(lctree.path(c, f).sum, 22.);
        assert_eq!(lctree.path(d, f).sum, 30.);
        assert_eq!(lctree.path(a, f).sum, 9.);
        assert_eq!(lctree.path(a, a).sum, 0.);
        assert_eq!(lctree.path(c, d).sum, 24.);
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
