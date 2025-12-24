//! # Link-cut-tree.
//! A self-balancing data structure to maintain a dynamic forest of (un)rooted trees
//! under the following operations that take `O(logn)` amortized time:
//! - `link(v, w)`: creates an edge between nodes `v` and `w`.
//! - `cut(v, w)`: removes the edge between nodes `v` and `w`.
//! - `connected(v, w)`: returns `true` if nodes `v` and `w` are in the same tree.
//! - `path(v, w)`: performs calculations on a path between nodes `v` and `w`.
//!
//! This crate implements link-cut tree for unrooted trees, which means all of the above operations
//! can be performed on any two nodes in the forest.
//!
//! # Path operations
//! The most common path aggregates are supported: `FindMax`, `FindMin`, and `FindSum`.
//! A custom path aggregate function can be implemented by using the [Path] trait.
//!
//! # Tree creation and removal
//! Tree nodes are created and removed using the following operations:
//! - `make_tree()`: creates a new tree containing a single node.
//! - `remove_tree(v)`: removes the tree containing a single node `v` from the forest.
//! - `extend_forest(weights)`: useful for creating a forest of trees from a vector of weights.
//!
//! For further documentation, see the [`LinkCutTree`] struct.
mod index;
mod lctree;
mod node;
mod path;
mod splay;
pub use crate::lctree::LinkCutTree;
pub use path::{FindMax, FindSum, FindXor, Path};
