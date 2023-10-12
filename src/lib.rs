#![allow(dead_code, clippy::module_name_repetitions)] // yes, I want to name my structs with the same name as the file
mod node;
mod path;
mod splay;
use path::update_max;
use splay::unflip;

use crate::{
    node::{Node, Parent},
    splay::splay,
};

pub struct LinkCutTree {
    forest: Vec<Node>,
}

impl LinkCutTree {
    #[must_use]
    pub fn new(n: usize) -> Self {
        let nodes = (0..n).map(|i| Node::new(i, 0.0)).collect();
        Self { forest: nodes }
    }

    pub fn set_weight(&mut self, v: usize, weight: f64) {
        self.forest[v].weight = weight;
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
        update_max(&mut self.forest, v);
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
            return; // already connected
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
            self.forest[w].left = None;
            self.forest[left].parent = Parent::Root;
        }
    }

    /// Finds the maximum weight in the path from nodes v and w (if they are connected)
    pub fn findmax(&mut self, v: usize, w: usize) -> usize {
        if !self.connected(v, w) {
            return usize::MAX;
        }
        self.forest[w].max_weight_idx
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

#[cfg(test)]
mod tests {
    use crate::node::Parent;

    #[test]
    pub fn access() {
        let mut tree = super::LinkCutTree::new(4);
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
        let mut tree = super::LinkCutTree::new(4);
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
        let mut tree = super::LinkCutTree::new(4);
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
        let mut tree = super::LinkCutTree::new(4);
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
        // We form a link-cut tree from a rooted tree with the following structure:
        //     0
        //    / \
        //   1   6
        //  / \   \
        // 2   3   7
        //    / \   \
        //   4   5   8
        //          /
        //         9
        let mut lctree = super::LinkCutTree::new(10);
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
    pub fn findroot() {
        // We form a link-cut tree from a rooted tree with the following structure:
        //     0
        //    / \
        //   1   6
        //  / \   \
        // 2   3   7
        //    / \   \
        //   4   5   8
        //          /
        //         9
        let mut lctree = super::LinkCutTree::new(10);
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
        // We form a link-cut tree from a rooted tree with the following structure:
        //     0
        //    / \
        //   1   4
        //  / \
        // 2   3
        let mut lctree = super::LinkCutTree::new(10);
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
}
