use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex, thread, time, vec};

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
/* ---------------- Key Expiry Information --------------------------- */
/* ------------------------------------------------------------------- */
#[derive(Debug)]
#[allow(dead_code)]
struct KeyExpiryInfo {
    data: HashMap<String, i64>,
}

// expiry
static mut EXPIRY_INFO: Lazy<Mutex<KeyExpiryInfo>> = Lazy::new(|| {
    Mutex::new(KeyExpiryInfo {
        data: HashMap::new(),
    })
});

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

    pub fn expire(&mut self, key: String, at_unix_time: i64) {
        thread::spawn(move || {
            let mut db = unsafe { DB.lock().unwrap() };
            let mut expiry_info_db = unsafe { EXPIRY_INFO.lock().unwrap() };

            if at_unix_time == -1 {
                db.data.remove(&key);
                expiry_info_db.data.remove(&key);
                return;
            }

            expiry_info_db.data.insert(key, at_unix_time);
        });
    }

    /*
     * if I directly use this code inside the `loop` of `delete_expired_keys`
     * method, the lock acquired on the database will never expire, making the
     * whole server freeze
     *
     * by extracting the code in a different function, we ensure that the
     * values are moved as soon as the function returns
     */
    fn expire_keys_helper() {
        let mut db = unsafe { DB.lock().unwrap() };
        let mut expiry_info_db = unsafe { EXPIRY_INFO.lock().unwrap() };
        let mut keys_to_delete: Vec<String> = vec![];

        for (key, value) in expiry_info_db.data.iter() {
            let unix_now = chrono::Utc::now().timestamp();
            if *value < unix_now {
                keys_to_delete.push(key.to_string());
            }
        }

        for key in keys_to_delete {
            db.data.remove(&key);
            expiry_info_db.data.remove(&key);
        }
    }

    pub fn delete_expired_keys(&mut self) {
        thread::spawn(move || loop {
            DatabaseOps::expire_keys_helper();

            let seconds = time::Duration::from_secs(1);
            thread::sleep(seconds);
        });
    }
}
