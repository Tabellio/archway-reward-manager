use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::Map;

// Share consists of an address and a percentage
// Address is the address of the recipient
// Percentage is the percentage of the total amount to be sent to the recipient
#[cw_serde]
pub struct Share {
    recipient: String,
    percentage: Decimal,
}

pub const SHARES: Map<Addr, Share> = Map::new("shares");
