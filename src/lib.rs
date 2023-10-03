#![allow(dead_code, clippy::module_name_repetitions)] // yes, I want to name my structs with the same name as the file
mod node;
mod path;
mod splay;

use crate::{
    node::{Node, Parent},
    splay::splay,
};

pub struct LinkCutTree {
    forest: Vec<Node>,
}

impl LinkCutTree {
    #[must_use]
    pub fn new(n: usize) -> Self {
        let nodes = (0..n).map(|i| Node::new(i, 0.0)).collect();
        Self { forest: nodes }
    }

    // Constructs a path from a node to the root of the tree.
    pub fn access(&mut self, v: usize) {
        splay(&mut self.forest, v);

        if let Some(right_idx) = self.forest[v].right {
            self.forest[v].right = None;
            self.forest[right_idx].parent = Parent::Path(v);
        }

        while let Parent::Path(path_idx) = self.forest[v].parent {
            splay(&mut self.forest, path_idx);
            // detach the right child of the path parent
            if let Some(right_idx) = self.forest[path_idx].right {
                self.forest[right_idx].parent = Parent::Path(path_idx);
                self.forest[path_idx].right = None;
            }

            // attach the node as the path parent's right child
            self.forest[path_idx].right = Some(v);
            self.forest[v].parent = Parent::Node(path_idx);
            splay(&mut self.forest, v);
        }
    }

    // Creates a link between two nodes in the forest (w becomes the parent of v)
    pub fn link(&mut self, v: usize, w: usize) {
        self.access(v);
        self.access(w);
        if !matches!(self.forest[v].parent, Parent::Root) || v == w {
            return; // already connected
        }
        self.forest[w].right = Some(v);
        self.forest[v].parent = Parent::Node(w);
    }

    // Checks if v and w are connected in the forest
    pub fn connected(&mut self, v: usize, w: usize) -> bool {
        self.access(v); // v is now the root of the tree
        self.access(w);
        // if access(w) messed with the root of the tree, then v and w are connected:
        !matches!(self.forest[v].parent, Parent::Root) || v == w
    }

    // Cuts the link between v and its parent.
    pub fn cut(&mut self, v: usize) {
        self.access(v);
        if let Some(left) = self.forest[v].left {
            self.forest[left].parent = Parent::Root;
            self.forest[v].left = None;
        }
    }

    // Finds the maximum weight in the path from v and its parent
    pub fn findmax(&mut self, v: usize) -> usize {
        // let mut cur = v;
        // while let Some(left_idx) = self.forest[cur].left {
        //     let cur_max_idx = self.forest[v].max_weight_idx;
        //     if self.forest[left_idx].weight > self.forest[cur_max_idx].weight {
        //         self.forest[v].max_weight_idx = left_idx;
        //     }
        //     cur = left_idx;
        // }
        self.access(v);
        let mut cur = v;
        while let Some(left_idx) = self.forest[cur].left {
            let cur_max_idx = self.forest[v].max_weight_idx;
            let left_max_idx = self.forest[left_idx].max_weight_idx;
            if self.forest[left_max_idx].weight > self.forest[cur_max_idx].weight {
                self.forest[v].max_weight_idx = left_max_idx;
            }
            cur = left_idx;
        }
        self.forest[v].max_weight_idx
    }
}

#[cfg(test)]
mod tests {
    use rand::seq::SliceRandom;

    use crate::node::Parent;

    #[test]
    pub fn access_base_case() {
        // access a single node, should do nothing
        let mut tree = super::LinkCutTree::new(1);
        tree.access(0);
        assert!(matches!(tree.forest[0].parent, Parent::Root));
    }

    #[test]
    pub fn access_leaf() {
        let mut tree = super::LinkCutTree::new(3);
        // '1' has a path pointer to '0', '1' has a right child '2'.
        // after access(2), '2' should be the root of the tree:
        //    0             0             0               2
        //    |             |              \             /
        //    1      =>     2      =>       2     =>    0
        //     \           /               /             \
        //      2         1               1               1
        tree.forest[1].parent = Parent::Path(0);
        tree.forest[1].right = Some(2);
        tree.forest[2].parent = Parent::Node(1);
        tree.access(2);
        assert!(matches!(tree.forest[2].parent, Parent::Root));
        assert_eq!(tree.forest[2].right, None);
        assert_eq!(tree.forest[2].left, Some(0));
        assert!(matches!(tree.forest[0].parent, Parent::Node(2)));
        assert_eq!(tree.forest[0].left, None);
        assert_eq!(tree.forest[0].right, Some(1));
        assert!(matches!(tree.forest[1].parent, Parent::Node(0)));
        assert_eq!(tree.forest[1].left, None);
        assert_eq!(tree.forest[1].right, None);
    }

