use cosmwasm_std::{Addr, Storage};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
// pub struct InstantiateMsg {
//     pub count: i32,
// }

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // Connecting wallet to smart contract
    InitWallet{address: String, password: Option<String>},
    SetPassword {current_password: Option<String>, new_password: String},
    SendTransaction {
        recipient: String,
        amount: u128,
        password: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // Query address
    GetBalance {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct AccountResponse {
    pub address: Addr,
    pub balance: u128,
}

