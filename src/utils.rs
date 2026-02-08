use crate::state::State;
use crate::tx::Tx;
use std::fmt::Error;

impl State{
    pub fn apply(&mut self, tx: &Tx) -> Result<(), Error>{
        if is_reward(tx){
            self.balances.entry(tx.to.clone()).and_modify(|v| *v += tx.value);
            return Ok(());
        }

        if self.balances[&tx.from] < tx.value {
            eprintln!("Insufficient balance");
        }
        
        self.balances.entry(tx.from.clone()).and_modify(|v| {*v -= tx.value});
        self.balances.entry(tx.to.clone()).and_modify(|v| {*v += tx.value});
        return Ok(());
    }
}

///## Checks if the tx is a reward type of tx
pub fn is_reward(tx: &Tx) -> bool {
    tx.data == "reward"
}