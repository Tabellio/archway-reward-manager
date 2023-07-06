use archway_bindings::{ArchwayQuery, ArchwayResult};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::COUNTER;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:custom";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<ArchwayQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> ArchwayResult<ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    COUNTER.save(deps.storage, &0)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<ArchwayQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ArchwayResult<ContractError> {
    match msg {
        // ... other execute messages
        ExecuteMsg::Increment {} => execute_increment(deps, env, info),
    }
}

fn execute_increment(
    deps: DepsMut<ArchwayQuery>,
    _env: Env,
    _info: MessageInfo,
) -> ArchwayResult<ContractError> {
    COUNTER.update(deps.storage, |item| -> StdResult<_> { Ok(item + 1) })?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps<ArchwayQuery>, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}
