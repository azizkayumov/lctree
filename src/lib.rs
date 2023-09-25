#![allow(dead_code, clippy::module_name_repetitions)] // yes, I want to name my structs with the same name as the file
mod node;
mod splay;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
