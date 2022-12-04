use crate::state::HighestBid;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Uint128};

#[cw_serde]
pub struct BidInstantiateMsg {
    pub owner: Option<String>,
    pub commission: Uint128,
    pub accepted_token: Coin,
}

#[cw_serde]
pub enum BidExecuteMsg {
    Bid {},
    Close {},
    Retract { receiver: Option<Addr> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum BidQueryMsg {
    #[returns(u128)]
    GetTotalBidAddr { address: Addr },

    #[returns(HighestBid)]
    GetHighestBid {},

    #[returns(Addr)]
    GetWinningBidder {},

    #[returns(String)]
    GetAcceptedDenom {},

    #[returns(bool)]
    IsBiddingClosed {},
}

#[cw_serde]
pub struct BidMigrateMsg {}
