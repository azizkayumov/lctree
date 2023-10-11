use lctree::LinkCutTree;
use rand::{rngs::StdRng, seq::IteratorRandom, Rng, SeedableRng};
use rand_derive2::RandGen;
use std::collections::HashSet;

fn create_random_generator() -> StdRng {
    let seed = rand::thread_rng().gen();
    println!("Seed: {}", seed); // print seed so we can reproduce the test (if it fails).
    StdRng::seed_from_u64(seed)
}

fn dfs(
    v: usize,
    adj: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    component_ids: &mut Vec<usize>,
    component_id: usize,
) {
    visited[v] = true;
    component_ids[v] = component_id;
    for &w in &adj[v] {
        if !visited[w] {
            dfs(w, adj, visited, component_ids, component_id);
        }
    }
}

fn connected_components(edges: &HashSet<(usize, usize)>) -> Vec<usize> {
    // create adjacency list from edges
    let mut adj = vec![vec![]; NUMBER_OF_NODES];
    for (v, w) in edges {
        adj[*v].push(*w);
        adj[*w].push(*v);
    }

    // explore each component using dfs and assign component ids
    let mut visited = vec![false; NUMBER_OF_NODES];
    let mut component_ids = vec![0; NUMBER_OF_NODES];
    for v in 0..NUMBER_OF_NODES {
        if !visited[v] {
            dfs(v, &adj, &mut visited, &mut component_ids, v);
        }
    }
    component_ids
}

const NUMBER_OF_NODES: usize = 100;
const NUMBER_OF_OPERATIONS: usize = 2000;

#[derive(RandGen)]
enum Operation {
    Link,
    Cut,
    Connected,
}

#[test]
pub fn connectivity() {
    let now = std::time::Instant::now();
    let mut rng = create_random_generator();
    let mut edges = HashSet::new();

    // initialize link-cut tree, we start with a forest of single nodes
    // (edges are not added yet):
    let mut lctree = LinkCutTree::new(NUMBER_OF_NODES);
    let mut component_ids = (0..NUMBER_OF_NODES).collect::<Vec<usize>>();

    // perform random operations: link, cut, or connected:
    for _ in 0..NUMBER_OF_OPERATIONS {
        let operation: Operation = rng.gen();
        match operation {
            Operation::Link => {
                let v = rng.gen_range(0..NUMBER_OF_NODES);
                let w = rng.gen_range(0..NUMBER_OF_NODES);
                lctree.link(v, w); // ignores if v and w are already connected

                // We only add the edge if it connects two different trees,
                // we don't want to create cycles:
                if component_ids[v] != component_ids[w] {
                    edges.insert((v, w));
                    component_ids = connected_components(&edges);
                }
            }
            Operation::Cut => {
                if edges.is_empty() {
                    continue;
                }
                let (v, w) = edges.iter().choose(&mut rng).unwrap();
                lctree.cut(*v, *w);

                edges.remove(&(*v, *w));
                component_ids = connected_components(&edges);
            }
            Operation::Connected => {
                let v = rng.gen_range(0..NUMBER_OF_NODES);
                let w = rng.gen_range(0..NUMBER_OF_NODES);
                assert_eq!(lctree.connected(v, w), component_ids[v] == component_ids[w]);
            }
        }
    }
    println!("Time: {}ms", now.elapsed().as_millis()); // Time: 40ms
}
