use crate::{
    node::{Node, Parent},
    splay::splay,
};

// constructs a path from the root to the node at idx
pub fn access(forest: &mut Vec<Node>, node_idx: usize) {
    splay(forest, node_idx);

    if let Some(right_idx) = forest[node_idx].right {
        forest[node_idx].right = None;
        forest[right_idx].parent = Parent::Path(node_idx);
    }

    while let Parent::Path(path_idx) = forest[node_idx].parent {
        splay(forest, path_idx);
        // detach the right child of the path parent
        if let Some(right_idx) = forest[path_idx].right {
            forest[right_idx].parent = Parent::Path(path_idx);
            forest[path_idx].right = None;
        }

        // attach the node as the path parent's right child
        forest[path_idx].right = Some(node_idx);
        forest[node_idx].parent = Parent::Node(path_idx);
        splay(forest, node_idx);
    }
}

#[cfg(test)]
mod tests {
    use crate::node::{Node, Parent};

    fn create_nodes(n: usize) -> Vec<Node> {
        (0..n).map(|i| Node::new(i, 0.0)).collect()
    }

    #[test]
    pub fn access_base_case() {
        // access a single node, should do nothing
        let mut forest = create_nodes(1);
        super::access(&mut forest, 0);
        assert!(matches!(forest[0].parent, Parent::Root));
    }

    #[test]
    pub fn access_splay_leaf() {
        let mut forest = create_nodes(3);
        // '1' has a path pointer to '0', '1' has a right child '2'.
        // after access(2), '2' should be the root of the tree:
        //    0             0             0               2
        //    |             |              \             /
        //    1      =>     2      =>       2     =>    0
        //     \           /               /             \
        //      2         1               1               1
        forest[1].parent = Parent::Path(0);
        forest[1].right = Some(2);
        forest[2].parent = Parent::Node(1);
        super::access(&mut forest, 2);
        assert!(matches!(forest[2].parent, Parent::Root));
        assert_eq!(forest[2].right, None);
        assert_eq!(forest[2].left, Some(0));
        assert!(matches!(forest[0].parent, Parent::Node(2)));
        assert_eq!(forest[0].left, None);
        assert_eq!(forest[0].right, Some(1));
        assert!(matches!(forest[1].parent, Parent::Node(0)));
        assert_eq!(forest[1].left, None);
        assert_eq!(forest[1].right, None);
    }
}
