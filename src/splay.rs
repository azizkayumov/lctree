use crate::{
    node::{Node, Parent},
    path::update_max,
};

/// Rotates the subtree rooted at `node_idx` to the left:
///
/// # Panics
/// Panics if `node_idx` is out of bounds or if `node_idx` does not have a right child.
///
/// Example:
//         0                2
//        / \       =>     / \
//       1   2            0   4
//          / \          / \
//         3   4        1   3
fn rotate_left(forest: &mut [Node], node_idx: usize) {
    assert!(
        node_idx < forest.len(),
        "rotate_left: node_idx out of bounds"
    );
    assert!(
        forest[node_idx].right.is_some(),
        "rotate_left: node_idx does not have a right child"
    );

    let right_child = forest[node_idx].right.unwrap();
    if let Parent::Node(parent_idx) = forest[node_idx].parent {
        if forest[parent_idx].left == Some(node_idx) {
            forest[parent_idx].left = Some(right_child);
        } else {
            forest[parent_idx].right = Some(right_child);
        }
    }

    forest[node_idx].right = forest[right_child].left;
    forest[right_child].left = Some(node_idx);
    forest[right_child].parent = forest[node_idx].parent;
    forest[node_idx].parent = Parent::Node(right_child);

    if let Some(new_right_child) = forest[node_idx].right {
        forest[new_right_child].parent = Parent::Node(node_idx);
    }

    // update aggregate information
    update_max(forest, node_idx);
    update_max(forest, right_child);
}

/// Rotates the subtree rooted at `node_idx` to the right:
///
/// # Panics
/// Panics if `node_idx` is out of bounds or if `node_idx` does not have a left child.
///
//  Example:
//         0                1
//        / \      =>      / \
//       1   4            2   0
//      / \                  / \
//     2   3                3   4
fn rotate_right(forest: &mut [Node], node_idx: usize) {
    assert!(
        node_idx < forest.len(),
        "rotate_right: node_idx out of bounds"
    );
    assert!(
        forest[node_idx].left.is_some(),
        "rotate_right: node_idx does not have a left child"
    );

    let left_child = forest[node_idx].left.unwrap();
    if let Parent::Node(parent_idx) = forest[node_idx].parent {
        if forest[parent_idx].left == Some(node_idx) {
            forest[parent_idx].left = Some(left_child);
        } else {
            forest[parent_idx].right = Some(left_child);
        }
    }

    forest[node_idx].left = forest[left_child].right;
    forest[left_child].right = Some(node_idx);
    forest[left_child].parent = forest[node_idx].parent;
    forest[node_idx].parent = Parent::Node(left_child);

    if let Some(new_left_child) = forest[node_idx].left {
        forest[new_left_child].parent = Parent::Node(node_idx);
    }

    // update aggregate information
    update_max(forest, node_idx);
    update_max(forest, left_child);
}

/// Rotates the parent of `node_idx` to the right or left, depending on the relationship between:
///
/// # Panics
/// Panics if `node_idx` is out of bounds or if `node_idx` does not have a parent.
//  Example:
//    0                1
//   /        =>        \
//  1                    0
fn rotate(forest: &mut [Node], node_idx: usize) {
    assert!(node_idx < forest.len(), "rotate: node_idx out of bounds");
    assert!(
        matches!(forest[node_idx].parent, Parent::Node(_)),
        "rotate: node_idx does not have a parent"
    );

    if let Parent::Node(parent_idx) = forest[node_idx].parent {
        if forest[parent_idx].left == Some(node_idx) {
            rotate_right(forest, parent_idx);
        } else {
            rotate_left(forest, parent_idx);
        }
    }
}

/// Splays the subtree rooted at `node_idx`, making it the new root of the tree.
///
/// # Examples:
///  splaying on '2':
///   0                  2
///    \       =>       / \
///     1              0   1
///    /
///   2
pub fn splay(forest: &mut [Node], node_idx: usize) {
    while let Parent::Node(parent_idx) = forest[node_idx].parent {
        if let Parent::Node(grandparent_idx) = forest[parent_idx].parent {
            unflip(forest, grandparent_idx);
        }
        unflip(forest, parent_idx);
        unflip(forest, node_idx);

        if let Parent::Node(grandparent_idx) = forest[parent_idx].parent {
            if (forest[grandparent_idx].left == Some(parent_idx))
                == (forest[parent_idx].left == Some(node_idx))
            {
                // zig-zig (same direction):
                rotate(forest, parent_idx);
                rotate(forest, node_idx);
            } else {
                // zig-zag:
                rotate(forest, node_idx);
                rotate(forest, node_idx);
            }
        } else {
            // zig
            rotate(forest, node_idx);
        }
    }
    unflip(forest, node_idx);
}

