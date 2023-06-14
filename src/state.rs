use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Decimal};
use cw_storage_plus::{Item, Map};

// Contract configuration
#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub mutable: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");

// Share consists of an address and a percentage
// Address is the address of the recipient
// Percentage is the percentage of the total amount to be sent to the recipient
#[cw_serde]
pub struct Share {
    pub recipient: String,
    pub percentage: Decimal,
}

pub const SHARES: Map<Addr, Share> = Map::new("shares");
