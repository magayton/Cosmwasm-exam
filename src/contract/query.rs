use crate::msg::BidQueryMsg;
use crate::state::{HighestBid, BIDDERS, BID_WINNER, CONFIG, HIGHEST_BID, IS_BIDDING_CLOSED};
use cosmwasm_std::{to_binary, Addr, Binary, Deps, Env, StdResult};

pub fn _query(deps: Deps, _env: Env, msg: BidQueryMsg) -> StdResult<Binary> {
    match msg {
        BidQueryMsg::GetTotalBidAddr { address } => to_binary(&get_total_bid_addr(deps, address)?),
        BidQueryMsg::GetHighestBid {} => to_binary(&get_highest_bid(deps)?),
        BidQueryMsg::GetWinningBidder {} => to_binary(&get_winning_bider(deps)?),
        BidQueryMsg::GetAcceptedDenom {} => to_binary(&get_accepted_denom(deps)?),
        BidQueryMsg::IsBiddingClosed {} => to_binary(&is_bidding_closed(deps)?),
    }
}

pub fn get_total_bid_addr(deps: Deps, address_to_check: Addr) -> StdResult<u128> {
    let bidder = BIDDERS.load(deps.storage, address_to_check)?;
    let total_bid = bidder.0.amount;
    Ok(total_bid.u128())
}

pub fn get_highest_bid(deps: Deps) -> StdResult<HighestBid> {
    let highest_bidder = HIGHEST_BID.load(deps.storage).unwrap();

    Ok(HighestBid {
        address: highest_bidder.address,
        bid: highest_bidder.bid,
    })
}

pub fn get_winning_bider(deps: Deps) -> StdResult<Addr> {
    if IS_BIDDING_CLOSED.load(deps.storage).unwrap() {
        Ok(BID_WINNER.load(deps.storage)?)
    } else {
        Ok(Addr::unchecked("nowinneryet"))
    }
}

pub fn get_accepted_denom(deps: Deps) -> StdResult<String> {
    Ok(CONFIG.load(deps.storage)?.accepted_token.denom)
}

pub fn is_bidding_closed(deps: Deps) -> StdResult<bool> {
    Ok(IS_BIDDING_CLOSED.load(deps.storage).unwrap())
}
