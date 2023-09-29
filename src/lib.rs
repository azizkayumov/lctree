#![allow(dead_code, clippy::module_name_repetitions)]
use node::Node; // yes, I want to name my structs with the same name as the file
mod access;
mod connected;
mod cut;
mod link;
mod node;
mod path;
mod splay;

pub struct LinkCutTree {
    forest: Vec<Node>,
}

impl LinkCutTree {
    #[must_use]
    pub fn new(n: usize) -> Self {
        let nodes = (0..n).map(|i| Node::new(i, 0.0)).collect();
        Self { forest: nodes }
    }

    pub fn link(&mut self, child: usize, parent: usize) {
        link::link(&mut self.forest, child, parent);
    }

    pub fn cut(&mut self, child: usize) {
        cut::cut(&mut self.forest, child);
    }

    pub fn connected(&mut self, a: usize, b: usize) -> bool {
        connected::connected(&mut self.forest, a, b)
    }
}

#[cfg(test)]
mod tests {
    use crate::node::Node;

    fn create_nodes(n: usize) -> Vec<Node> {
        (0..n).map(|i| Node::new(i, 0.0)).collect()
    }

    #[test]
    pub fn rooted_tree() {
        // We form a tree with the following structure:
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

        // checking connectivity:
        for i in 0..10 {
            for j in 0..10 {
                assert!(lctree.connected(i, j));
            }
        }

        // we cut node 6 from its parent 0:
        lctree.cut(6);

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

        // we check connectivity again for the two trees:
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
}
