use crate::bundle::Bundle;
use serde_cbor;
use sled::{Db, IVec};
use std::fs::File;
use std::io::Write;

pub struct PersistentStorage {
    db: Db,
}

impl PersistentStorage {
    pub fn new(path: &str) -> Self {
        let db = sled::open(path).unwrap();
        Self { db }
    }

    pub fn store_bundle(&self, bundle: &Bundle) {
        let id = bundle.id.as_bytes();
        let payload = serde_cbor::to_vec(&bundle).unwrap();
        self.db.insert(id, payload).unwrap();
    }

    pub fn get_bundle(&self, id: &str) -> Option<Bundle> {
        self.db
            .get(id.as_bytes())
            .unwrap()
            .map(|ivec| serde_cbor::from_slice::<Bundle>(&ivec).unwrap())
    }

    pub fn save_bundle(&self, bundle: &Bundle, file_path: &str) {
        let mut file = File::create(file_path).expect("Unable to create file");
        file.write_all(&bundle.payload)
            .expect("Unable to write data");
    }
}
