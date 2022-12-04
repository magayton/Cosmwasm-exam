use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub commission: Uint128,
    pub accepted_token: Coin,
}
pub const CONFIG: Item<Config> = Item::new("config");

#[cw_serde]
pub struct HighestBid {
    pub address: Addr,
    pub bid: Uint128,
}

// the u32 represent how many time the user bidded
pub const BIDDERS: Map<Addr, (Coin, u32)> = Map::new("bidders");
pub const HIGHEST_BID: Item<HighestBid> = Item::new("highets_bid");

pub const IS_BIDDING_CLOSED: Item<bool> = Item::new("is_bidding_close");
pub const BID_WINNER: Item<Addr> = Item::new("bid_winner");
