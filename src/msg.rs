// use archway_bindings::Coins;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Decimal;

use crate::state::Share;

#[cw_serde]
pub struct InstantiateMsg {
    shares: Vec<Share>,
    // If true, the contract will not allow any more shares to be added
    mutable: bool,
}

#[cw_serde]
pub enum ExecuteMsg {
    // Add a share to the contract
    AddShare {
        recipient: String,
        percentage: Decimal,
    },
    // Update a share in the contract
    UpdateShare {
        recipient: String,
        percentage: Decimal,
    },
    // Remove a share from the contract
    RemoveShare {
        recipient: String,
    },
    // Distribute rewards to all shares
    DistributeRewards {},
    // Distribute native tokens to all shares
    DistributeNativeTokens {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // Returns all the shares
    #[returns(Vec<Share>)]
    Shares {},
    // Returns a single share for an address
    #[returns(Share)]
    Share { recipient: String },
    // // Returns the outstanding rewards in the contract
    // #[returns(OutstandingRewardsResponse)]
    // OutstandingRewards {},
}

// #[cw_serde]
// pub struct OutstandingRewardsResponse {
//     pub rewards_balance: Coins,
//     pub total_records: u64,
// }
