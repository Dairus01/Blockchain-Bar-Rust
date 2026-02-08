pub use serde::{Deserialize, Serialize};
use crate::tx::Tx;

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct UserAccount {
    address: [u8;32],
    balance: u64,
    tx: Vec<Tx>,
}
