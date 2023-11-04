// #![allow(dead_code, clippy::module_name_repetitions)] // yes, I want to name my structs with the same name as the file
mod index;
mod lctree;
mod node;
mod path;
mod splay;
pub use crate::lctree::LinkCutTree;
pub use path::{FindMax, FindMin, FindSum, Path};
