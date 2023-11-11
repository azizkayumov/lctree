use crate::{
    index::Index,
    node::{Node, Parent},
    path::Path,
};

pub struct Forest<P: Path> {
    nodes: Vec<Node<P>>,
    index: Index,
}

impl<P: Path> Forest<P> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            index: Index::new(),
        }
    }

    pub fn create_node(&mut self, weight: f64) -> usize {
        let idx = self.index.insert();
        if idx < self.nodes.len() {
            self.nodes[idx] = Node::new(idx, weight);
            return idx;
        }
        self.nodes.push(Node::new(idx, weight));
        idx
    }

    pub fn delete_node(&mut self, node_idx: usize) {
        assert!(
            self.nodes[node_idx].degree == 0,
            "Invalid deletion: tree contains more than one node."
        );
        self.index.delete(node_idx);
    }

    #[inline]
    pub fn set_right(&mut self, node_idx: usize, right_idx: usize) {
        assert!(
            self.nodes[node_idx].right.is_none(),
            "set_right: node_idx already has a right child"
        );
        self.nodes[node_idx].right = Some(right_idx);
        self.nodes[right_idx].parent = Parent::Node(node_idx);
    }

    #[inline]
    pub fn set_left(&mut self, node_idx: usize, left_idx: usize) {
        assert!(
            self.nodes[node_idx].left.is_none(),
            "set_left: node_idx already has a left child"
        );
        self.nodes[node_idx].left = Some(left_idx);
        self.nodes[left_idx].parent = Parent::Node(node_idx);
        self.nodes[node_idx].degree += 1;
        self.nodes[left_idx].degree += 1;
    }

    #[inline]
    pub fn cut_left(&mut self, node_idx: usize) {
        assert!(
            self.nodes[node_idx].left.is_some(),
            "cut_left: node_idx does not have a left child"
        );
        let left = self.nodes[node_idx].left.unwrap();
        self.nodes[node_idx].left = None;
        self.nodes[left].parent = Parent::Root;
        self.nodes[node_idx].degree -= 1;
        self.nodes[left].degree -= 1;
    }

    #[inline]
    pub fn parent_of(&self, node_idx: usize) -> Option<usize> {
        if let Parent::Node(parent_idx) = self.nodes[node_idx].parent {
            Some(parent_idx)
        } else {
            None
        }
    }

    #[inline]
    pub fn path_parent_of(&self, node_idx: usize) -> Option<usize> {
        if let Parent::Path(parent_idx) = self.nodes[node_idx].parent {
            Some(parent_idx)
        } else {
            None
        }
    }

    #[inline]
    pub fn left_of(&self, node_idx: usize) -> Option<usize> {
        self.nodes[node_idx].left
    }

    #[inline]
    pub fn right_of(&self, node_idx: usize) -> Option<usize> {
        self.nodes[node_idx].right
    }

    #[inline]
    pub fn aggregated_path_of(&self, node_idx: usize) -> P {
        self.nodes[node_idx].path
    }

    // Unflips the subtree rooted at `node_idx`, swapping the left and right children.
    // The children's `flipped` flag is also toggled to propogate the change down the tree.
    pub fn normalize(&mut self, node_idx: usize) {
        if self.nodes[node_idx].flipped {
            self.nodes[node_idx].flip_children();
            self.nodes[node_idx].flipped = false;
            if let Some(left_child) = self.nodes[node_idx].left {
                self.nodes[left_child].flipped ^= true;
            }
            if let Some(right_child) = self.nodes[node_idx].right {
                self.nodes[right_child].flipped ^= true;
            }
        }
    }

    // Updates the path aggregate information for the subtree rooted at `node_idx`.
    pub fn update(&mut self, node_idx: usize) {
        self.nodes[node_idx].path = P::default(self.nodes[node_idx].weight, node_idx);
        if let Some(left_child) = self.nodes[node_idx].left {
            let left_path = self.nodes[left_child].path;
            self.nodes[node_idx].path.aggregate(left_path);
        }
        if let Some(right_child) = self.nodes[node_idx].right {
            let right_path = self.nodes[right_child].path;
            self.nodes[node_idx].path.aggregate(right_path);
        }
    }

    pub fn remove_preferred_child(&mut self, node_idx: usize) {
        if let Some(right_idx) = self.nodes[node_idx].right {
            self.nodes[node_idx].right = None;
            self.nodes[right_idx].parent = Parent::Path(node_idx);
            self.update(node_idx);
        }
    }

    pub fn flip(&mut self, node_idx: usize) {
        self.nodes[node_idx].flipped ^= true;
        self.normalize(node_idx);
    }

    // Rotates the subtree rooted at `node_idx` to the left:
    //  For ex., left rotation on '0':
    //         0                2
    //        / \       =>     / \
    //       1   2            0   4
    //          / \          / \
    //         3   4        1   3
    fn rotate_left(&mut self, node_idx: usize) {
        assert!(
            node_idx < self.nodes.len(),
            "rotate_left: node_idx out of bounds"
        );
        assert!(
            self.nodes[node_idx].right.is_some(),
            "rotate_left: node_idx does not have a right child"
        );

        let right_child = self.nodes[node_idx].right.unwrap();
        if let Parent::Node(parent_idx) = self.nodes[node_idx].parent {
            if self.nodes[parent_idx].left == Some(node_idx) {
                self.nodes[parent_idx].left = Some(right_child);
            } else {
                self.nodes[parent_idx].right = Some(right_child);
            }
        }

        self.nodes[node_idx].right = self.nodes[right_child].left;
        self.nodes[right_child].left = Some(node_idx);
        self.nodes[right_child].parent = self.nodes[node_idx].parent;
        self.nodes[node_idx].parent = Parent::Node(right_child);

        if let Some(new_right_child) = self.nodes[node_idx].right {
            self.nodes[new_right_child].parent = Parent::Node(node_idx);
        }
    }

    // Rotates the subtree rooted at `node_idx` to the right:
    //  For ex., right rotation on '0':
    //         0                1
    //        / \      =>      / \
    //       1   4            2   0
    //      / \                  / \
    //     2   3                3   4
    fn rotate_right(&mut self, node_idx: usize) {
        assert!(
            node_idx < self.nodes.len(),
            "rotate_right: node_idx out of bounds"
        );
        assert!(
            self.nodes[node_idx].left.is_some(),
            "rotate_right: node_idx does not have a left child"
        );

        let left_child = self.nodes[node_idx].left.unwrap();
        if let Parent::Node(parent_idx) = self.nodes[node_idx].parent {
            if self.nodes[parent_idx].left == Some(node_idx) {
                self.nodes[parent_idx].left = Some(left_child);
            } else {
                self.nodes[parent_idx].right = Some(left_child);
            }
        }

        self.nodes[node_idx].left = self.nodes[left_child].right;
        self.nodes[left_child].right = Some(node_idx);
        self.nodes[left_child].parent = self.nodes[node_idx].parent;
        self.nodes[node_idx].parent = Parent::Node(left_child);

        if let Some(new_left_child) = self.nodes[node_idx].left {
            self.nodes[new_left_child].parent = Parent::Node(node_idx);
        }
    }

    // Rotates the parent of `node_idx` to the right or left, depending on the relationship between.
    fn rotate(&mut self, node_idx: usize) {
        assert!(
            node_idx < self.nodes.len(),
            "rotate: node_idx out of bounds"
        );
        assert!(
            matches!(self.nodes[node_idx].parent, Parent::Node(_)),
            "rotate: node_idx does not have a parent"
        );

        if let Parent::Node(parent_idx) = self.nodes[node_idx].parent {
            self.normalize(parent_idx);
            self.normalize(node_idx);
            if self.nodes[parent_idx].left == Some(node_idx) {
                self.rotate_right(parent_idx);
            } else {
                self.rotate_left(parent_idx);
            }
            self.update(parent_idx);
        }
    }

    // Splays the subtree rooted at `node_idx`, making it the new root of the tree.
    //  For ex., splaying on '2':
    //   0                  2
    //    \       =>       / \
    //     1              0   1
    //    /
    //   2
    pub fn splay(&mut self, node_idx: usize) {
        while let Parent::Node(parent_idx) = self.nodes[node_idx].parent {
            if let Parent::Node(grandparent_idx) = self.nodes[parent_idx].parent {
                if (self.nodes[grandparent_idx].left == Some(parent_idx))
                    == (self.nodes[parent_idx].left == Some(node_idx))
                {
                    // zig-zig (same direction):
                    self.rotate(parent_idx);
                } else {
                    // zig-zag:
                    self.rotate(node_idx);
                }
            }
            // zig
            self.rotate(node_idx);
        }
        self.normalize(node_idx);
        self.update(node_idx);
    }
}

