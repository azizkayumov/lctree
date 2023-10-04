use lctree::LinkCutTree;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

fn create_random_generator() -> StdRng {
    let seed = rand::thread_rng().gen();
    println!("Seed: {}", seed); // print seed so we can reproduce the test (if it fails).
    StdRng::seed_from_u64(seed)
}

fn create_random_tree(rng: &mut StdRng) -> (Vec<(usize, usize)>, Vec<f64>, Vec<usize>) {
    let mut edges = Vec::new();
    let mut weights: Vec<f64> = (0..NUMBER_OF_NODES).map(|i| i as f64).collect();
    weights.shuffle(rng);

    let mut max_to_root = vec![0; NUMBER_OF_NODES];
    let mut in_tree = Vec::from([0]);
    for i in 1..NUMBER_OF_NODES {
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

const NUMBER_OF_NODES: usize = 1000;

#[test]
pub fn findmax_random() {
    let mut rng = create_random_generator();
    let (edges, weights, expected_findmax) = create_random_tree(&mut rng);

    // initialize link-cut tree
    let mut lctree = LinkCutTree::new(NUMBER_OF_NODES);
    for i in 0..NUMBER_OF_NODES {
        lctree.set_weight(i, weights[i]);
    }
    for (v, w) in edges {
        lctree.link(v, w);
    }

    // perform random findmax queries
    let mut nodes = (0..NUMBER_OF_NODES).collect::<Vec<usize>>();
    nodes.shuffle(&mut rng);
    for v in nodes {
        assert_eq!(lctree.findmax(v), expected_findmax[v]);
    }
}
