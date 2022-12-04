use crate::error::BidError;
use crate::msg::BidExecuteMsg;
use crate::state::{HighestBid, BIDDERS, BID_WINNER, CONFIG, HIGHEST_BID, IS_BIDDING_CLOSED};
use cosmwasm_std::{coin, Addr, BankMsg, Coin, DepsMut, Env, MessageInfo, Response, Uint128};

pub fn _execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: BidExecuteMsg,
) -> Result<Response, BidError> {
    match msg {
        BidExecuteMsg::Bid {} => bid(deps, info),
        BidExecuteMsg::Close {} => close(deps, info),
        BidExecuteMsg::Retract { receiver } => retract(deps, info, receiver),
    }
}

pub fn bid(deps: DepsMut, info: MessageInfo) -> Result<Response, BidError> {
    // Can not bid if bidding close
    if IS_BIDDING_CLOSED.load(deps.storage)? {
        return Err(BidError::BiddingClosed {});
    }

    //Can not bid if owner
    if info.sender == CONFIG.load(deps.storage)?.owner {
        return Err(BidError::OwnerCantBid {});
    }

    let accepted_denom = CONFIG.load(deps.storage)?.accepted_token.denom;

    // If fund do not have valid denom, error
    if !info.funds.iter().any(|coin| coin.denom == accepted_denom) {
        return Err(BidError::WrongToken {});
    }

    // Prepare commission
    let config = CONFIG.load(deps.storage)?;

    let current_bid = info
        .funds
        .iter()
        .find(|c| c.denom == accepted_denom)
        .map(|m| m.amount)
        .unwrap_or_else(|| Uint128::from(0u128));

    if current_bid < config.commission {
        return Err(BidError::BidUnderCommission {});
    }

    let commission_msg = BankMsg::Send {
        to_address: CONFIG.load(deps.storage)?.owner.to_string(),
        amount: vec![coin(
            u128::from(config.commission),
            config.accepted_token.denom,
        )],
    };

    // if total bid of user < Max bid, fail

    let mut total_current_bid = current_bid;
    let opt_sender_bid = BIDDERS.may_load(deps.storage, info.sender.clone())?;
    if let Some(ref sender_bid) = opt_sender_bid {
        total_current_bid = current_bid + sender_bid.0.amount;
    };

    let highest_bid = HIGHEST_BID.load(deps.storage)?;

    if highest_bid.bid >= total_current_bid {
        return Err(BidError::BidTooLow {});
    } else {
        // It is a new highest bid. We need to update

        HIGHEST_BID.save(
            deps.storage,
            &HighestBid {
                address: info.sender.clone(),
                bid: total_current_bid,
            },
        )?;

        // New bidder
        if opt_sender_bid.is_none() {
            BIDDERS.save(
                deps.storage,
                info.sender,
                &(coin(u128::from(current_bid), accepted_denom), 1),
            )?;
        } else {
            // Ancient bidder, we need to update the amount
            BIDDERS.update(
                deps.storage,
                info.sender,
                |x: Option<(Coin, u32)>| -> Result<(Coin, u32), BidError> {
                    let mut last_bid: (Coin, u32) = x.unwrap();
                    last_bid.0.amount += current_bid;
                    last_bid.1 += 1;
                    Ok(last_bid)
                },
            )?;
        }
    }

    Ok(Response::new()
        .add_attribute("Execute bid", "OK")
        .add_message(commission_msg))
}

pub fn close(deps: DepsMut, info: MessageInfo) -> Result<Response, BidError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(BidError::Unauthorized {});
    }

    let highest_bid = HIGHEST_BID.load(deps.storage)?;
    BID_WINNER.save(deps.storage, &highest_bid.address)?;

    IS_BIDDING_CLOSED.save(deps.storage, &true)?;

    if highest_bid.bid > Uint128::zero() {
        let nb_bid = Uint128::from(
            BIDDERS
                .load(deps.storage, highest_bid.address.clone())
                .unwrap()
                .1,
        );
        let amount_to_send = highest_bid.bid - nb_bid * config.commission;

        let msg_send_highest_bid_to_owner = BankMsg::Send {
            to_address: config.owner.to_string(),
            amount: vec![coin(
                u128::from(amount_to_send),
                config.accepted_token.denom,
            )],
        };

        return Ok(Response::new()
            .add_attribute("Execute close with funds to owner", "OK")
            .add_attribute("Bid winner", highest_bid.address.to_string())
            .add_message(msg_send_highest_bid_to_owner));
    }

    Ok(Response::new().add_attribute("Execute close without funds to owner", "OK"))
}

pub fn retract(
    deps: DepsMut,
    info: MessageInfo,
    receiver: Option<Addr>,
) -> Result<Response, BidError> {
    if !IS_BIDDING_CLOSED.load(deps.storage)? {
        return Err(BidError::BiddingNotClose {});
    }

    let opt_bidder_coin = BIDDERS.may_load(deps.storage, info.sender.clone())?;
    if opt_bidder_coin.is_none() {
        return Err(BidError::NothingToRetract {});
    }

    let highest_bidder = HIGHEST_BID.load(deps.storage)?.address;
    if highest_bidder == info.sender {
        return Err(BidError::WinnerCantRetract {});
    }

    let config = CONFIG.load(deps.storage).unwrap();

    let bid = opt_bidder_coin.unwrap();
    let token_to_send = bid.0.amount - Uint128::from(bid.1) * config.commission;

    let mut token_receiver = info.sender.clone();
    if let Some(new_token_receiver) = receiver {
        token_receiver = new_token_receiver;
    }

    let msg_send_retract = BankMsg::Send {
        to_address: token_receiver.to_string(),
        amount: vec![coin(u128::from(token_to_send), config.accepted_token.denom)],
    };

    BIDDERS.remove(deps.storage, info.sender.clone());

    Ok(Response::new()
        .add_attribute("Execute retract", "OK")
        .add_attribute("Address calling", info.sender.to_string())
        .add_message(msg_send_retract))
}
