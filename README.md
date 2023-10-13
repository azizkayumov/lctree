[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/azizkayumov/lctree/ci.yml?style=plastic)](#)
[![crates.io](https://img.shields.io/crates/v/lctree)](https://crates.io/crates/lctree)

# lctree
Rust implementation of [Link-Cut-Tree](https://dl.acm.org/doi/10.1145/253262.253347](https://dl.acm.org/doi/pdf/10.1145/800076.802464)): self-balancing data structure to maintain a forest of rooted trees through dynamically linking and cutting edges.

## Example
This example shows how to link and cut edges:
```rust
use lctree::LinkCutTree;

fn main() {
    // We form a link-cut tree from the following rooted tree:
    //     0
    //    / \
    //   1   4
    //  / \   \
    // 2   3   5
    //        /
    //       6
    let mut lctree = lctree::LinkCutTree::new();
    for i in 0..7 {
        lctree.make_tree(i as f64);
    }
    lctree.link(1, 0);
    lctree.link(2, 1);
    lctree.link(3, 1);
    lctree.link(4, 0);
    lctree.link(5, 4);
    lctree.link(6, 5);

    // Checking connectivity:
    assert!(lctree.connected(2, 6)); // connected

    // We cut node 4 from its parent 0:
    lctree.cut(4, 0);

    // The forest should now look like this:
    //     0
    //    /   
    //   1     4
    //  / \     \
    // 2   3     5
    //          /
    //         6

    // We check connectivity again:
    assert!(!lctree.connected(2, 6)); // not connected anymore
}
```

## License

Copyright 2019-2024 Kayumov Abduaziz.

Licensed under [Apache License, Version 2.0][apache-license] (the "License");
you may not use this crate except in compliance with the License.

Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See [LICENSE](LICENSE) for
the specific language governing permissions and limitations under the License.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the [Apache-2.0
license][apache-license], shall be licensed as above, without any additional
terms or conditions.

[apache-license]: http://www.apache.org/licenses/LICENSE-2.0