#[cfg(test)]
mod tests {
    use super::Forest;
    use crate::{node, path::FindMax};

    fn create_forest(n: usize) -> Forest<FindMax> {
        let mut forest = Forest::new();
        for i in 0..n {
            forest.create_node(i as f64);
        }
        forest
    }

    #[test]
    pub fn create_node() {
        let mut forest: Forest<FindMax> = super::Forest::new();
        let alice = forest.create_node(0.0);
        let bob = forest.create_node(1.0);
        let charlie = forest.create_node(2.0);
        assert_eq!([alice, bob, charlie], [0, 1, 2]);

        forest.delete_node(bob);
        let david = forest.create_node(4.0);
        // Should reuse the space of bob's tree (which was removed)
        assert_eq!(david, bob);
    }

    #[test]
    #[should_panic]
    pub fn set_right() {
        let mut forest: Forest<FindMax> = super::Forest::new();
        let alice = forest.create_node(0.0);
        let bob = forest.create_node(1.0);
        forest.set_right(alice, bob);
        let charlie = forest.create_node(2.0);
        // Should panic because alice already has a right child
        forest.set_right(alice, charlie);
    }

    #[test]
    #[should_panic]
    pub fn set_left() {
        let mut forest: Forest<FindMax> = super::Forest::new();
        let alice = forest.create_node(0.0);
        let bob = forest.create_node(1.0);
        forest.set_left(alice, bob);
        let charlie = forest.create_node(2.0);
        // Should panic because alice already has a left child
        forest.set_left(alice, charlie);
    }

