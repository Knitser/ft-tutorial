//create a method called internal_deposit which takes an AccountId and a Balance and adds the amount to the accounts current supply of fungible tokens
use near_sdk::require;

use crate::*;

impl Contract {
    pub(crate) fn internal_deposit(&mut self, account_id: AccountId, amount: Balance) {
        let balance = self.accounts.get(&account_id).unwrap_or(0);
        if let Some(new_balance) = balance.checked_add(amount) {
            self.accounts.insert(account_id, &new_balance);
        } else {
            env::panic(b"Balance overflow");
        }
    }
}
