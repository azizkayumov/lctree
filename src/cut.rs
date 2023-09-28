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
        node::{Node, Parent}, link::link,
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
}