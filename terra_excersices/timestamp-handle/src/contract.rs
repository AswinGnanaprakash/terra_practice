#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, StdError};
use cw2::set_contract_version;
use chrono::Local;
use std::convert::TryFrom;
// use crate::error::ContractError;
use crate::msg::{ExecuteMsg, QueryMsg};
use crate::state::{TimeStamp};
use cw20_base::ContractError;
// use cw20_base::msg::ExecuteMsg::Mint as Mint;
// use cw20_base::msg::ExecuteMsg
// use cw20_base::msg::QueryMsg;
use cw20_base::msg::InstantiateMsg;
use cw20_base::contract::{query_balance, execute_mint, create_accounts, };
use cw20_base::state::{TOKEN_INFO, BALANCES, TokenInfo, MinterData };


// version info for migration info
const CONTRACT_NAME: &str = "crates.io:timestamp-handle";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // check valid token info
    msg.validate()?;
    // create initial accounts
    let total_supply = create_accounts(&mut deps, &msg.initial_balances)?;

    if let Some(limit) = msg.get_cap() {
        if total_supply > limit {
            return Err(StdError::generic_err("Initial supply greater than cap").into());
        }
    }

    let mint = match msg.mint {
        Some(m) => Some(MinterData {
            minter: deps.api.addr_validate(&m.minter)?,
            cap: m.cap,
        }),
        None => None,
    };

    // store token info
    let data = TokenInfo {
        name: msg.name,
        symbol: msg.symbol,
        decimals: msg.decimals,
        total_supply,
        mint,
    };

    TOKEN_INFO.save(deps.storage, &data)?;
    Ok(Response::default())

}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {

    match msg {
        ExecuteMsg::Mint {recipient, amount} => execute_mint_local(deps, _env, info, recipient, amount),
    }

}

pub fn execute_mint_local(deps: DepsMut,_env:Env, info:MessageInfo, recipient:String, amount:Uint128) -> Result<Response, ContractError>{

    let timenow = "27 Jun 2021 10AM".to_string();
    let rcpt_addr = deps.api.addr_validate(&recipient)?;
    TimeStamp.save(deps.storage, &rcpt_addr, &timenow );

    execute_mint_local(deps, _env, info, recipient, amount)

}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg ) -> StdResult<Binary> {
    match msg {
        QueryMsg::Balance {address} => to_binary(&query_balance(deps, address)?),
    }
}