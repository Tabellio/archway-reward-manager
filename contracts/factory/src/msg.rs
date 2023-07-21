use cosmwasm_schema::{cw_serde, QueryResponses};
use pantheon_utils::Share;

#[cw_serde]
pub struct InstantiateMsg {
    pub splitter_code_id: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateSplitterCodeId {
        code_id: u64,
    },
    CreateSplitter {
        shares: Vec<Share>,
        mutable: bool,
        label: String,
        /* TODO: Add sender here */
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(u64)]
    SplitterCodeID {},
}
