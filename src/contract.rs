use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::msg::{AccountResponse, ExecuteMsg, QueryMsg};
use crate::state::{Account, store_account, get_account, get_balance};

// 1. instantiate (not needed for this wallet)
// #[entry_point]
// pub fn instantiate(
//     deps: DepsMut,
//     _env: Env,
//     info: MessageInfo,
//     msg: InstantiateMsg,
// ) -> StdResult<Response> {
// }

// 2. execute
#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::InitWallet { address, password } => {
            execute_init_wallet(deps, info, address, password)
        },
        ExecuteMsg::SetPassword { .. } => todo!(), // Implement later
        ExecuteMsg::SendTransaction { .. } => todo!(), // Implement later
    }
}

pub fn execute_init_wallet(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
    password: Option<String>,
) -> StdResult<Response> {
    // Create a new account with 0 balance
    let account = Account {
        address: deps.api.addr_validate(&address)?,
        balance: 0,
        password_hash: None,
    };

    // Store the account in state
    store_account(deps.storage, &account)?;

    Ok(Response::new()
        .add_attribute("action", "init_wallet")
        .add_attribute("address", address))
}

// 3. query
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalance {} => to_binary(&get_balance(deps.storage, &info.sender)?), // Get balance of sender's address
    }
}
