use crate::{
    access::access,
    node::{Node, Parent},
};

// creates a link between two nodes in the forest (w becomes the parent of v)
pub fn link(forest: &mut Vec<Node>, v: usize, w: usize) {
    assert!(
        v < forest.len() && w < forest.len(),
        "link: node_idx out of bounds"
    );
    access(forest, v);
    access(forest, w);
    if !matches!(forest[v].parent, Parent::Root) || v == w {
        return; // already connected
    }
    assert!(forest[w].right.is_none(), "link: w should be a root!");
    forest[w].right = Some(v);
    forest[v].parent = Parent::Node(w);
}

#[cfg(test)]
mod tests {
    use super::link;
    use crate::{
        connected::connected,
        node::{Node, Parent},
    };

    fn create_nodes(n: usize) -> Vec<Node> {
        (0..n).map(|i| Node::new(i, 0.0)).collect()
    }

    #[test]
    pub fn base_case() {
        let mut forest = create_nodes(2);
        assert!(!connected(&mut forest, 0, 1)); // not connected yet
        super::link(&mut forest, 0, 1);
        assert!(connected(&mut forest, 0, 1)); // now connected
    }

    #[test]
    pub fn already_connected() {
        // '2' has a right child '3':
        // link(0, 3) should add no link, and result in (| denotes a path pointer):
        //   0                0             0            3
        //  / \              /  |          /  |         /
        // 1   2     =>     1   2    =>   1   3   =>   0
        //      \                \           /        / \
        //       3                3         2        1   2
        //
        let mut forest = create_nodes(4);
        forest[0].left = Some(1);
        forest[0].right = Some(2);
        forest[1].parent = Parent::Node(0);
        forest[2].parent = Parent::Node(0);
        forest[2].right = Some(3);
        forest[3].parent = Parent::Node(2);
        link(&mut forest, 0, 3);
        assert!(matches!(forest[3].parent, Parent::Root));
        assert_eq!(forest[3].left, Some(0));
        assert_eq!(forest[3].right, None);
        assert!(matches!(forest[0].parent, Parent::Node(3)));
        assert_eq!(forest[0].left, Some(1));
        assert_eq!(forest[0].right, Some(2));
        assert!(matches!(forest[1].parent, Parent::Node(0)));
        assert_eq!(forest[1].left, None);
        assert_eq!(forest[1].right, None);
        assert!(matches!(forest[2].parent, Parent::Node(0)));
        assert_eq!(forest[2].left, None);
        assert_eq!(forest[2].right, None);
    }

    #[test]
    pub fn already_connected_with_path() {
        // '3' has a path pointer to '2', and '2' has a path pointer to '0':
        // link(0, 1) should add no link, and result in (| denotes a path pointer):
        //   0               0              0               3
        //  / \             / |            / |             /
        // 1   2     =>    1  2    =>     1  3      =>    0
        //     |              |             /            / \
        //     3              3            2            1   2
        //
        let mut forest = create_nodes(4);
        forest[0].left = Some(1);
        forest[0].right = Some(2);
        forest[1].parent = Parent::Node(0);
        forest[2].parent = Parent::Node(0);
        forest[3].parent = Parent::Path(2);
        link(&mut forest, 0, 3);
        assert!(matches!(forest[3].parent, Parent::Root));
        assert_eq!(forest[3].left, Some(0));
        assert_eq!(forest[3].right, None);
        assert!(matches!(forest[0].parent, Parent::Node(3)));
        assert_eq!(forest[0].left, Some(1));
        assert_eq!(forest[0].right, Some(2));
        assert!(matches!(forest[1].parent, Parent::Node(0)));
        assert_eq!(forest[1].left, None);
        assert_eq!(forest[1].right, None);
        assert!(matches!(forest[2].parent, Parent::Node(0)));
        assert_eq!(forest[2].left, None);
        assert_eq!(forest[2].right, None);
    }

    #[test]
    pub fn link_to_leftmost() {
        // Given two trees:
        //   0               3
        //  / \
        // 1   2
        // link(1, 3) should result in a single tree (| denotes a path pointer):
        //   1      3           1
        //   |                  | \
        //   0            =>    0  3
        //    \                  \
        //     2                  2
        //
        let mut forest = create_nodes(4);
        forest[0].left = Some(1);
        forest[0].right = Some(2);
        forest[1].parent = Parent::Node(0);
        forest[2].parent = Parent::Node(0);
        link(&mut forest, 3, 1);
        assert!(matches!(forest[1].parent, Parent::Root));
        assert_eq!(forest[1].right, Some(3));
        assert_eq!(forest[1].left, None);
        assert!(matches!(forest[3].parent, Parent::Node(1)));
        assert_eq!(forest[3].right, None);
        assert_eq!(forest[3].left, None);
        assert!(matches!(forest[0].parent, Parent::Path(1)));
        assert_eq!(forest[0].right, Some(2));
        assert_eq!(forest[0].left, None);
        assert!(matches!(forest[2].parent, Parent::Node(0)));
        assert_eq!(forest[2].right, None);
        assert_eq!(forest[2].left, None);
    }
}
