use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ExecuteMsg {
    UpdateRewardMetadata {
        owner_address: Option<String>,
        rewards_address: Option<String>,
    },
}
