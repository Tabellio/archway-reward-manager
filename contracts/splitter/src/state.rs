use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use pantheon_utils::Share;

// Contract configuration
#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub mutable: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");

pub const SHARES: Map<Addr, Share> = Map::new("shares");
