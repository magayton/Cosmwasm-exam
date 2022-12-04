use crate::error::BidError;
use crate::msg::{BidExecuteMsg, BidInstantiateMsg, BidMigrateMsg, BidQueryMsg};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

mod execute;
mod instantiate;
mod query;
use crate::contract::execute::_execute;
use crate::contract::instantiate::_instantiate;
use crate::contract::query::_query;

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: BidInstantiateMsg,
) -> Result<Response, BidError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    _instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: BidExecuteMsg,
) -> Result<Response, BidError> {
    _execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: BidQueryMsg) -> StdResult<Binary> {
    _query(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: BidMigrateMsg) -> Result<Response, BidError> {
    Ok(Response::default())
}
