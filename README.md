[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/azizkayumov/lctree/ci.yml?style=plastic)](#)
[![crates.io](https://img.shields.io/crates/v/lctree)](https://crates.io/crates/lctree)
[![codecov](https://codecov.io/gh/azizkayumov/lctree/graph/badge.svg?token=RBW7UKFCS0)](https://codecov.io/gh/azizkayumov/lctree)

# lctree
Rust implementation of [Link-cut tree](https://dl.acm.org/doi/pdf/10.1145/800076.802464): self-balancing data structure to maintain a dynamic forest of (un)rooted trees under the following operations that take `O(logn)` amortized time:
* `link(v, w)`: creates an edge between nodes `v` and `w`.
* `cut(v, w)`: removes the edge between nodes `v` and `w`.
* `connected(v, w)`: returns `true` if nodes `v` and `w` are in the same tree.
* `path(v, w)`: performs calculations on a path between nodes `v` and `w`.

## Usage
This example shows how to link and cut edges:
```rust
use lctree::LinkCutTree;

fn main() {
    // We form a link-cut tree for the following forest:
    // (the numbers in parentheses are the weights of the nodes):
    //            a(9)
    //           /    \
    //         b(1)    e(2)
    //        /   \      \
    //      c(8)  d(10)   f(4)
    let mut lctree = LinkCutTree::default();
    let a = lctree.make_tree(9.);
    let b = lctree.make_tree(1.);
    let c = lctree.make_tree(8.);
    let d = lctree.make_tree(10.);
    let e = lctree.make_tree(2.);
    let f = lctree.make_tree(4.);

    lctree.link(b, a);
    lctree.link(c, b);
    lctree.link(d, b);
    lctree.link(e, a);
    lctree.link(f, e);

    // Checking connectivity:
    assert!(lctree.connected(c, f)); // connected

    // Path aggregation:
    // We find the node with max weight on the path between c to f,
    // where a has the maximum weight of 9.0:
    let heaviest_node = lctree.path(c, f);
    assert_eq!(heaviest_node.idx, a);
    assert_eq!(heaviest_node.weight, 9.0);

    // We cut node e from its parent a:
    lctree.cut(e, a);

    // The forest should now look like this:
    //            a(9)
    //           /    
    //         b(1)      e(2)
    //        /   \        \
    //      c(8)  d(10)    f(4)

    // We check connectivity again:
    assert!(!lctree.connected(c, f)); // not connected anymore
}
```
Advanced usage include operations on paths:
<details>
<summary>Common path aggregates</summary>

Various kinds of calculations can be performed on a path between two nodes, such as `findmax` or `findsum`:

```rust
use lctree::{LinkCutTree, FindMax, FindSum};

fn main() {
    // We form a link-cut tree from the following rooted tree
    // (the numbers in parentheses are the weights of the nodes):
    //           a(9)
    //           /  \
    //         b(1)  e(2)
    //        /   \    \
    //      c(8)  d(10)  f(4)

    // Use FindMax or FindSum, depending on your usage:
    let mut lctree: LinkCutTree<FindSum> = lctree::LinkCutTree::new();
    let a = lctree.make_tree(9.);
    let b = lctree.make_tree(1.);
    let c = lctree.make_tree(8.);
    let d = lctree.make_tree(10.);
    let e = lctree.make_tree(2.);
    let f = lctree.make_tree(4.);

    lctree.link(b, a);
    lctree.link(c, b);
    lctree.link(d, b);
    lctree.link(e, a);
    lctree.link(f, e);

    // We find the sum of the weights on the path between c to f,
    let result = lctree.path(c, f);
    assert_eq!(result.sum, 8. + 1. + 9. + 2. + 4.);
}
```
</details>

<details>
<summary>Custom path aggregate function</summary>
    
A custom path aggregate function can be defined by using the `Path` trait:
    
```rust
use lctree::{LinkCutTree, Path};

#[derive(Copy, Clone)]
pub struct FindXor {
    pub xor: u64,
}

impl Path for FindXor {
    fn default(weight: f64, _: usize) -> Self {
        FindXor {
            xor: weight as u64,
        }
    }

    fn aggregate(&mut self, other: Self) {
        self.xor ^= other.xor;
    }
}

fn main() {
    // We form a link-cut tree from the following rooted tree
    // (the numbers in parentheses are the weights of the nodes):
    //           a(9)
    //           /  \
    //         b(1)  e(2)
    //        /   \    \
    //      c(8)  d(10)  f(4)
    let mut lctree: LinkCutTree<FindXor> = LinkCutTree::new();
    let a = lctree.make_tree(9.);
    let b = lctree.make_tree(1.);
    let c = lctree.make_tree(8.);
    let d = lctree.make_tree(10.);
    let e = lctree.make_tree(2.);
    let f = lctree.make_tree(4.);

    lctree.link(b, a);
    lctree.link(c, b);
    lctree.link(d, b);
    lctree.link(e, a);
    lctree.link(f, e);

    // We find the xor of the weights on the path between c to f,
    let result = lctree.path(c, f);
    assert_eq!(result.xor, 8 ^ 1 ^ 9 ^ 2 ^ 4);
}
```

</details>

## Benchmark
The overall running time for performing a number of random operations (`link(v, w)`, `cut(v, w)`, `connected(v, w)` or `findmax(v, w)`) on forests of varying sizes (check benchmark details [here](https://github.com/azizkayumov/lctree/blob/main/benches/README.md)).

| # Nodes     | # Operations    | [lctree](https://github.com/azizkayumov/lctree/blob/main/src/lctree.rs)    | [brute-force](https://github.com/azizkayumov/lctree/blob/main/benches/benchmark.rs)  |
| :---        | :---            | :---          | :---            |
| 100         | 10K             | 4.8161 ms     | 18.013 ms       |
| 200         | 20K             | 11.091 ms     | 69.855 ms       |
| 500         | 50K             | 31.623 ms     | 429.53 ms       |
| 1000        | 100K            | 68.649 ms     | 1.8746 s        |
| 5000        | 500K            | 445.83 ms     | 46.854 s        |
| 10K         | 1M              | 964.64 ms     | 183.24 s        |

## Credits
This crate applies the core concepts and ideas presented in the following sources:
- "A data structure for dynamic trees" by D. Sleator and R. E. TarJan ([published](https://dl.acm.org/doi/10.1145/800076.802464) in STOC '81).
- Link-cut tree [source code](https://codeforces.com/contest/117/submission/860934) by the author D. Sleator.
- MIT's lecture on dynamic graphs: [lecture](https://www.youtube.com/watch?v=XZLN6NxEQWo), [notes](https://courses.csail.mit.edu/6.851/spring12/scribe/L19.pdf), and [source code](https://github.com/6851-2021/rust-link-cut-tree).
- Helpful blog posts on the concepts of [rooted trees](https://codeforces.com/blog/entry/80383), [rerooting](https://codeforces.com/blog/entry/75885) and [splay operations](https://www.youtube.com/watch?v=2eCKpEmkxIc).

## License
This project is licensed under the [Apache License, Version 2.0](LICENSE.md) - See the [LICENSE.md](https://github.com/azizkayumov/lctree/blob/main/LICENSE) file for details.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the [Apache-2.0
license][apache-license], shall be licensed as above, without any additional
terms or conditions.

[apache-license]: http://www.apache.org/licenses/LICENSE-2.0