    #[test]
    #[should_panic]
    pub fn cut_left() {
        let mut forest: Forest<FindMax> = super::Forest::new();
        let alice = forest.create_node(0.0);
        let bob = forest.create_node(1.0);
        forest.set_left(alice, bob);
        // Should panic because alice does not have a left child
        forest.cut_left(bob);
    }

    #[test]
    pub fn rotate_left_root() {
        // form the following tree and rotate left on '0':
        //         0                2
        //        / \       =>     / \
        //       1   2            0   4
        //          / \          / \
        //         3   4        1   3
        let mut forest = create_forest(5);
        forest.nodes[0].left = Some(1);
        forest.nodes[0].right = Some(2);
        forest.nodes[1].parent = node::Parent::Node(0);
        forest.nodes[2].parent = node::Parent::Node(0);
        forest.nodes[2].left = Some(3);
        forest.nodes[2].right = Some(4);
        forest.nodes[3].parent = node::Parent::Node(2);
        forest.nodes[4].parent = node::Parent::Node(2);
        forest.rotate_left(0);
        assert!(matches!(forest.nodes[2].parent, node::Parent::Root));
        assert_eq!(forest.nodes[2].left, Some(0));
        assert!(matches!(forest.nodes[0].parent, node::Parent::Node(2)));
        assert_eq!(forest.nodes[2].right, Some(4));
        assert!(matches!(forest.nodes[4].parent, node::Parent::Node(2)));
        assert_eq!(forest.nodes[0].left, Some(1));
        assert!(matches!(forest.nodes[1].parent, node::Parent::Node(0)));
        assert_eq!(forest.nodes[0].right, Some(3));
        assert!(matches!(forest.nodes[3].parent, node::Parent::Node(0)));
        assert!(forest.nodes[1].left.is_none());
        assert!(forest.nodes[1].right.is_none());
        assert!(forest.nodes[3].left.is_none());
        assert!(forest.nodes[3].right.is_none());
    }

