use serde::{Deserialize, Serialize};
use cosmwasm_schema::cw_serde;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize)]
pub struct State {
    pub message: String,
}

#[cw_serde]
pub struct Entry {
    pub id: u64,
    pub description: String,
    pub owner: String,
}

pub const STATE: Item<State> = Item::new("state");

pub const ENTRY_SEQ: Item<u64> = Item::new("entry_seq");
pub const LIST: Map<u64, Entry> = Map::new("list");