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
    use crate::path::FindMax;

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
    #[should_panic]
    pub fn rotate_left_invalid() {
        let mut forest: Forest<FindMax> = super::Forest::new();
        let alice = forest.create_node(0.0);
        let bob = forest.create_node(0.0);
        forest.set_left(alice, bob);
        // Should panic because alice does not have a right child
        forest.rotate_left(alice);
    }

    #[test]
    pub fn rotate_left_root() {
        // form the following tree and rotate left on 'a':
        //         a                c
        //        / \       =>     / \
        //       b   c            a   e
        //          / \          / \
        //         d   e        b   d
        let mut forest: Forest<FindMax> = super::Forest::new();
        let a = forest.create_node(0.0);
        let b = forest.create_node(0.0);
        let c = forest.create_node(0.0);
        let d = forest.create_node(0.0);
        let e = forest.create_node(0.0);
        forest.set_left(a, b);
        forest.set_right(a, c);
        forest.set_left(c, d);
        forest.set_right(c, e);
        forest.rotate_left(a);

        assert!(forest.parent_of(c).is_none());
        assert!(forest.path_parent_of(c).is_none());
        assert_eq!(forest.left_of(c), Some(a));
        assert_eq!(forest.parent_of(a), Some(c));
        assert_eq!(forest.right_of(c), Some(e));
        assert_eq!(forest.parent_of(e), Some(c));
        assert_eq!(forest.left_of(a), Some(b));
        assert_eq!(forest.parent_of(b), Some(a));
        assert_eq!(forest.right_of(a), Some(d));
        assert_eq!(forest.parent_of(d), Some(a));
        assert!(forest.left_of(b).is_none());
        assert!(forest.right_of(b).is_none());
        assert!(forest.left_of(d).is_none());
        assert!(forest.right_of(d).is_none());
        assert!(forest.left_of(e).is_none());
        assert!(forest.right_of(e).is_none());
    }

    #[test]
    #[should_panic]
    pub fn rotate_right_invalid() {
        let mut forest: Forest<FindMax> = super::Forest::new();
        let alice = forest.create_node(0.0);
        let bob = forest.create_node(0.0);
        forest.set_right(alice, bob);
        // Should panic because alice does not have a left child
        forest.rotate_right(alice);
    }

    #[test]
    pub fn rotate_right_root() {
        // form the tree and rotate right on 'a':
        //         a                b
        //        / \       =>     / \
        //       b   e            c   a
        //      / \                  / \
        //     c   d                d   e
        let mut forest: Forest<FindMax> = super::Forest::new();
        let a = forest.create_node(0.0);
        let b = forest.create_node(0.0);
        let c = forest.create_node(0.0);
        let d = forest.create_node(0.0);
        let e = forest.create_node(0.0);
        forest.set_left(a, b);
        forest.set_right(a, e);
        forest.set_left(b, c);
        forest.set_right(b, d);
        forest.rotate_right(0);

        assert!(forest.parent_of(b).is_none());
        assert!(forest.path_parent_of(b).is_none());
        assert_eq!(forest.left_of(b), Some(c));
        assert_eq!(forest.right_of(b), Some(a));
        assert_eq!(forest.parent_of(c), Some(b));
        assert_eq!(forest.parent_of(a), Some(b));
        assert_eq!(forest.left_of(a), Some(d));
        assert_eq!(forest.right_of(a), Some(e));
        assert_eq!(forest.parent_of(d), Some(a));
        assert_eq!(forest.parent_of(e), Some(a));
        assert!(forest.left_of(c).is_none());
        assert!(forest.right_of(c).is_none());
        assert!(forest.left_of(d).is_none());
        assert!(forest.right_of(d).is_none());
        assert!(forest.left_of(e).is_none());
        assert!(forest.right_of(e).is_none());
    }

    #[test]
    #[should_panic]
    pub fn rotate_invalid() {
        let mut forest: Forest<FindMax> = super::Forest::new();
        let alice = forest.create_node(0.0);
        let bob = forest.create_node(0.0);
        forest.set_left(alice, bob);
        // Should panic because alice does not have a parent
        forest.rotate(alice);
    }

    #[test]
    pub fn rotate_parent_left() {
        // form the tree and rotate on '1':
        //      a              b
        //     /       =>       \
        //    b                  a
        let mut forest: Forest<FindMax> = super::Forest::new();
        let a = forest.create_node(0.0);
        let b = forest.create_node(0.0);
        forest.set_left(a, b);
        forest.rotate(b);
        assert!(forest.parent_of(b).is_none());
        assert!(forest.path_parent_of(b).is_none());
        assert!(forest.left_of(b).is_none());
        assert_eq!(forest.right_of(b), Some(a));
        assert_eq!(forest.parent_of(a), Some(b));
        assert!(forest.left_of(a).is_none());
        assert!(forest.right_of(a).is_none());
    }

    #[test]
    pub fn rotate_parent_right() {
        // form the tree and rotate on '1':
        //    a                 b
        //     \        =>     /
        //      b             a
        let mut forest: Forest<FindMax> = super::Forest::new();
        let a = forest.create_node(0.0);
        let b = forest.create_node(0.0);
        forest.set_right(a, b);
        forest.rotate(b);
        assert!(forest.parent_of(b).is_none());
        assert!(forest.path_parent_of(b).is_none());
        assert!(forest.right_of(b).is_none());
        assert_eq!(forest.left_of(b), Some(a));
        assert_eq!(forest.parent_of(a), Some(b));
        assert!(forest.left_of(a).is_none());
        assert!(forest.right_of(a).is_none());
    }

    #[test]
    pub fn splay_leaf() {
        // form the tree and splay on '2':
        //   a                  c
        //    \       =>       / \
        //     b              a   b
        //    /
        //   c
        let mut forest: Forest<FindMax> = super::Forest::new();
        let a = forest.create_node(0.0);
        let b = forest.create_node(0.0);
        let c = forest.create_node(0.0);
        forest.set_right(a, b);
        forest.set_left(b, c);
        forest.splay(c);
        assert!(forest.parent_of(c).is_none());
        assert!(forest.path_parent_of(c).is_none());
        assert_eq!(forest.left_of(c), Some(a));
        assert_eq!(forest.parent_of(a), Some(c));
        assert_eq!(forest.right_of(c), Some(b));
        assert_eq!(forest.parent_of(b), Some(c));
        assert!(forest.left_of(a).is_none());
        assert!(forest.right_of(a).is_none());
        assert!(forest.left_of(b).is_none());
        assert!(forest.right_of(b).is_none());
    }

    #[test]
    pub fn splay_internal_node() {
        // form the tree and splay on '1':
        //   a                  b
        //    \       =>       /
        //     b              a
        //    /                \
        //   c                  c
        let mut forest: Forest<FindMax> = super::Forest::new();
        let a = forest.create_node(0.0);
        let b = forest.create_node(0.0);
        let c = forest.create_node(0.0);
        forest.set_right(a, b);
        forest.set_left(b, c);
        forest.splay(b);
        assert!(forest.parent_of(b).is_none());
        assert!(forest.path_parent_of(b).is_none());
        assert_eq!(forest.left_of(b), Some(a));
        assert!(forest.right_of(b).is_none());
        assert_eq!(forest.parent_of(a), Some(b));
        assert_eq!(forest.right_of(a), Some(c));
        assert_eq!(forest.parent_of(c), Some(a));
        assert!(forest.left_of(a).is_none());
        assert!(forest.left_of(c).is_none());
        assert!(forest.right_of(c).is_none());
    }

    #[test]
    pub fn splay_preserve_path_pointer() {
        // Node 'a' has a path pointer to Node 'p',
        // the remaning nodes are represented in a Splay-tree as given below.
        // splaying a leaf node 'c' should result in the following tree and
        // preserve the path pointer to Node 'p':
        //    p              p
        //    |              |
        //    a              c
        //     \            / \
        //      b     =>   a   b
        //     /
        //    c
        let mut forest: Forest<FindMax> = super::Forest::new();
        let a = forest.create_node(0.0);
        let b = forest.create_node(0.0);
        let c = forest.create_node(0.0);
        let p = forest.create_node(0.0);
        forest.set_right(a, b);
        forest.set_left(b, c);
        forest.set_right(p, a);
        forest.remove_preferred_child(p);
        assert_eq!(forest.path_parent_of(a), Some(p));

        forest.splay(c);
        assert!(forest.parent_of(c).is_none());
        assert_eq!(forest.path_parent_of(c), Some(p));
    }

    #[test]
    pub fn toggle_flip() {
        let mut forest: Forest<FindMax> = super::Forest::new();
        let a = forest.create_node(0.0);
        let b = forest.create_node(0.0);
        let c = forest.create_node(0.0);
        forest.set_left(a, b);
        forest.set_right(a, c);
        forest.flip(a);
        assert_eq!(forest.left_of(a), Some(c));
        assert_eq!(forest.right_of(a), Some(b));
    }
}
