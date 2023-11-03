use lctree::LinkCutTree;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use rand_derive2::RandGen;
use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};


// These can be larger if you have time to spare (see tests/README.md)
const NUMBER_OF_NODES: usize = 100;
const NUMBER_OF_OPERATIONS: usize = 2000;

fn create_random_generator() -> StdRng {
    let seed = rand::thread_rng().gen();
    println!("Seed: {}", seed); // print seed so we can reproduce the test (if it fails).
    StdRng::seed_from_u64(seed)
}

struct BruteForce {
    weights: Vec<f64>,
    adj: Vec<HashSet<usize>>,
    component_ids: Vec<usize>,
}

#[test]
pub fn connectivity() {
    let mut rng = create_random_generator();

    // Generate distinct random weights:
    let mut weights = (0..NUMBER_OF_NODES).map(|i| i as f64).collect::<Vec<_>>();
    weights.shuffle(&mut rng);

    // Initialize link-cut tree, we start with a forest of single nodes
    // (edges are not added yet):
    let mut lctree = LinkCutTree::default();
    for w in 0..NUMBER_OF_NODES {
        lctree.make_tree(weights[w]);
    }

    // Initialize brute force data structure:
    let mut brute = BruteForce::new(weights.clone());

    // Time the operations:
    let mut count_operations = [0; 4];
    let mut lctree_time = [Duration::new(0, 0); 4];
    let mut brute_time = [Duration::new(0, 0); 4];

    // Perform random operations: link, cut, or connected:
    for _ in 0..NUMBER_OF_OPERATIONS {
        // Choose two random nodes to perform:
        // - link: link the two nodes if they are not connected
        // - cut: cut the edge between the two nodes if it exists
        // - connected: check if the two nodes are connected
        // - path: find the maximum weight in the path between the two nodes
        let v = rng.gen_range(0..NUMBER_OF_NODES);
        let w = rng.gen_range(0..NUMBER_OF_NODES);

        // Choose a random operation:
        let operation: Operation = rng.gen();
        match operation {
            Operation::Link => {
                let now = std::time::Instant::now();
                lctree.link(v, w);
                lctree_time[0] += now.elapsed();

                let now = std::time::Instant::now();
                brute.link(v, w);
                brute_time[0] += now.elapsed();
            }
            Operation::Cut => {
                let now = std::time::Instant::now();
                lctree.cut(v, w);
                lctree_time[1] += now.elapsed();

                let now = std::time::Instant::now();
                brute.cut(v, w);
                brute_time[1] += now.elapsed();
            }
            Operation::Connected => {
                let now = std::time::Instant::now();
                let actual = lctree.connected(v, w);
                lctree_time[2] += now.elapsed();

                let now = std::time::Instant::now();
                let expected = brute.connected(v, w);
                brute_time[2] += now.elapsed();

                assert_eq!(actual, expected);
            }
            Operation::Path => {
                let now = std::time::Instant::now();
                let actual = lctree.path(v, w).max_weight_idx;
                lctree_time[3] += now.elapsed();

                let now = std::time::Instant::now();
                let expected = brute.findmax(v, w);
                brute_time[3] += now.elapsed();

                assert_eq!(actual, expected);
            }
        }
        count_operations[operation as usize] += 1;
    }

    println!("Number of nodes:       {}", NUMBER_OF_NODES);
    println!("Number of operations:  {}", NUMBER_OF_OPERATIONS);
    println!("------------------------------------------");
    println!(
        "{:10}    {:10}    {:10}    {:10}",
        "Operation", "Count", "lctree", "brute"
    );
    let operations = ["link", "cut", "connected", "path"];
    for (i, operation) in operations.iter().enumerate() {
        let lctree_op_time = format!("{:?}", lctree_time[i]);
        let brute_op_time = format!("{:?}", brute_time[i]);
        let count = format!("{}", count_operations[i]);
        println!(
            "{:10}    {:10}    {:10}    {:10}",
            operation, count, lctree_op_time, brute_op_time
        );
    }
    println!("------------------------------------------");
    println!("lctree:    {:?}", lctree_time.iter().sum::<Duration>());
    println!("brute:     {:?}", brute_time.iter().sum::<Duration>());
}

#[derive(RandGen)]
enum Operation {
    Link,
    Cut,
    Connected,
    Path,
}

impl BruteForce {
    pub fn new(weights: Vec<f64>) -> Self {
        // We start with a forest of single nodes:
        let component_ids = (0..NUMBER_OF_NODES).collect::<Vec<usize>>();
        let adj = vec![HashSet::new(); NUMBER_OF_NODES];
        Self {
            weights,
            adj,
            component_ids,
        }
    }

    fn update_component_ids(&mut self, node_idx: usize, new_component_id: usize) {
        // Explore each component and assign new component id
        let mut visited = HashSet::new();
        let mut stack = vec![node_idx];
        while let Some(cur) = stack.pop() {
            if visited.contains(&cur) {
                continue;
            }
            visited.insert(cur);
            self.component_ids[cur] = new_component_id;
            for next in &self.adj[cur] {
                if !visited.contains(next) {
                    stack.push(*next);
                }
            }
        }
    }

    pub fn link(&mut self, v: usize, w: usize) {
        // We only add the edge if it connects two different trees,
        // (we don't want to create cycles):
        if self.component_ids[v] != self.component_ids[w] {
            let new_component_id = self.component_ids[v].min(self.component_ids[w]);
            if self.component_ids[v] == new_component_id {
                self.update_component_ids(w, new_component_id);
            } else {
                self.update_component_ids(v, new_component_id);
            }
            self.adj[v].insert(w);
            self.adj[w].insert(v);
        }
    }

    pub fn cut(&mut self, v: usize, w: usize) {
        // We only cut the edge if it exists:
        if !self.adj[v].contains(&w) {
            return;
        }
        // Remove the edge and update the component ids:
        self.adj[v].remove(&w);
        self.adj[w].remove(&v);
        self.update_component_ids(v, v);
        self.update_component_ids(w, w);
    }

    pub fn connected(&self, v: usize, w: usize) -> bool {
        self.component_ids[v] == self.component_ids[w]
    }

    pub fn findmax(&self, src: usize, dest: usize) -> usize {
        if self.component_ids[src] != self.component_ids[dest] {
            return usize::MAX;
        }
        // explore each component and compute aggregates in the path
        // until we reach the destination
        let mut max = HashMap::new();

        max.insert(src, src);
        let mut visited = HashSet::new();
        let mut stack = vec![(src, src)];
        while let Some((prev, cur)) = stack.pop() {
            visited.insert(cur);

            max.insert(cur, cur);
            let prev_max = max[&prev];
            if self.weights[prev_max] > self.weights[cur] {
                max.insert(cur, prev_max);
            }

            if cur == dest {
                return max[&dest];
            }
            for next in &self.adj[cur] {
                if !visited.contains(next) {
                    stack.push((cur, *next));
                }
            }
        }
        usize::MAX
    }
}
