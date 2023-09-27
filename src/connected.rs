use crate::{
    access::access,
    node::{Node, Parent},
};

pub fn connected(forest: &mut Vec<Node>, v: usize, w: usize) -> bool {
    assert!(
        v < forest.len() || w < forest.len(),
        "splay: node_idx out of bounds"
    );
    access(forest, v);
    access(forest, w);
    // if v is still root of its tree, then it is not connected to w:
    !matches!(forest[v].parent, Parent::Root) || v == w
}

#[cfg(test)]
mod tests {
    use crate::node::{Node, Parent};

    fn create_nodes(n: usize) -> Vec<Node> {
        (0..n).map(|i| Node::new(i, 0.0)).collect()
    }

    #[test]
    pub fn base_case() {
        // check two nodes, should return false
        let mut forest = create_nodes(2);
        assert!(!super::connected(&mut forest, 0, 1));
    }

    #[test]
    pub fn connected_with_root() {
        // check three nodes, one is root:
        //    0
        //   / \
        //  1   2
        let mut forest = create_nodes(3);
        forest[0].left = Some(1);
        forest[0].right = Some(2);
        forest[1].parent = Parent::Node(0);
        forest[2].parent = Parent::Node(0);

        assert!(super::connected(&mut forest, 0, 1));
        assert!(super::connected(&mut forest, 0, 2));
        assert!(super::connected(&mut forest, 1, 2));
        assert!(super::connected(&mut forest, 0, 0));
        assert!(super::connected(&mut forest, 1, 1));
        assert!(super::connected(&mut forest, 2, 2));
    }

    #[test]
    pub fn connected_with_path_pointers() {
        // check two trees that are connected by a path pointer
        //     0
        //    / \
        //   1   2
        //       |
        //       3
        let mut forest = create_nodes(4);
        forest[0].left = Some(1);
        forest[0].right = Some(2);
        forest[1].parent = Parent::Node(0);
        forest[2].parent = Parent::Node(0);
        forest[3].parent = Parent::Path(2);

        assert!(super::connected(&mut forest, 0, 3));
        assert!(super::connected(&mut forest, 1, 3));
        assert!(super::connected(&mut forest, 2, 3));
    }
}
