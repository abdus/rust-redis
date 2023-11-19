use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex, vec};

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

impl Clone for DataTypes {
    fn clone(&self) -> Self {
        match self {
            DataTypes::String(s) => DataTypes::String(s.clone()),
        }
    }
}

/* ------------------------------------------------------------------- */
/* ------------------------ DATABASE --------------------------------- */
/* ------------------------------------------------------------------- */

#[derive(Debug)]
struct Database {
    data: HashMap<String, DataTypes>,
}

unsafe impl Sync for Database {}

// persistant db variable through out the program
static mut DB: Lazy<Mutex<Database>> = Lazy::new(|| {
    Mutex::new(Database {
        data: HashMap::new(),
    })
});

#[derive(Debug)]
pub struct DatabaseOps;

impl DatabaseOps {
    pub fn set(&mut self, key: String, value: DataTypes) {
        let mut db = unsafe { DB.lock().unwrap() };

        db.data.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<DataTypes> {
        let db = unsafe { DB.lock().unwrap() };

        let key = &key[..];
        let data = db.data.get(key);

        let owned_data = data.clone();

        match owned_data {
            Some(data) => Some(data.clone()),
            None => None,
        }
    }

    pub fn keys(&self) -> Vec<String> {
        let db = unsafe { DB.lock().unwrap() };
        let keys = db.data.keys();
        let mut keys_list: Vec<String> = vec![];

        for key in keys {
            keys_list.push(key.to_string());
        }

        keys_list
    }

    pub fn del(&self, key: String) -> Option<DataTypes> {
        let mut db = unsafe { DB.lock().unwrap() };
        let key = &key[..];

        let removed_key = db.data.remove(key);

        removed_key
    }
}
