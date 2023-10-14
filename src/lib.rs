#![allow(dead_code, clippy::module_name_repetitions)] // yes, I want to name my structs with the same name as the file
mod node;
mod path;
mod splay;
use path::{update, FindMax, FindMin, FindSum, Path};
use splay::unflip;

use crate::{
    node::{Node, Parent},
    splay::splay,
};

pub struct LinkCutTree<T: Path + Copy + Clone> {
    forest: Vec<Node<T>>,
}

impl<T: Path + Copy + Clone> LinkCutTree<T> {
    #[must_use]
    pub fn new() -> Self {
        Self { forest: Vec::new() }
    }

    pub fn make_tree(&mut self, weight: f64) -> usize {
        let idx = self.forest.len();
        self.forest.push(Node::new(idx, weight));
        idx
    }

    /// Constructs a path from a node to the root of the tree.
    fn access(&mut self, v: usize) {
        splay(&mut self.forest, v);

        if let Some(right_idx) = self.forest[v].right {
            self.forest[v].right = None;
            self.forest[right_idx].parent = Parent::Path(v);
        }

        while let Parent::Path(path_idx) = self.forest[v].parent {
            splay(&mut self.forest, path_idx);
            // detach the right child of the path parent
            if let Some(right_idx) = self.forest[path_idx].right {
                self.forest[right_idx].parent = Parent::Path(path_idx);
                self.forest[path_idx].right = None;
            }

            // attach the node as the path parent's right child
            self.forest[path_idx].right = Some(v);
            self.forest[v].parent = Parent::Node(path_idx);

            splay(&mut self.forest, v);
        }

        // update aggregate information
        update(&mut self.forest, v);
    }

    /// Makes v the root of its represented tree by flipping the path from v to the root.
    fn reroot(&mut self, v: usize) {
        self.access(v);
        self.forest[v].flipped ^= true;
        unflip(&mut self.forest, v);
    }

    /// Checks if v and w are connected in the forest.
    pub fn connected(&mut self, v: usize, w: usize) -> bool {
        self.reroot(v); // v is now the root of the tree
        self.access(w);
        // if access(w) messed with the root of the tree, then v and w are connected:
        !matches!(self.forest[v].parent, Parent::Root) || v == w
    }

    /// Creates a link between two nodes in the forest (where w is the parent of v).
    pub fn link(&mut self, v: usize, w: usize) {
        if self.connected(v, w) {
            return;
        }
        // v is the root of its represented tree, so no need to check if it has a left child
        self.forest[v].left = Some(w);
        self.forest[w].parent = Parent::Node(v);
    }

    /// Cuts the link between nodes v and w (if it exists)
    pub fn cut(&mut self, v: usize, w: usize) {
        if !self.connected(v, w) {
            return;
        }
        // detach w from its parent (which is v)
        if let Some(left) = self.forest[w].left {
            if left != v {
                eprintln!("Error: no link between {v} and {w}"); // maybe this should be a panic?
                return;
            }
            self.forest[w].left = None;
            self.forest[left].parent = Parent::Root;
        }
    }

    /// Performs path aggregation on a path between v and w (if they are connected)
    pub fn path(&mut self, v: usize, w: usize) -> T {
        if !self.connected(v, w) {
            return T::default(f64::INFINITY, usize::MAX);
        }
        self.forest[w].path
    }

    /// Finds the root of the tree that v is in.
    pub fn findroot(&mut self, v: usize) -> usize {
        self.access(v);
        let mut root = v;
        while let Some(left) = self.forest[root].left {
            root = left;
        }
        splay(&mut self.forest, root); // fast access to the root next time
        root
    }
}

impl Default for LinkCutTree<FindMax> {
    fn default() -> Self {
        Self::new()
    }
}

impl LinkCutTree<FindMax> {
    #[must_use]
    pub fn findmax() -> Self {
        Self::new()
    }
}

impl LinkCutTree<FindMin> {
    #[must_use]
    pub fn findmin() -> Self {
        Self::new()
    }
}

impl LinkCutTree<FindSum> {
    #[must_use]
    pub fn findsum() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::node::Parent;

