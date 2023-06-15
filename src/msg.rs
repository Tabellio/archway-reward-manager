// use archway_bindings::Coins;
use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::Share;

#[cw_serde]
pub struct InstantiateMsg {
    pub shares: Vec<Share>,
    // If true, the contract will not allow any more shares to be added
    pub mutable: bool,
}

#[cw_serde]
pub enum ExecuteMsg {
    // Update shares in the contract
    UpdateShares { shares: Vec<Share> },
    // Set the mutable flag on the contract
    LockContract {},
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
    Shares {
        start_after: Option<String>,
        limit: Option<u32>,
    },
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
