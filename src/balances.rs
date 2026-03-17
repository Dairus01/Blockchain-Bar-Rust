use crate::state::State;
use crate::user::UserAccount;

impl State {
    pub fn get_balance(&self, account: &UserAccount) -> u64 {
        *self.balances.get(account).unwrap_or(&0)
    }
}
