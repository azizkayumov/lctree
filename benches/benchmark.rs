use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lctree::LinkCutTree;
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use rand_derive2::RandGen;
use std::collections::{HashMap, HashSet};

fn benchmark(criterion: &mut Criterion) {
    let num_nodes = [1000, 1000, 1000, 1000, 1000];
    let num_operations = [10_000, 50_000, 100_000, 500_000, 1_000_000];
    let seeds: [u64; 5] = [0, 1, 2, 3, 4];

    // The last two benchmarks may be slow with the brute force implementation:
    for i in 0..num_operations.len() {
        let mut group = criterion.benchmark_group(format!("forest_{}", num_operations[i]).as_str());
        group.sample_size(10);

        group.bench_function("lctree", |bencher| {
            bencher.iter(|| {
                lctree(
                    black_box(num_nodes[i]),
                    black_box(num_operations[i]),
                    black_box(seeds[i]),
                );
            });
        });

        group.bench_function("bruteforce", |bencher| {
            bencher.iter(|| {
                bruteforce(
                    black_box(num_nodes[i]),
                    black_box(num_operations[i]),
                    black_box(seeds[i]),
                );
            });
        });
    }
}

criterion_group!(benches, benchmark);
criterion_main!(benches);

#[derive(RandGen)]
enum Operation {
    Link,
    Cut,
    Connected,
    Path,
}

fn lctree(num_nodes: usize, num_operations: usize, seed: u64) {
    let mut rng = StdRng::seed_from_u64(seed);
    // Generate distinct random weights:
    let mut weights = (0..num_nodes).map(|i| i as f64).collect::<Vec<_>>();
    weights.shuffle(&mut rng);

    // Initialize link-cut tree, we start with a forest of single nodes
    // (edges are not added yet):
    let mut lctree = LinkCutTree::default();
    for w in 0..num_nodes {
        lctree.make_tree(weights[w]);
    }

    for _ in 0..num_operations {
        let v = rng.gen_range(0..num_nodes);
        let w = rng.gen_range(0..num_nodes);

        // Choose a random operation:
        let operation: Operation = rng.gen();
        match operation {
            Operation::Link => {
                lctree.link(v, w);
            }
            Operation::Cut => {
                lctree.cut(v, w);
            }
            Operation::Connected => {
                lctree.connected(v, w);
            }
            Operation::Path => {
                lctree.path(v, w);
            }
        }
    }
}

fn bruteforce(num_nodes: usize, num_operations: usize, seed: u64) {
    let mut rng = StdRng::seed_from_u64(seed);
    // Generate distinct random weights:
    let mut weights = (0..num_nodes).map(|i| i as f64).collect::<Vec<_>>();
    weights.shuffle(&mut rng);

    // Initialize link-cut tree, we start with a forest of single nodes
    // (edges are not added yet):
    let mut bruteforce = BruteForce::new(weights);

    for _ in 0..num_operations {
        let v = rng.gen_range(0..num_nodes);
        let w = rng.gen_range(0..num_nodes);

        // Choose a random operation:
        let operation: Operation = rng.gen();
        match operation {
            Operation::Link => {
                bruteforce.link(v, w);
            }
            Operation::Cut => {
                bruteforce.cut(v, w);
            }
            Operation::Connected => {
                bruteforce.connected(v, w);
            }
            Operation::Path => {
                bruteforce.path(v, w);
            }
        }
    }
}

struct BruteForce {
    weights: Vec<f64>,
    adj: Vec<HashSet<usize>>,
    component_ids: Vec<usize>,
}

impl BruteForce {
    pub fn new(weights: Vec<f64>) -> Self {
        // We start with a forest of single nodes:
        let component_ids = (0..weights.len()).collect::<Vec<usize>>();
        let adj = vec![HashSet::new(); weights.len()];
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

    pub fn path(&self, src: usize, dest: usize) -> usize {
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