    #[test]
    pub fn rotate_right_root() {
        // form the tree and rotate left on '0':
        //         0                1
        //        / \       =>     / \
        //       1   4            2   0
        //      / \                  / \
        //     2   3                3   4
        let mut forest = create_forest(5);
        forest.nodes[0].left = Some(1);
        forest.nodes[0].right = Some(4);
        forest.nodes[1].parent = node::Parent::Node(0);
        forest.nodes[4].parent = node::Parent::Node(0);
        forest.nodes[1].left = Some(2);
        forest.nodes[1].right = Some(3);
        forest.nodes[2].parent = node::Parent::Node(1);
        forest.nodes[3].parent = node::Parent::Node(1);
        forest.rotate_right(0);
        assert!(matches!(forest.nodes[1].parent, node::Parent::Root));
        assert_eq!(forest.nodes[1].left, Some(2));
        assert!(matches!(forest.nodes[2].parent, node::Parent::Node(1)));
        assert_eq!(forest.nodes[1].right, Some(0));
        assert!(matches!(forest.nodes[0].parent, node::Parent::Node(1)));
        assert_eq!(forest.nodes[0].left, Some(3));
        assert!(matches!(forest.nodes[3].parent, node::Parent::Node(0)));
        assert_eq!(forest.nodes[0].right, Some(4));
        assert!(matches!(forest.nodes[4].parent, node::Parent::Node(0)));
        assert!(forest.nodes[3].left.is_none());
        assert!(forest.nodes[3].right.is_none());
        assert!(forest.nodes[4].left.is_none());
        assert!(forest.nodes[4].right.is_none());
    }

    #[test]
    pub fn rotate_parent_left() {
        // form the tree and rotate on '1':
        //      0              1
        //     /       =>       \
        //    1                  0
        let mut forest = create_forest(2);
        forest.nodes[0].left = Some(1);
        forest.nodes[1].parent = node::Parent::Node(0);
        forest.rotate(1);
        assert!(matches!(forest.nodes[1].parent, node::Parent::Root));
        assert_eq!(forest.nodes[1].left, None);
        assert_eq!(forest.nodes[1].right, Some(0));
        assert!(matches!(forest.nodes[0].parent, node::Parent::Node(1)));
        assert!(forest.nodes[0].left.is_none());
        assert!(forest.nodes[0].right.is_none());
    }

    #[test]
    pub fn rotate_parent_right() {
        // form the tree and rotate on '1':
        //    0                 1
        //     \        =>     /
        //      1             0
        let mut forest = create_forest(2);
        forest.nodes[0].right = Some(1);
        forest.nodes[1].parent = node::Parent::Node(0);
        forest.rotate(1);
        assert!(matches!(forest.nodes[1].parent, node::Parent::Root));
        assert_eq!(forest.nodes[1].left, Some(0));
        assert_eq!(forest.nodes[1].right, None);
        assert!(matches!(forest.nodes[0].parent, node::Parent::Node(1)));
        assert!(forest.nodes[0].left.is_none());
        assert!(forest.nodes[0].right.is_none());
    }

    #[test]
    pub fn splay_leaf() {
        // form the tree and splay on '2':
        //   0                  2
        //    \       =>       / \
        //     1              0   1
        //    /
        //   2
        let mut forest = create_forest(3);
        forest.nodes[0].right = Some(1);
        forest.nodes[1].parent = node::Parent::Node(0);
        forest.nodes[1].left = Some(2);
        forest.nodes[2].parent = node::Parent::Node(1);
        forest.splay(2);
        assert!(matches!(forest.nodes[2].parent, node::Parent::Root));
        assert_eq!(forest.nodes[2].left, Some(0));
        assert_eq!(forest.nodes[2].right, Some(1));
        assert!(matches!(forest.nodes[0].parent, node::Parent::Node(2)));
        assert!(matches!(forest.nodes[1].parent, node::Parent::Node(2)));
    }

