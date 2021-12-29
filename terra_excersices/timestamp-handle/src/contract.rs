#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128};
use cw2::set_contract_version;
use chrono::Local;
use std::convert::TryFrom;
use crate::error::ContractError;
use crate::msg::{  InstantiateMsg, QueryMsg, ExecuteMsg};
use crate::state::{TokenInfo, BALANCES, TOKEN_INFO, BalanceAndTime};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:timestamp-handle";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let data = TokenInfo {
        name: msg.name,
        symbol: msg.symbol,
    };

    TOKEN_INFO.save(deps.storage, &data)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Mint {recipient, amount} => execute_mint(deps, _env, info, recipient, amount),
        // ExecuteMsg::Reset { count } => try_reset(deps, info, count),
    }
}

pub fn execute_mint(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // save amount to recipient balance

    let date = Local::now();
    let date_time : String = date.format("%Y-%m-%d][%H:%M:%S").to_string();

    let rcpt_addr = deps.api.addr_validate(&recipient)?;
    let balanceandtime = BalanceAndTime {
        balance : amount,
        timestamp : date_time,
    };

    BALANCES.save(
        deps.storage,
        &rcpt_addr,
        &balanceandtime)?;

    let res = Response::new()
        .add_attribute("action", "mint")
        .add_attribute("to", recipient)
        .add_attribute("amount", amount);
    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance {address} => to_binary(&query_balance(deps, address)?),
    }
}

pub fn query_balance(deps: Deps, address: String) -> StdResult<BalanceAndTime> {
    let address = deps.api.addr_validate(&address)?;
    let balance = BALANCES
        .load(deps.storage, &address)?;

    Ok(BalanceAndTime { balance: balance.balance, timestamp: balance.timestamp })
}
