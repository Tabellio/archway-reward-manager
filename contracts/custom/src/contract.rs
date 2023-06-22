use archway_bindings::{ArchwayMsg, ArchwayQuery, ArchwayResult};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

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
        ExecuteMsg::UpdateRewardMetadata {
            owner_address,
            rewards_address,
        } => execute_update_reward_metadata(deps, env, info, owner_address, rewards_address),
    }
}

// ... other execute methods

// TODO: Add this execute method
// This will update the owner and rewards address on this contract
fn execute_update_reward_metadata(
    _deps: DepsMut<ArchwayQuery>,
    _env: Env,
    _info: MessageInfo,
    owner_address: Option<String>,
    rewards_address: Option<String>,
) -> ArchwayResult<ContractError> {
    // ... write custom logic if needed

    let msg = ArchwayMsg::UpdateContractMetadata {
        owner_address,
        rewards_address,
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "update_rewards_address"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps<ArchwayQuery>, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}
