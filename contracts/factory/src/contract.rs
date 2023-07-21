#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    instantiate2_address, to_binary, Binary, CodeInfoResponse, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult, WasmMsg,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, SPLITTER_CODE_ID};

use archway_bindings::{ArchwayMsg, ArchwayQuery, ArchwayResult};

use pantheon_splitter::msg::InstantiateMsg as SplitterInstantiateMsg;
use pantheon_utils::Share;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:pantheon-factory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<ArchwayQuery>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ArchwayResult<ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config { admin: info.sender };
    CONFIG.save(deps.storage, &config)?;

    SPLITTER_CODE_ID.save(deps.storage, &msg.splitter_code_id)?;

    Ok(Response::new().add_attribute("admin", config.admin))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<ArchwayQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ArchwayResult<ContractError> {
    match msg {
        ExecuteMsg::UpdateSplitterCodeID { code_id } => {
            execute_update_splitter_code_id(deps, info, code_id)
        }
        ExecuteMsg::CreateSplitter {
            shares,
            mutable,
            label,
        } => execute_create_splitter(deps, env, info, shares, mutable, label),
    }
}

fn execute_update_splitter_code_id(
    deps: DepsMut<ArchwayQuery>,
    info: MessageInfo,
    code_id: u64,
) -> ArchwayResult<ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    SPLITTER_CODE_ID.save(deps.storage, &code_id)?;

    Ok(Response::new().add_attribute("action", "update_splitter_code_id"))
}

fn execute_create_splitter(
    deps: DepsMut<ArchwayQuery>,
    env: Env,
    info: MessageInfo,
    shares: Vec<Share>,
    mutable: bool,
    label: String,
) -> ArchwayResult<ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let code_id = SPLITTER_CODE_ID.load(deps.storage)?;

    let msg = to_binary(&SplitterInstantiateMsg { shares, mutable })?;

    let creator = deps.api.addr_canonicalize(env.contract.address.as_str())?;
    let CodeInfoResponse { checksum, .. } = deps.querier.query_wasm_code_info(code_id)?;
    let salt = msg.clone();
    let address = deps
        .api
        .addr_humanize(&instantiate2_address(&checksum, &creator, &salt)?)?;

    Ok(Response::new()
        .add_message(WasmMsg::Instantiate2 {
            admin: Some(info.sender.to_string()),
            code_id,
            msg,
            funds: vec![],
            label,
            salt,
        })
        .add_message(ArchwayMsg::UpdateContractMetadata {
            contract_address: Some(address.to_string()),
            owner_address: Some(config.admin.to_string()),
            rewards_address: Some(config.admin.to_string()),
        })
        .add_message(WasmMsg::UpdateAdmin {
            contract_addr: address.to_string(),
            admin: info.sender.to_string(),
        }))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<ArchwayQuery>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::SplitterCodeID {} => to_binary(&query_splitter_code_id(deps, env)?),
    }
}

fn query_splitter_code_id(deps: Deps<ArchwayQuery>, _env: Env) -> StdResult<u64> {
    let code_id = SPLITTER_CODE_ID.load(deps.storage)?;
    Ok(code_id)
}
