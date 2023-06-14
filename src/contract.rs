use archway_bindings::{ArchwayQuery, ArchwayResult};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Decimal, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, SHARES};

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

    // Total percentage of shares
    // Used to validate that the total percentage does not exceed 100% and does not fall below 100%
    let total_percentage = msg
        .shares
        .iter()
        .fold(Decimal::zero(), |acc, share| acc + share.percentage);

    if total_percentage > Decimal::one() {
        return Err(ContractError::PercentageLimitExceeded {});
    }
    if total_percentage < Decimal::one() {
        return Err(ContractError::PercentageLimitNotMet {});
    }

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
    _deps: DepsMut<ArchwayQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> ArchwayResult<ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps<ArchwayQuery>, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}
