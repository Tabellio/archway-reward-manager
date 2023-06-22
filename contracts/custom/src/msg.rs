use cosmwasm_schema::{cw_serde, QueryResponses};

use archway_reward_manager_utils::ExecuteMsg as ArchwayRewardManagerUtils;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    // ... other execute messages
    // Simple counter increment message
    Increment {},
    // TODO: Add this new execute message
    // This will update the owner and rewards address on this contract
    UpdateRewardMetadata {
        owner_address: Option<String>,
        rewards_address: Option<String>,
    },
}

// TODO: Add this implementation
// This will convert the execute messages to the ArchwayRewardManagerUtils messages
impl From<ExecuteMsg> for ArchwayRewardManagerUtils {
    fn from(msg: ExecuteMsg) -> Self {
        match msg {
            ExecuteMsg::UpdateRewardMetadata {
                owner_address,
                rewards_address,
            } => ArchwayRewardManagerUtils::UpdateRewardMetadata {
                owner_address,
                rewards_address,
            },
            _ => unreachable!("Cannot convert {:?} to ArchwayRewardManagerUtils", msg),
        }
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
