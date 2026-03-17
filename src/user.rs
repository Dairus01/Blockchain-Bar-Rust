pub use serde::{Deserialize, Serialize};
use crate::tx::Tx;

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct UserAccount {
    pub address: [u8;32],
    pub balance: u64,
    pub tx: Vec<Tx>,
}

impl UserAccount {
    pub fn from_name(name: &str, balance: u64) -> Self {
        let mut address = [0u8; 32];
        let bytes = name.as_bytes();
        let len = bytes.len().min(32);
        address[..len].copy_from_slice(&bytes[..len]);
        UserAccount { address, balance, tx: vec![] }
    }
}