    #[test]
    pub fn link_base_case() {
        let mut tree = super::LinkCutTree::new(2);
        assert!(!tree.connected(0, 1)); // not connected yet
        tree.link(0, 1);
        assert!(tree.connected(0, 1)); // now connected
    }

    #[test]
    pub fn link_already_connected() {
        // '2' has a right child '3':
        // link(0, 3) should add no link, and result in (| denotes a path pointer):
        //   0                0             0            3
        //  / \              /  |          /  |         /
        // 1   2     =>     1   2    =>   1   3   =>   0
        //      \                \           /        / \
        //       3                3         2        1   2
        //
        let mut tree = super::LinkCutTree::new(4);
        tree.forest[0].left = Some(1);
        tree.forest[0].right = Some(2);
        tree.forest[1].parent = Parent::Node(0);
        tree.forest[2].parent = Parent::Node(0);
        tree.forest[2].right = Some(3);
        tree.forest[3].parent = Parent::Node(2);
        tree.link(0, 3);
        assert!(matches!(tree.forest[3].parent, Parent::Root));
        assert_eq!(tree.forest[3].left, Some(0));
        assert_eq!(tree.forest[3].right, None);
        assert!(matches!(tree.forest[0].parent, Parent::Node(3)));
        assert_eq!(tree.forest[0].left, Some(1));
        assert_eq!(tree.forest[0].right, Some(2));
        assert!(matches!(tree.forest[1].parent, Parent::Node(0)));
        assert_eq!(tree.forest[1].left, None);
        assert_eq!(tree.forest[1].right, None);
        assert!(matches!(tree.forest[2].parent, Parent::Node(0)));
        assert_eq!(tree.forest[2].left, None);
        assert_eq!(tree.forest[2].right, None);
    }

    #[test]
    pub fn link_already_connected_with_path() {
        // '3' has a path pointer to '2', and '2' has a path pointer to '0':
        // link(0, 3) should add no link, and result in (| denotes a path pointer):
        //   0               0              0               3
        //  / \             / |            / |             /
        // 1   2     =>    1  2    =>     1  3      =>    0
        //     |              |             /            / \
        //     3              3            2            1   2
        //
        let mut tree = super::LinkCutTree::new(4);
        tree.forest[0].left = Some(1);
        tree.forest[0].right = Some(2);
        tree.forest[1].parent = Parent::Node(0);
        tree.forest[2].parent = Parent::Node(0);
        tree.forest[3].parent = Parent::Path(2);
        tree.link(0, 3);
        assert!(matches!(tree.forest[3].parent, Parent::Root));
        assert_eq!(tree.forest[3].left, Some(0));
        assert_eq!(tree.forest[3].right, None);
        assert!(matches!(tree.forest[0].parent, Parent::Node(3)));
        assert_eq!(tree.forest[0].left, Some(1));
        assert_eq!(tree.forest[0].right, Some(2));
        assert!(matches!(tree.forest[1].parent, Parent::Node(0)));
        assert_eq!(tree.forest[1].left, None);
        assert_eq!(tree.forest[1].right, None);
        assert!(matches!(tree.forest[2].parent, Parent::Node(0)));
        assert_eq!(tree.forest[2].left, None);
        assert_eq!(tree.forest[2].right, None);
    }

    #[test]
    pub fn link_to_leftmost() {
        // Given two trees:
        //   0               3
        //  / \
        // 1   2
        // link(3, 1) should result in a single tree (| denotes a path pointer):
        //   1      3           1
        //   |                  | \
        //   0            =>    0  3
        //    \                  \
        //     2                  2
        //
        let mut tree = super::LinkCutTree::new(4);
        tree.forest[0].left = Some(1);
        tree.forest[0].right = Some(2);
        tree.forest[1].parent = Parent::Node(0);
        tree.forest[2].parent = Parent::Node(0);
        tree.link(3, 1);
        assert!(matches!(tree.forest[1].parent, Parent::Root));
        assert_eq!(tree.forest[1].right, Some(3));
        assert_eq!(tree.forest[1].left, None);
        assert!(matches!(tree.forest[3].parent, Parent::Node(1)));
        assert_eq!(tree.forest[3].right, None);
        assert_eq!(tree.forest[3].left, None);
        assert!(matches!(tree.forest[0].parent, Parent::Path(1)));
        assert_eq!(tree.forest[0].right, Some(2));
        assert_eq!(tree.forest[0].left, None);
        assert!(matches!(tree.forest[2].parent, Parent::Node(0)));
        assert_eq!(tree.forest[2].right, None);
        assert_eq!(tree.forest[2].left, None);
    }

