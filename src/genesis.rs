use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize)]
pub struct Genesis {
    pub balances: HashMap<String, u64>,
}

pub fn load_genesis(content: &str) -> Result<Genesis, std::io::Error> {
    let loaded_genesis: Genesis = serde_json::from_str(content)?;
    Ok(loaded_genesis)
}