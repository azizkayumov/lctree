[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/azizkayumov/lctree/rust.yml?style=plastic)](#)
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
    // Build a forest consisting of 6 nodes with the following weights
    // (the numbers in parentheses are the weights of the nodes):
    let mut lctree: LinkCutTree<FindMax> = LinkCutTree::new();
    let a = lctree.make_tree(9.);
    let b = lctree.make_tree(1.);
    let c = lctree.make_tree(8.);
    let d = lctree.make_tree(10.);
    let e = lctree.make_tree(2.);
    let f = lctree.make_tree(4.);
    
    //  Link the nodes to form the following tree
    //            a(9)
    //           /    \
    //         b(1)    e(2)
    //        /   \      \
    //      c(8)  d(10)   f(4)
    lctree.link(b, a);
    lctree.link(c, b);
    lctree.link(d, b);
    lctree.link(e, a);
    lctree.link(f, e);

    // Checking connectivity:
    assert!(lctree.connected(c, f)); // connected

    // Find the node with the maximum weight on the path from c to f:
    let heaviest_node = lctree.path(c, f);
    assert_eq!(heaviest_node.idx, a);
    assert_eq!(heaviest_node.weight, 9.0);

    // Cut the edge between e and a:
    lctree.cut(e, a);

    // The forest should now look like this:
    //            a(9)
    //           /    
    //         b(1)      e(2)
    //        /   \        \
    //      c(8)  d(10)    f(4)

    // Now c and f should not be connected anymore:
    assert!(!lctree.connected(c, f)); // not connected anymore
}
```

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