    #[test]
    pub fn splay_internal_node() {
        // form the tree and splay on '1':
        //   0                  1
        //    \       =>       /
        //     1              0
        //    /                \
        //   2                  2
        let mut forest = create_forest(3);
        forest.nodes[0].right = Some(1);
        forest.nodes[1].parent = node::Parent::Node(0);
        forest.nodes[1].left = Some(2);
        forest.nodes[2].parent = node::Parent::Node(1);
        forest.splay(1);
        assert!(matches!(forest.nodes[1].parent, node::Parent::Root));
        assert_eq!(forest.nodes[1].left, Some(0));
        assert_eq!(forest.nodes[1].right, None);
        assert!(matches!(forest.nodes[0].parent, node::Parent::Node(1)));
        assert_eq!(forest.nodes[0].left, None);
        assert_eq!(forest.nodes[0].right, Some(2));
        assert!(matches!(forest.nodes[2].parent, node::Parent::Node(0)));
        assert!(forest.nodes[2].left.is_none());
        assert!(forest.nodes[2].right.is_none());
    }

    #[test]
    pub fn splay_preserve_path_pointer() {
        // Node '0' has a path pointer to Node '6',
        // the remaning nodes are represented in a Splay-tree as given below.
        // splaying a leaf node '4' should result in:
        //    6              6                6
        //    |              |                |
        //    0              0                4
        //     \              \              / \
        //      1     =>       4      =>    0   1
        //     /              / \            \
        //    2              2   1            2
        //   / \            /                /
        //  3   4          3                3
        //
        let mut forest = create_forest(6);
        forest.nodes[0].parent = node::Parent::Path(6);
        forest.nodes[0].right = Some(1);
        forest.nodes[1].parent = node::Parent::Node(0);
        forest.nodes[1].left = Some(2);
        forest.nodes[2].parent = node::Parent::Node(1);
        forest.nodes[2].left = Some(3);
        forest.nodes[3].parent = node::Parent::Node(2);
        forest.nodes[2].right = Some(4);
        forest.nodes[4].parent = node::Parent::Node(2);
        forest.splay(4);
        // The path pointer to Node '6' should be preserved:
        assert!(matches!(forest.nodes[4].parent, node::Parent::Path(6)));
        // The rest of the tree should be a rotated Splay-tree:
        assert_eq!(forest.nodes[4].left, Some(0));
        assert!(matches!(forest.nodes[0].parent, node::Parent::Node(4)));
        assert_eq!(forest.nodes[4].right, Some(1));
        assert!(matches!(forest.nodes[1].parent, node::Parent::Node(4)));
        assert_eq!(forest.nodes[0].right, Some(2));
        assert!(matches!(forest.nodes[2].parent, node::Parent::Node(0)));
        assert_eq!(forest.nodes[2].left, Some(3));
        assert!(matches!(forest.nodes[3].parent, node::Parent::Node(2)));
    }

    #[test]
    pub fn toggle_flip() {
        let mut forest = create_forest(3);
        forest.nodes[0].left = Some(1);
        forest.nodes[0].right = Some(2);
        forest.nodes[1].parent = node::Parent::Node(0);
        forest.nodes[2].parent = node::Parent::Node(0);
        forest.nodes[0].flipped = true;
        forest.normalize(0);
        assert!(!forest.nodes[0].flipped);
        assert!(forest.nodes[1].flipped);
        assert!(forest.nodes[2].flipped);
        assert_eq!(forest.nodes[0].left, Some(2));
        assert_eq!(forest.nodes[0].right, Some(1));
    }
}
