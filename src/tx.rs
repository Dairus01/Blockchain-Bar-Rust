use serde::{Deserialize, Serialize};
// use serde_json;
use crate::user::UserAccount;

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct Tx {
    pub from: UserAccount,
    pub to: UserAccount,
    pub value: u64,
    pub data: String,
}