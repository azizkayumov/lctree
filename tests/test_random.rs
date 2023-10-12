use lctree::LinkCutTree;
use rand::{
    rngs::StdRng,
    seq::{IteratorRandom, SliceRandom},
    Rng, SeedableRng,
};
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

fn findmax_brute_force(
    v: usize,
    w: usize,
    edges: &HashSet<(usize, usize)>,
    weights: &[f64],
) -> usize {
    // create adjacency list from edges
    let mut adj = vec![vec![]; NUMBER_OF_NODES];
    for (v, w) in edges {
        adj[*v].push(*w);
        adj[*w].push(*v);
    }

    let mut findmax = (0..NUMBER_OF_NODES).collect::<Vec<usize>>();
    let mut visited = vec![false; NUMBER_OF_NODES];
    let mut stack = vec![(v, v)];

    // dfs from v to w, keeping track of the maximum weight in the path
    while let Some((prev, cur)) = stack.pop() {
        if visited[cur] {
            continue;
        }
        visited[cur] = true;
        if weights[findmax[prev]] > weights[findmax[cur]] {
            findmax[cur] = findmax[prev]
        }

        if cur == w {
            break;
        }

        for &next in &adj[cur] {
            if !visited[next] {
                stack.push((cur, next));
            }
        }
    }
    findmax[w]
}

const NUMBER_OF_NODES: usize = 100;
const NUMBER_OF_OPERATIONS: usize = 2000;

#[derive(RandGen)]
enum Operation {
    Link,
    Cut,
    Connected,
    Findmax,
}

#[test]
pub fn connectivity() {
    let mut rng = create_random_generator();
    let mut edges = HashSet::new();

    // initialize link-cut tree, we start with a forest of single nodes
    // (edges are not added yet):
    let mut lctree = LinkCutTree::new(NUMBER_OF_NODES);
    let mut component_ids = (0..NUMBER_OF_NODES).collect::<Vec<usize>>();
    let mut weights = (0..NUMBER_OF_NODES).map(|i| i as f64).collect::<Vec<_>>();
    weights.shuffle(&mut rng);
    for i in 0..NUMBER_OF_NODES {
        lctree.set_weight(i, weights[i]);
    }

    // perform random operations: link, cut, or connected:
    for _ in 0..NUMBER_OF_OPERATIONS {
        let operation: Operation = rng.gen();
        match operation {
            Operation::Link => {
                // Choose two random nodes to link:
                let v = rng.gen_range(0..NUMBER_OF_NODES);
                let w = rng.gen_range(0..NUMBER_OF_NODES);

                lctree.link(v, w); // ignores cycles

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
                // Choose a random edge to cut:
                let (v, w) = edges.iter().choose(&mut rng).unwrap();
                lctree.cut(*v, *w);

                // Remove the edge and update the component ids:
                edges.remove(&(*v, *w));
                component_ids = connected_components(&edges);
            }
            Operation::Connected => {
                // Choose two random nodes to check if they are connected:
                let v = rng.gen_range(0..NUMBER_OF_NODES);
                let w = rng.gen_range(0..NUMBER_OF_NODES);
                assert_eq!(lctree.connected(v, w), component_ids[v] == component_ids[w]);
            }
            Operation::Findmax => {
                // Choose two random nodes from the same tree to find the node
                // with the maximum weight in the path between them:
                let v = rng.gen_range(0..NUMBER_OF_NODES);
                let w = (0..NUMBER_OF_NODES)
                    .filter(|&w| component_ids[w] == component_ids[v])
                    .choose(&mut rng)
                    .unwrap();
                let expected = findmax_brute_force(v, w, &edges, &weights);
                assert_eq!(lctree.findmax(v, w), expected);
            }
        }
    }
}
