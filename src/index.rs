pub struct Index {
    time_id: usize,
    deleted_ids: Vec<usize>, // maybe use a set instead?
}

impl Index {
    pub fn new() -> Self {
        Self {
            time_id: 0,
            deleted_ids: Vec::new(),
        }
    }

    pub fn insert(&mut self) -> usize {
        if !self.deleted_ids.is_empty() {
            return self.deleted_ids.pop().unwrap();
        }
        self.time_id += 1;
        self.time_id - 1
    }

    pub fn delete(&mut self, id: usize) {
        assert!(id < self.time_id, "Invalid deletion");
        self.deleted_ids.push(id);
    }
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn test_indexing() {
        let mut index = super::Index::new();
        // make 3 insertions
        assert_eq!(index.insert(), 0);
        assert_eq!(index.insert(), 1);
        assert_eq!(index.insert(), 2);
        assert_eq!(index.time_id, 3);

        // delete 1
        index.delete(1);
        assert_eq!(index.time_id, 3);
        assert_eq!(index.deleted_ids, vec![1]);

        // next insertion should be 1
        assert_eq!(index.insert(), 1);
        assert_eq!(index.time_id, 3);
    }

    #[test]
    #[should_panic]
    pub fn test_invalid_deletion() {
        let mut index = super::Index::new();
        // make 3 insertions
        assert_eq!(index.insert(), 0);
        assert_eq!(index.insert(), 1);
        assert_eq!(index.insert(), 2);
        index.delete(4);
    }
}
