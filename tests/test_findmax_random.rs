use lctree::LinkCutTree;
use rand::{seq::SliceRandom, Rng, SeedableRng};

fn create_random_tree(n: usize, seed: u64) -> (Vec<(usize, usize)>, Vec<f64>, Vec<usize>) {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    let mut edges = Vec::new();
    let mut weights: Vec<f64> = (0..n).map(|i| i as f64).collect();
    weights.shuffle(&mut rng);

    let mut max_to_root = vec![0; n];
    let mut in_tree = Vec::from([0]);
    for i in 1..n {
        let parent_idx = rng.gen_range(0..in_tree.len());
        let parent = in_tree[parent_idx];
        edges.push((i, parent));

        max_to_root[i] = if weights[i] > weights[max_to_root[parent]] {
            i
        } else {
            max_to_root[parent]
        };
        in_tree.push(i);
    }

    (edges, weights, max_to_root)
}

#[test]
pub fn findmax_random() {
    let n = 100;
    let seed = rand::thread_rng().gen();
    let (edges, weights, max_to_root) = create_random_tree(n, seed);
    let mut lctree = LinkCutTree::new(n);
    for i in 0..n {
        lctree.set_weight(i, weights[i]);
    }

    for (v, w) in edges {
        lctree.link(v, w);
    }

    for _ in 0..n * 100 {
        let v = rand::thread_rng().gen_range(0..n);
        assert_eq!(lctree.findmax(v), max_to_root[v]);
    }
}
