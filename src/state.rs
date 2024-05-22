use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Storage, StdError, StdResult};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Account {
    pub address: Addr,
    pub balance: u128,
    pub password_hash: Option<Vec<u8>>,  // Optional field to store password hash
}

// Map to store accounts by address using AddrWrapper
pub const ACCOUNTS: Map<&Addr, Account> = Map::new("accounts");

// Function to store an account
pub fn store_account(storage: &mut dyn Storage, account: &Account) -> StdResult<()> {
    ACCOUNTS.save(storage, &account.address, account)
}

// Function to load an account by address
pub fn load_account(storage: &dyn Storage, address: &Addr) -> StdResult<Account> {
    ACCOUNTS.load(storage, address)
}

// Function to update an account's balance
pub fn update_balance(
    storage: &mut dyn Storage,
    address: &Addr,
    amount: u128,
) -> Result<(), StdError> {
    ACCOUNTS.update(storage, address, |account| -> Result<_, StdError> {
        let mut account = account?;
        account.balance += amount; // Add the amount to existing balance
        Ok(account)
    })
}

// Function to set a new password hash
pub fn set_password_hash(storage: &mut dyn Storage, address: &Addr, password_hash: Vec<u8>) -> StdResult<()> {
    ACCOUNTS.update(storage, address, |account| -> StdResult<_> {
        let mut account = account?;
        account.password_hash = Some(password_hash);
        Ok(account)
    })
}

// Function to validate a password
pub fn validate_password(storage: &dyn Storage, address: &Addr, password_attempt: &[u8]) -> StdResult<bool> {
    let account = load_account(storage, address)?;
    if let Some(stored_hash) = account.password_hash {
        Ok(stored_hash == password_attempt)
    } else {
        Ok(false) // No password set, consider invalid
    }
}

// Function to query an account's balance
pub fn query_balance(storage: &dyn Storage, address: &Addr) -> StdResult<u128> {
    let account = load_account(storage, address)?;
    Ok(account.balance)
}
