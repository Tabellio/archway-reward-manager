// use archway_bindings::Coins;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Binary;

use pantheon_utils::Share;

use crate::state::Config;

#[cw_serde]
pub struct InstantiateMsg {
    pub shares: Vec<Share>,
    // If true, the contract will not allow any more shares to be added
    pub mutable: bool,
}

#[cw_serde]
pub enum ExecuteMsg {
    // Update shares in the contract
    UpdateShares {
        shares: Vec<Share>,
    },
    // Instantiate a new contract as admin
    // The new contract's owner and reward address will be this contract
    AddCustomContract {
        code_id: u64,
        msg: Binary,
    },
    // Updates the custom contract's owner and reward address
    UpdateCustomContractRewardMetadata {
        address: String,
        owner_address: Option<String>,
        rewards_address: Option<String>,
    },
    // Set the mutable flag on this contract
    LockContract {},
    // Withdraw rewards for the contract
    WithdrawRewards {},
    // Distribute native tokens to all shares
    DistributeNativeTokens {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // Returns the contract config
    #[returns(Config)]
    Config {},
    // Returns all the shares
    #[returns(Vec<Share>)]
    Shares {
        start_after: Option<String>,
        limit: Option<u8>,
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
