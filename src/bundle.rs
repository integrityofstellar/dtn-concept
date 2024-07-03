use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bundle {
    pub id: String,
    pub payload: Vec<u8>,
    pub timestamp: u64,
    pub destination: String,
}
