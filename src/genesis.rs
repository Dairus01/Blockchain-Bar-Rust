use crate::user::UserAccount;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use serde_json;

#[derive(Deserialize, Serialize)]
pub struct Genesis {
    pub balances: HashMap<UserAccount, u64>,
}

pub fn load_genesis(path: &str)-> Result<Genesis,std::io::Error> {
    let gen_content = std::fs::read_to_string(path)?;
    let loaded_genesis: Genesis = serde_json::from_str(&gen_content)?;
    Ok(loaded_genesis)
}