use crate::{Tx, UserAccount};
use crate::{genesis::load_genesis};
use crate::error::MempoolError;
use std::collections::HashMap;
use std::env;
use std::fs::{self,File};
use std::io::{Write,BufRead, BufReader};

pub struct State {
    pub balances: HashMap<UserAccount, u64>,
    pub tx_mempool: Vec<Tx>,
    pub db_file: Option<File>,
}

///## This is to get current state of chain from db
pub fn new_state_from_disk() -> Result<State, std::io::Error> {
    // get current working directory
    let cwd = env::current_dir()?;
    println!("The current direcory is {:?}", &cwd);

    // get genesis.json file path and join to the current working directory
    let gen_file_path = cwd.join("database/logs/genesis.json");
    let gen_content = fs::read_to_string(&gen_file_path)?;
    let genesis = load_genesis(&gen_content)?;

    // get tx data base file path
    let tx_db_file_path = cwd.join("database/logs/tx.db");
    let tx_file = File::open(&tx_db_file_path)?;
    let tx_reader = BufReader::new(&tx_file);
    // let tx: Tx = serde_json::from_reader(&mut tx_reader)?;
    let db_file = fs::OpenOptions::new().append(true).create(true).open(&tx_db_file_path).ok();

    // Seed state balances from genesis (convert String keys to UserAccount)
    let mut initial_balances: HashMap<UserAccount, u64> = HashMap::new();
    for (name, balance) in genesis.balances {
        initial_balances.insert(UserAccount::from_name(&name, balance), balance);
    }

    // create empty mutable state within the function
    let mut state = State{
        balances: initial_balances,
        tx_mempool: vec![],
        db_file
    };

    for line in tx_reader.lines(){
        let line = line?;
        let tx: Tx = serde_json::from_str(&line)?;
        let _ = state.apply(&tx);
    }

    //Get new updated state from the genesis.
    return Ok(state);
}

///## Add new tx to tx_mempool
impl State{
    pub fn add(&mut self, tx: Tx) -> Result<(), MempoolError> {
        if !self.apply(&tx).is_ok(){
            return Err(MempoolError::ApplyTxUnsuccessful);
        }
        self.tx_mempool.push(tx);
        Ok(())
    }

    ///## Persist the tx to disk
    pub fn persist(&mut self) -> Result<(), MempoolError>{
        let mempool = self.tx_mempool.clone();
        // let mut i = 0;
        for i in mempool.iter(){
            let tx_json = serde_json::to_vec(i)
            .map_err(|_| MempoolError::ApplyTxUnsuccessful)?;

            self.db_file.as_ref()
            .expect("not able to unpack file option")
            .write_all(&tx_json).map_err(|_| MempoolError::WriteAllUnsuccessful)?;

            // THIS IS TO GIVE CONSISTENCY AS THIS WOULD WRITE A NEW LINE TO THE DISK
            self.db_file.as_ref()
            .expect("Could not add the new line")
            .write_all(b"\n").map_err(|_| MempoolError::WriteAllUnsuccessful)?; // New line
        }
        self.tx_mempool.clear();
        Ok(())
    }
}

