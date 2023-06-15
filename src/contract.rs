use archway_bindings::{ArchwayQuery, ArchwayResult};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, Share, CONFIG, SHARES};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:archway-reward-manager";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<ArchwayQuery>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ArchwayResult<ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Save the sender as the admin
    let config = Config {
        admin: info.sender.clone(),
        mutable: msg.mutable,
    };
    CONFIG.save(deps.storage, &config)?;

    check_share_percentages(&msg.shares)?;

    // Processing each share
    for share in msg.shares {
        // Validating the recipient address
        let recipient = deps.api.addr_validate(&share.recipient)?;

        // Saving the share
        SHARES.save(deps.storage, recipient, &share)?;
    }

    Ok(Response::new().add_attribute("admin", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<ArchwayQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ArchwayResult<ContractError> {
    match msg {
        ExecuteMsg::UpdateShares { shares } => execute_update_shares(deps, env, info, shares),
        ExecuteMsg::LockContract {} => unimplemented!(),
        ExecuteMsg::DistributeRewards {} => unimplemented!(),
        ExecuteMsg::DistributeNativeTokens {} => unimplemented!(),
    }
}

fn execute_update_shares(
    deps: DepsMut<ArchwayQuery>,
    _env: Env,
    info: MessageInfo,
    shares: Vec<Share>,
) -> ArchwayResult<ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Only mutable contracts can add a share
    if config.mutable == false {
        return Err(ContractError::ContractNotMutable {});
    }

    // Only the admin can add a share
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    check_share_percentages(&shares)?;

    // Processing each share
    for share in shares {
        // Validating the recipient address
        let recipient = deps.api.addr_validate(&share.recipient)?;

        // Saving the share
        SHARES.save(deps.storage, recipient, &share)?;
    }

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ArchwayQuery>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Share { recipient } => unimplemented!(),
        QueryMsg::Shares { start_after, limit } => unimplemented!(),
    }
}

// Used to validate that the total percentage does not exceed 100% and does not fall below 100%
fn check_share_percentages(shares: &Vec<Share>) -> Result<(), ContractError> {
    let total_percentage = shares
        .iter()
        .fold(Decimal::zero(), |acc, share| acc + share.percentage);

    if total_percentage > Decimal::one() {
        return Err(ContractError::PercentageLimitExceeded {});
    };
    if total_percentage < Decimal::one() {
        return Err(ContractError::PercentageLimitNotMet {});
    };

    Ok(())
}
