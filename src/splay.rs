use crate::node::{Node, Parent};

/// Rotates the subtree rooted at `node_idx` to the left:
///
/// # Panics
/// Panics if `node_idx` is out of bounds or if `node_idx` does not have a right child.
///
/// # Examples:
///  0                  1
///   \       =>       /
///    1              0
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
/// # Examples:
///    0                1
///   /        =>        \
///  1                    0
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

pub fn update_max(forest: &mut [Node], node_idx: usize) {
    let mut max_idx = node_idx;
    if let Some(left_child) = forest[node_idx].left {
        let left_max = forest[left_child].max_weight_idx;
        if forest[left_max].weight > forest[max_idx].weight {
            max_idx = left_max;
        }
    }
    if let Some(right_child) = forest[node_idx].right {
        let right_max = forest[right_child].max_weight_idx;
        if forest[right_max].weight > forest[max_idx].weight {
            max_idx = right_max;
        }
    }
    forest[node_idx].max_weight_idx = max_idx;
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
    assert!(node_idx < forest.len(), "splay: node_idx out of bounds");

    while let Parent::Node(parent_idx) = forest[node_idx].parent {
        if forest[parent_idx].left == Some(node_idx) {
            if let Parent::Node(grandparent_idx) = forest[parent_idx].parent {
                if forest[grandparent_idx].left == Some(parent_idx) {
                    // zig-zig
                    rotate_right(forest, grandparent_idx);
                    rotate_right(forest, parent_idx);
                } else {
                    // zig-zag
                    rotate_right(forest, parent_idx);
                    rotate_left(forest, grandparent_idx);
                }
            } else {
                // zig
                rotate_right(forest, parent_idx);
            }
        } else if let Parent::Node(grandparent_idx) = forest[parent_idx].parent {
            if forest[grandparent_idx].right == Some(parent_idx) {
                // zig-zig
                rotate_left(forest, grandparent_idx);
                rotate_left(forest, parent_idx);
            } else {
                // zig-zag
                rotate_left(forest, parent_idx);
                rotate_right(forest, grandparent_idx);
            }
        } else {
            // zig
            rotate_left(forest, parent_idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{rotate_left, rotate_right};
    use crate::node::{self, Node};

    fn create_nodes(n: usize) -> Vec<Node> {
        (0..n).map(|i| Node::new(i, 0.0)).collect()
    }

    #[test]
    #[should_panic]
    pub fn rotate_left_single_node() {
        // rotate left a single node, should panic:
        let mut forest = create_nodes(1);
        rotate_left(&mut forest, 0);
    }

    #[test]
    pub fn rotate_left_with_parent() {
        // connect two nodes and rotate left on '0':
        //      0                  2
        //     / \      =>        /
        //    1   2              0
        //                      /
        //                     1
        let mut forest = create_nodes(3);
        forest[0].left = Some(1);
        forest[0].right = Some(2);
        forest[1].parent = node::Parent::Node(0);
        forest[2].parent = node::Parent::Node(0);
        rotate_left(&mut forest, 0);
        assert!(matches!(forest[2].parent, node::Parent::Root));
        assert_eq!(forest[2].left, Some(0));
        assert_eq!(forest[2].right, None);
        assert!(matches!(forest[0].parent, node::Parent::Node(2)));
        assert_eq!(forest[0].left, Some(1));
        assert_eq!(forest[0].right, None);
        assert!(matches!(forest[1].parent, node::Parent::Node(0)));
        assert!(forest[1].left.is_none());
        assert!(forest[1].right.is_none());
    }

    #[test]
    #[should_panic]
    pub fn rotate_right_single_node() {
        // rotate right a single node, should panic:
        let mut forest = create_nodes(1);
        rotate_right(&mut forest, 0);
        assert!(forest[0].left.is_none());
        assert!(forest[0].right.is_none());
        assert!(matches!(forest[0].parent, node::Parent::Root));
    }

    #[test]
    pub fn rotate_right_with_parent() {
        // connect two nodes and rotate left on '0':
        //      0               1
        //     / \      =>       \
        //    1   2               0
        //                         \
        //                          2
        let mut forest = create_nodes(3);
        forest[0].left = Some(1);
        forest[0].right = Some(2);
        forest[1].parent = node::Parent::Node(0);
        forest[2].parent = node::Parent::Node(0);
        rotate_right(&mut forest, 0);
        assert!(matches!(forest[1].parent, node::Parent::Root));
        assert_eq!(forest[1].left, None);
        assert_eq!(forest[1].right, Some(0));
        assert!(matches!(forest[0].parent, node::Parent::Node(1)));
        assert_eq!(forest[0].left, None);
        assert_eq!(forest[0].right, Some(2));
        assert!(matches!(forest[2].parent, node::Parent::Node(0)));
        assert!(forest[2].left.is_none());
        assert!(forest[2].right.is_none());
    }

    #[test]
    pub fn splay_single_node() {
        // splay a single node, should do nothing:
        let mut forest = create_nodes(1);
        super::splay(&mut forest, 0);
        assert!(forest[0].left.is_none());
        assert!(forest[0].right.is_none());
        assert!(matches!(forest[0].parent, node::Parent::Root));
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
}
