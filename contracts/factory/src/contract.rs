use archway_bindings::{ArchwayMsg, ArchwayQuery, ArchwayResult};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult,
    SubMsg, WasmMsg,
};
use cw2::set_contract_version;
use cw_utils::parse_reply_instantiate_data;

use pantheon_utils::Share;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG, SPLITTER_CODE_ID};

use pantheon_splitter::msg::InstantiateMsg as SplitterInstantiateMsg;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:pantheon-factory";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const SPLITTER_INSTANTIATE_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config { admin: info.sender };
    CONFIG.save(deps.storage, &config)?;

    SPLITTER_CODE_ID.save(deps.storage, &msg.splitter_code_id)?;

    Ok(Response::new().add_attribute("admin", config.admin))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateSplitterCodeID { code_id } => {
            execute_splitter_code_id(deps, info, code_id)
        }
        ExecuteMsg::CreateSplitter {
            shares,
            mutable,
            label,
        } => execute_create_splitter(deps, env, info, shares, mutable, label),
    }
}

fn execute_splitter_code_id(
    deps: DepsMut,
    info: MessageInfo,
    code_id: u64,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    SPLITTER_CODE_ID.save(deps.storage, &code_id)?;

    Ok(Response::new().add_attribute("action", "update_splitter_code_id"))
}

fn execute_create_splitter(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    shares: Vec<Share>,
    mutable: bool,
    label: String,
) -> Result<Response, ContractError> {
    let code_id = SPLITTER_CODE_ID.load(deps.storage)?;

    let msg = SubMsg::reply_on_success(
        WasmMsg::Instantiate {
            admin: Some(info.sender.to_string()),
            code_id,
            msg: to_binary(&SplitterInstantiateMsg { shares, mutable })?,
            funds: vec![],
            label,
        },
        SPLITTER_INSTANTIATE_REPLY_ID,
    );

    Ok(Response::new()
        .add_submessage(msg)
        .add_attribute("action", "create_splitter"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}
