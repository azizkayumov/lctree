use crate::node::{Node, Parent};

pub fn rotate_left(forest: &mut Vec<Node>, node_idx: usize) {
    // base cases:
    // - node_idx is out of bounds
    // - node_idx should have a right child
    if node_idx >= forest.len() || forest[node_idx].right.is_none() {
        return;
    }

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
}

pub fn rotate_right(forest: &mut Vec<Node>, node_idx: usize) {
    // base cases:
    // - node_idx is out of bounds
    // - node_idx should have a left child
    if node_idx >= forest.len() || forest[node_idx].left.is_none() {
        return;
    }

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
}

pub fn splay(forest: &mut Vec<Node>, node_idx: usize) {
    assert!(node_idx < forest.len(), "splay: node_idx out of bounds");
    // todo: implement splay
}

#[cfg(test)]
mod tests {
    use crate::node::{Node, self};
    use super::{rotate_right, rotate_left};

    fn create_nodes() -> Vec<Node> {
        let mut forest = Vec::new();
        for i in 0..5 {
            forest.push(Node::new(i, 0.0));
        }
        forest
    }

    #[test]
    pub fn rotate_left_test() {
        // try to rotate a single node
        // should do nothing
        let mut forest = create_nodes();
        rotate_left(&mut forest, 0);
        assert_eq!(forest[0].right, None);
        assert!(forest[0].right.is_none());
        assert!(matches!(forest[0].parent, node::Parent::Root));

        // Connect two nodes and rotate left
        let mut forest = create_nodes();
        forest[0].right = Some(1);
        forest[1].parent = node::Parent::Node(0);
        rotate_left(&mut forest, 0);
        assert_eq!(forest[0].right, None);
        assert_eq!(forest[1].left, Some(0));
        assert!(matches!(forest[0].parent, node::Parent::Node(1)));
        assert!(matches!(forest[1].parent, node::Parent::Root));

        // Connect all nodes and rotate left
        let mut forest = create_nodes();
        forest[0].right = Some(1);
        forest[1].parent = node::Parent::Node(0);
        forest[1].right = Some(2);
        forest[2].parent = node::Parent::Node(1);
        forest[2].left = Some(3);
        forest[3].parent = node::Parent::Node(2);
        forest[2].right = Some(4);
        forest[4].parent = node::Parent::Node(2);
        rotate_left(&mut forest, 1);
        assert!(matches!(forest[0].parent, node::Parent::Root));
        assert_eq!(forest[0].right, Some(2));
        assert!(matches!(forest[2].parent, node::Parent::Node(0)));
        assert_eq!(forest[2].left, Some(1));
        assert!(matches!(forest[1].parent, node::Parent::Node(2)));
        assert_eq!(forest[2].right, Some(4));
        assert!(matches!(forest[4].parent, node::Parent::Node(2)));
        assert_eq!(forest[1].right, Some(3));
        assert!(matches!(forest[3].parent, node::Parent::Node(1)));
    }

    #[test]
    pub fn rotate_right_test() {
        // try to rotate a single node
        // should do nothing
        let mut forest = create_nodes();
        rotate_right(&mut forest, 0);
        assert!(forest[0].left.is_none());
        assert!(forest[0].right.is_none());
        assert!(matches!(forest[0].parent, node::Parent::Root));

        // connect two nodes and rotate right
        let mut forest = create_nodes();
        forest[0].left = Some(1);
        forest[1].parent = node::Parent::Node(0);
        rotate_right(&mut forest, 0);
        assert_eq!(forest[0].left, None);
        assert_eq!(forest[1].right, Some(0));
        assert!(matches!(forest[0].parent, node::Parent::Node(1)));
        assert!(matches!(forest[1].parent, node::Parent::Root));

        // connect all nodes and rotate right
        let mut forest = create_nodes();
        forest[0].right = Some(1);
        forest[1].parent = node::Parent::Node(0);
        forest[1].left = Some(2);
        forest[2].parent = node::Parent::Node(1);
        forest[2].left = Some(3);
        forest[3].parent = node::Parent::Node(2);
        forest[2].right = Some(4);
        forest[4].parent = node::Parent::Node(2);
        rotate_right(&mut forest, 1);
        assert!(matches!(forest[0].parent, node::Parent::Root));
        assert_eq!(forest[0].right, Some(2));
        assert!(matches!(forest[2].parent, node::Parent::Node(0)));
        assert_eq!(forest[2].left, Some(3));
        assert!(matches!(forest[3].parent, node::Parent::Node(2)));
        assert_eq!(forest[2].right, Some(1));
        assert!(matches!(forest[1].parent, node::Parent::Node(2)));
        assert_eq!(forest[1].left, Some(4));
        assert!(matches!(forest[4].parent, node::Parent::Node(1)));
    }
}