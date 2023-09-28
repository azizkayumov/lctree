use crate::{
    access::access,
    node::{Node, Parent},
};

pub fn cut(forest: &mut Vec<Node>, v: usize) {
    self::access(forest, v);
    if let Some(left) = forest[v].left {
        forest[left].parent = Parent::Root;
        forest[v].left = None;
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        connected::connected,
        cut::cut,
        link::link,
        node::{Node, Parent},
    };

    fn create_nodes(n: usize) -> Vec<Node> {
        (0..n).map(|i| Node::new(i, 0.0)).collect()
    }

    #[test]
    pub fn base_case() {
        let mut forest = create_nodes(2);
        assert!(!connected(&mut forest, 0, 1)); // not connected yet

        link(&mut forest, 0, 1);
        assert!(matches!(forest[1].parent, Parent::Root));
        assert_eq!(forest[1].right, Some(0));
        assert_eq!(forest[1].left, None);
        assert!(matches!(forest[0].parent, Parent::Node(1)));
        //  1
        //   \       <= link(0, 1)
        //    0

        assert!(connected(&mut forest, 0, 1)); // now connected
        assert!(matches!(forest[1].parent, Parent::Root));
        assert_eq!(forest[1].right, None);
        assert_eq!(forest[1].left, None);
        assert!(matches!(forest[0].parent, Parent::Path(1)));
        //    0             1
        //   /      =>      |
        //  1               0

        super::cut(&mut forest, 0);
        assert!(matches!(forest[0].parent, Parent::Root));
        assert_eq!(forest[0].right, None);
        //    0           0
        //   /     =>
        //  1           1
        assert!(!connected(&mut forest, 0, 1)); // now disconnected
    }

    #[test]
    pub fn cut_into_two_subtrees() {
        let mut forest = create_nodes(5);
        forest[0].left = Some(1);
        forest[1].parent = Parent::Node(0);
        forest[1].left = Some(2);
        forest[2].parent = Parent::Node(1);
        forest[3].right = Some(4);
        forest[4].parent = Parent::Node(3);
        // Given two trees:
        //       0       3
        //      /         \
        //     1           4
        //    /
        //   2
        link(&mut forest, 2, 3);
        // link(2, 3) should result in:
        //      3
        //      | \
        //      4  2
        //         |
        //         1
        //          \
        //           0
        assert!(matches!(forest[3].parent, Parent::Root));
        assert!(matches!(forest[4].parent, Parent::Path(3)));
        assert_eq!(forest[3].right, Some(2));
        assert!(matches!(forest[2].parent, Parent::Node(3)));
        assert!(matches!(forest[1].parent, Parent::Path(2)));
        assert_eq!(forest[1].right, Some(0));
        assert!(connected(&mut forest, 2, 3));
        cut(&mut forest, 2);
        assert!(!connected(&mut forest, 2, 3));
        assert!(!connected(&mut forest, 2, 4));
    }
}