/// Unflips the subtree rooted at `node_idx`, swapping the left and right children.
/// The children's `flipped` flag is also toggled to propogate the change down the tree.
pub fn unflip(forest: &mut [Node], node_idx: usize) {
    if forest[node_idx].flipped {
        forest[node_idx].flipped = false;
        std::mem::swap(&mut forest[node_idx].left, &mut forest[node_idx].right);
        if let Some(left_child) = forest[node_idx].left {
            forest[left_child].flipped ^= true;
        }
        if let Some(right_child) = forest[node_idx].right {
            forest[right_child].flipped ^= true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{rotate, rotate_left, rotate_right, unflip};
    use crate::node::{self, Node};

    fn create_nodes(n: usize) -> Vec<Node> {
        (0..n).map(|i| Node::new(i, 0.0)).collect()
    }

    #[test]
    pub fn rotate_left_root() {
        // form the following tree and rotate left on '0':
        //         0                2
        //        / \       =>     / \
        //       1   2            0   4
        //          / \          / \
        //         3   4        1   3
        let mut forest = create_nodes(5);
        forest[0].left = Some(1);
        forest[0].right = Some(2);
        forest[1].parent = node::Parent::Node(0);
        forest[2].parent = node::Parent::Node(0);
        forest[2].left = Some(3);
        forest[2].right = Some(4);
        forest[3].parent = node::Parent::Node(2);
        forest[4].parent = node::Parent::Node(2);
        rotate_left(&mut forest, 0);
        assert!(matches!(forest[2].parent, node::Parent::Root));
        assert_eq!(forest[2].left, Some(0));
        assert!(matches!(forest[0].parent, node::Parent::Node(2)));
        assert_eq!(forest[2].right, Some(4));
        assert!(matches!(forest[4].parent, node::Parent::Node(2)));
        assert_eq!(forest[0].left, Some(1));
        assert!(matches!(forest[1].parent, node::Parent::Node(0)));
        assert_eq!(forest[0].right, Some(3));
        assert!(matches!(forest[3].parent, node::Parent::Node(0)));
        assert!(forest[1].left.is_none());
        assert!(forest[1].right.is_none());
        assert!(forest[3].left.is_none());
        assert!(forest[3].right.is_none());
    }

    #[test]
    pub fn rotate_right_root() {
        // form the tree and rotate left on '0':
        //         0                1
        //        / \       =>     / \
        //       1   4            2   0
        //      / \                  / \
        //     2   3                3   4
        let mut forest = create_nodes(5);
        forest[0].left = Some(1);
        forest[0].right = Some(4);
        forest[1].parent = node::Parent::Node(0);
        forest[4].parent = node::Parent::Node(0);
        forest[1].left = Some(2);
        forest[1].right = Some(3);
        forest[2].parent = node::Parent::Node(1);
        forest[3].parent = node::Parent::Node(1);
        rotate_right(&mut forest, 0);
        assert!(matches!(forest[1].parent, node::Parent::Root));
        assert_eq!(forest[1].left, Some(2));
        assert!(matches!(forest[2].parent, node::Parent::Node(1)));
        assert_eq!(forest[1].right, Some(0));
        assert!(matches!(forest[0].parent, node::Parent::Node(1)));
        assert_eq!(forest[0].left, Some(3));
        assert!(matches!(forest[3].parent, node::Parent::Node(0)));
        assert_eq!(forest[0].right, Some(4));
        assert!(matches!(forest[4].parent, node::Parent::Node(0)));
        assert!(forest[3].left.is_none());
        assert!(forest[3].right.is_none());
        assert!(forest[4].left.is_none());
        assert!(forest[4].right.is_none());
    }

    #[test]
    pub fn rotate_parent_left() {
        // form the tree and rotate on '1':
        //      0              1
        //     /       =>       \
        //    1                  0
        let mut forest = create_nodes(2);
        forest[0].left = Some(1);
        forest[1].parent = node::Parent::Node(0);
        rotate(&mut forest, 1);
        assert!(matches!(forest[1].parent, node::Parent::Root));
        assert_eq!(forest[1].left, None);
        assert_eq!(forest[1].right, Some(0));
        assert!(matches!(forest[0].parent, node::Parent::Node(1)));
        assert!(forest[0].left.is_none());
        assert!(forest[0].right.is_none());
    }

    #[test]
    pub fn rotate_parent_right() {
        // form the tree and rotate on '1':
        //    0                 1
        //     \        =>     /
        //      1             0
        let mut forest = create_nodes(2);
        forest[0].right = Some(1);
        forest[1].parent = node::Parent::Node(0);
        rotate(&mut forest, 1);
        assert!(matches!(forest[1].parent, node::Parent::Root));
        assert_eq!(forest[1].left, Some(0));
        assert_eq!(forest[1].right, None);
        assert!(matches!(forest[0].parent, node::Parent::Node(1)));
        assert!(forest[0].left.is_none());
        assert!(forest[0].right.is_none());
    }

    #[test]
    pub fn splay_leaf() {
        // form the tree and splay on '2':
        //   0                  2
        //    \       =>       / \
        //     1              0   1
        //    /
        //   2
        let mut forest = create_nodes(3);
        forest[0].right = Some(1);
        forest[1].parent = node::Parent::Node(0);
        forest[1].left = Some(2);
        forest[2].parent = node::Parent::Node(1);
        super::splay(&mut forest, 2);
        assert!(matches!(forest[2].parent, node::Parent::Root));
        assert_eq!(forest[2].left, Some(0));
        assert_eq!(forest[2].right, Some(1));
        assert!(matches!(forest[0].parent, node::Parent::Node(2)));
        assert!(matches!(forest[1].parent, node::Parent::Node(2)));
    }

    #[test]
    pub fn splay_internal_node() {
        // form the tree and splay on '1':
        //   0                  1
        //    \       =>       /
        //     1              0
        //    /                \
        //   2                  2
        let mut forest = create_nodes(3);
        forest[0].right = Some(1);
        forest[1].parent = node::Parent::Node(0);
        forest[1].left = Some(2);
        forest[2].parent = node::Parent::Node(1);
        super::splay(&mut forest, 1);
        assert!(matches!(forest[1].parent, node::Parent::Root));
        assert_eq!(forest[1].left, Some(0));
        assert_eq!(forest[1].right, None);
        assert!(matches!(forest[0].parent, node::Parent::Node(1)));
        assert_eq!(forest[0].left, None);
        assert_eq!(forest[0].right, Some(2));
        assert!(matches!(forest[2].parent, node::Parent::Node(0)));
        assert!(forest[2].left.is_none());
        assert!(forest[2].right.is_none());
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
        let mut forest = create_nodes(6);
        forest[0].parent = node::Parent::Path(6);
        forest[0].right = Some(1);
        forest[1].parent = node::Parent::Node(0);
        forest[1].left = Some(2);
        forest[2].parent = node::Parent::Node(1);
        forest[2].left = Some(3);
        forest[3].parent = node::Parent::Node(2);
        forest[2].right = Some(4);
        forest[4].parent = node::Parent::Node(2);
        super::splay(&mut forest, 4);
        // The path pointer to Node '6' should be preserved:
        assert!(matches!(forest[4].parent, node::Parent::Path(6)));
        // The rest of the tree should be a rotated Splay-tree:
        assert_eq!(forest[4].left, Some(0));
        assert!(matches!(forest[0].parent, node::Parent::Node(4)));
        assert_eq!(forest[4].right, Some(1));
        assert!(matches!(forest[1].parent, node::Parent::Node(4)));
        assert_eq!(forest[0].right, Some(2));
        assert!(matches!(forest[2].parent, node::Parent::Node(0)));
        assert_eq!(forest[2].left, Some(3));
        assert!(matches!(forest[3].parent, node::Parent::Node(2)));
    }

    #[test]
    pub fn toggle_flip() {
        let mut forest = create_nodes(3);
        forest[0].left = Some(1);
        forest[0].right = Some(2);
        forest[1].parent = node::Parent::Node(0);
        forest[2].parent = node::Parent::Node(0);
        forest[0].flipped = true;
        unflip(&mut forest, 0);
        assert!(!forest[0].flipped);
        assert!(forest[1].flipped);
        assert!(forest[2].flipped);
        assert_eq!(forest[0].left, Some(2));
        assert_eq!(forest[0].right, Some(1));
    }
}