    #[test]
    pub fn connected_with_root() {
        // check three nodes, one is root:
        //    0
        //   / \
        //  1   2
        let mut tree = super::LinkCutTree::new(3);
        tree.forest[0].left = Some(1);
        tree.forest[0].right = Some(2);
        tree.forest[1].parent = Parent::Node(0);
        tree.forest[2].parent = Parent::Node(0);

        assert!(tree.connected(0, 1));
        assert!(tree.connected(0, 2));
        assert!(tree.connected(1, 2));
        assert!(tree.connected(0, 0));
        assert!(tree.connected(1, 1));
        assert!(tree.connected(2, 2));
    }

    #[test]
    pub fn connected_with_path_pointers() {
        // check two trees that are connected by a path pointer
        //     0
        //    / \
        //   1   2
        //       |
        //       3
        let mut tree = super::LinkCutTree::new(4);
        tree.forest[0].left = Some(1);
        tree.forest[0].right = Some(2);
        tree.forest[1].parent = Parent::Node(0);
        tree.forest[2].parent = Parent::Node(0);
        tree.forest[3].parent = Parent::Path(2);

        assert!(tree.connected(0, 3));
        assert!(tree.connected(1, 3));
        assert!(tree.connected(2, 3));
    }

    #[test]
    pub fn cut_base_case() {
        let mut tree = super::LinkCutTree::new(2);
        assert!(!tree.connected(0, 1)); // not connected yet

        tree.link(0, 1);
        assert!(matches!(tree.forest[1].parent, Parent::Root));
        assert_eq!(tree.forest[1].right, Some(0));
        assert_eq!(tree.forest[1].left, None);
        assert!(matches!(tree.forest[0].parent, Parent::Node(1)));
        //  1
        //   \       <= link(0, 1)
        //    0

        assert!(tree.connected(0, 1)); // now connected
        assert!(matches!(tree.forest[1].parent, Parent::Root));
        assert_eq!(tree.forest[1].right, None);
        assert_eq!(tree.forest[1].left, None);
        assert!(matches!(tree.forest[0].parent, Parent::Path(1)));
        //    0             1
        //   /      =>      |
        //  1               0

        tree.cut(0);
        assert!(matches!(tree.forest[0].parent, Parent::Root));
        assert_eq!(tree.forest[0].right, None);
        //    0           0
        //   /     =>
        //  1           1
        assert!(!tree.connected(0, 1)); // now disconnected
    }

    #[test]
    pub fn cut_into_two_subtrees() {
        let mut tree = super::LinkCutTree::new(5);
        tree.forest[0].left = Some(1);
        tree.forest[1].parent = Parent::Node(0);
        tree.forest[1].left = Some(2);
        tree.forest[2].parent = Parent::Node(1);
        tree.forest[3].right = Some(4);
        tree.forest[4].parent = Parent::Node(3);
        // Given two trees:
        //       0       3
        //      /         \
        //     1           4
        //    /
        //   2
        tree.link(2, 3);
        // link(2, 3) should result in:
        //      3
        //      | \
        //      4  2
        //         |
        //         1
        //          \
        //           0
        assert!(matches!(tree.forest[3].parent, Parent::Root));
        assert!(matches!(tree.forest[4].parent, Parent::Path(3)));
        assert_eq!(tree.forest[3].right, Some(2));
        assert!(matches!(tree.forest[2].parent, Parent::Node(3)));
        assert!(matches!(tree.forest[1].parent, Parent::Path(2)));
        assert_eq!(tree.forest[1].right, Some(0));
        assert!(tree.connected(2, 3));

        // we cut node 2 from its parent 3:
        tree.cut(2);
        assert!(!tree.connected(2, 3));
        assert!(!tree.connected(2, 4));
    }