    #[test]
    pub fn access() {
        let mut tree = super::LinkCutTree::findmax();
        for i in 0..4 {
            tree.make_tree(i as f64);
        }
        // '1' has a path pointer to '0', '1' has a right child '2'.
        // after access(2), '2' should be the root of the tree:
        //    0             0             0               2
        //    |             |              \             /
        //    1      =>     2      =>       2     =>    0
        //     \           /               /             \
        //      2         1               1               1
        tree.forest[1].parent = Parent::Path(0);
        tree.forest[1].right = Some(2);
        tree.forest[2].parent = Parent::Node(1);
        tree.access(2);
        assert!(matches!(tree.forest[2].parent, Parent::Root));
        assert_eq!(tree.forest[2].right, None);
        assert_eq!(tree.forest[2].left, Some(0));
        assert!(matches!(tree.forest[0].parent, Parent::Node(2)));
        assert_eq!(tree.forest[0].left, None);
        assert_eq!(tree.forest[0].right, Some(1));
        assert!(matches!(tree.forest[1].parent, Parent::Node(0)));
        assert_eq!(tree.forest[1].left, None);
        assert_eq!(tree.forest[1].right, None);
    }

    #[test]
    pub fn link_already_connected() {
        // '2' has a right child '3':
        // link(0, 3) should add no link, and result in (| denotes a path pointer):
        //   0                   3
        //  / \                 /
        // 1   2     =>        0
        //      \              |\
        //       3             1 2
        //
        let mut tree = super::LinkCutTree::default();
        for i in 0..4 {
            tree.make_tree(i as f64);
        }
        tree.forest[0].left = Some(1);
        tree.forest[0].right = Some(2);
        tree.forest[1].parent = Parent::Node(0);
        tree.forest[2].parent = Parent::Node(0);
        tree.forest[2].right = Some(3);
        tree.forest[3].parent = Parent::Node(2);
        tree.link(0, 3);
        assert!(matches!(tree.forest[3].parent, Parent::Root));
        assert_eq!(tree.forest[3].left, Some(0));
        assert_eq!(tree.forest[3].right, None);
        assert!(matches!(tree.forest[0].parent, Parent::Node(3)));
        assert_eq!(tree.forest[0].left, None);
        assert!(matches!(tree.forest[1].parent, Parent::Path(0)));
        assert_eq!(tree.forest[0].right, Some(2));
        assert!(matches!(tree.forest[2].parent, Parent::Node(0)));
    }

    #[test]
    pub fn link() {
        // Given two trees:
        //   0               3
        //  / \
        // 1   2
        // link(3, 1) should result in a single tree (| denotes a path pointer):
        //                          3
        //                         /
        //   1      3             1
        //   |                    |
        //   0            =>      0
        //    \                    \
        //     2                    2
        let mut tree = super::LinkCutTree::default();
        for i in 0..4 {
            tree.make_tree(i as f64);
        }
        tree.forest[0].left = Some(1);
        tree.forest[0].right = Some(2);
        tree.forest[1].parent = Parent::Node(0);
        tree.forest[2].parent = Parent::Node(0);
        tree.link(3, 1);
        assert!(matches!(tree.forest[3].parent, Parent::Root));
        assert_eq!(tree.forest[3].left, Some(1));
        assert!(matches!(tree.forest[1].parent, Parent::Node(3)));
        assert!(matches!(tree.forest[0].parent, Parent::Path(1)));
        assert_eq!(tree.forest[0].left, None);
        assert_eq!(tree.forest[0].right, Some(2));
    }

    #[test]
    pub fn connected() {
        // check two splay trees that are connected by a path pointer
        //     0
        //    / \
        //   1   2
        //       |
        //       3
        let mut tree = super::LinkCutTree::default();
        for i in 0..4 {
            tree.make_tree(i as f64);
        }
        tree.forest[0].left = Some(1);
        tree.forest[0].right = Some(2);
        tree.forest[1].parent = Parent::Node(0);
        tree.forest[2].parent = Parent::Node(0);
        tree.forest[3].parent = Parent::Path(2);

        assert!(tree.connected(0, 3));
        assert!(tree.connected(1, 3));
        assert!(tree.connected(2, 3));
    }

    #[test]
    pub fn cut() {
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
        let mut lctree = super::LinkCutTree::default();
        let weights = [9.0, 1.0, 8.0, 0.0, 6.0, 2.0, 4.0, 3.0, 7.0, 5.0];
        for i in 0..weights.len() {
            lctree.make_tree(weights[i]);
        }

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
        let mut lctree = super::LinkCutTree::findmin();
        let weights = [9.0, 1.0, 8.0, 0.0, 6.0, 2.0, 4.0, 3.0, 7.0, 5.0];
        for i in 0..weights.len() {
            lctree.make_tree(weights[i]);
        }

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
        let mut lctree = super::LinkCutTree::findsum();
        let weights = [9.0, 1.0, 8.0, 0.0, 6.0, 2.0, 4.0, 3.0, 7.0, 5.0];
        for i in 0..weights.len() {
            lctree.make_tree(weights[i]);
        }

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
