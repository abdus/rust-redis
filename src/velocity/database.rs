use std::collections::HashMap;

/*
 * redis primarily have these five data-types:
 * * String
 * * List
 * * Set
 * * Hash
 * * Sorted Set
 */

#[derive(Debug)]
pub enum DataTypes {
    String(String),
}

#[derive(Debug)]
pub struct Database {
    data: HashMap<String, DataTypes>,
}

impl Database {
    pub fn get_instance() -> Database {
        Database {
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: DataTypes) {
        self.data.insert(key, value);
        println!("Database: {:?}", self.data);
    }

    pub fn get(&self, key: String) -> Option<&DataTypes> {
        let key = &key[..];
        self.data.get(key)
    }
}