    #[test]
    pub fn connectivity() {
        // We form a link-cut tree from a rooted tree with the following structure:
        //     0
        //    / \
        //   1   6
        //  / \   \
        // 2   3   7
        //    / \   \
        //   4   5   8
        //          /
        //         9
        let mut lctree = super::LinkCutTree::new(10);
        lctree.link(1, 0);
        lctree.link(2, 1);
        lctree.link(3, 1);
        lctree.link(4, 3);
        lctree.link(5, 3);
        lctree.link(6, 0);
        lctree.link(7, 6);
        lctree.link(8, 7);
        lctree.link(9, 8);

        // checking connectivity:
        for i in 0..10 {
            for j in 0..10 {
                assert!(lctree.connected(i, j));
            }
        }

        // we cut node 6 from its parent 0:
        lctree.cut(6);

        // the forest should now look like this:
        //         0
        //        /
        //       1        6
        //      / \        \
        //     2   3        7
        //        / \        \
        //       4   5        8
        //                   /
        //                  9

        // we check connectivity again for the two trees:
        for i in 0..6 {
            for j in 0..6 {
                assert!(lctree.connected(i, j));
            }
        }
        for i in 6..10 {
            for j in 6..10 {
                assert!(lctree.connected(i, j));
            }
        }
        for i in 0..6 {
            for j in 6..10 {
                assert!(!lctree.connected(i, j));
            }
        }
    }

    #[test]
    pub fn findmax() {
        // We form a link-cut tree from a rooted tree with the following structure
        // (the numbers in parentheses are the weights of the nodes):
        //     0(4)
        //    /    \
        //   1(6)   6(1)
        //  /   \     \
        // 2(0)  3(7)  7(3)
        //      /   \     \
        //    4(2)  5(9)   8(7)
        //                  /
        //                9(5)
        let mut lctree = super::LinkCutTree::new(10);
        lctree.link(1, 0);
        lctree.link(2, 1);
        lctree.link(3, 1);
        lctree.link(4, 3);
        lctree.link(5, 3);
        lctree.link(6, 0);
        lctree.link(7, 6);
        lctree.link(8, 7);
        lctree.link(9, 8);
        lctree.forest[0].weight = 4.0;
        lctree.forest[1].weight = 6.0;
        lctree.forest[2].weight = 0.0;
        lctree.forest[3].weight = 7.0;
        lctree.forest[4].weight = 2.0;
        lctree.forest[5].weight = 9.0;
        lctree.forest[6].weight = 1.0;
        lctree.forest[7].weight = 3.0;
        lctree.forest[8].weight = 7.0;
        lctree.forest[9].weight = 5.0;
        let ground_truth = vec![0, 1, 1, 3, 3, 5, 0, 0, 8, 8];

        for _ in 0..1000 {
            let v = rand::random::<usize>() % 10;
            assert_eq!(lctree.findmax(v), ground_truth[v]);
        }
    }

    // creates a random tree with n nodes
    fn create_random_tree(n: usize) -> (Vec<(usize, usize)>, Vec<f64>, Vec<usize>) {
        let mut edges = Vec::new();
        let mut weights: Vec<f64> = (0..n).map(|i| i as f64).collect();
        weights.shuffle(&mut rand::thread_rng());

        let mut ground_truth = vec![0; n];
        let mut in_tree = Vec::from([0]);
        for i in 1..n {
            let parent_idx = rand::random::<usize>() % in_tree.len();
            let parent = in_tree[parent_idx];
            edges.push((i, parent));

            let max_idx = if weights[i] >= weights[ground_truth[parent]] {
                i
            } else {
                ground_truth[parent]
            };
            ground_truth[i] = max_idx;

            in_tree.push(i);
        }

        (edges, weights, ground_truth)
    }

    #[test]
    pub fn findmax_random() {
        let n = 7;
        let (edges, weights, ground_truth) = create_random_tree(n);
        let mut lctree = super::LinkCutTree::new(n);

        println!("weights: {:?}", weights);
        println!("edges: {:?}", edges);
        println!("ground truth: {:?}", ground_truth);

        for (v, w) in edges {
            lctree.link(v, w);
        }
        for i in 0..n {
            lctree.forest[i].weight = weights[i];
        }

        for _ in 0..1000 {
            let v = rand::random::<usize>() % n;
            println!("v = {}", v);
            println!("-----------------------------");
            for node in &lctree.forest {
                println!("{}", node.to_str());
            }

            let findmax_result = lctree.findmax(v);
            
            println!("-----------------------------");
            for node in &lctree.forest {
                println!("{}", node.to_str());
            }

            assert_eq!(findmax_result, ground_truth[v]);
        }
    }
}
