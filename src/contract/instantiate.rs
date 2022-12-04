use crate::error::BidError;
use crate::msg::BidInstantiateMsg;
use crate::state::{Config, HighestBid, CONFIG, HIGHEST_BID, IS_BIDDING_CLOSED};
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};

pub fn _instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: BidInstantiateMsg,
) -> Result<Response, BidError> {
    // Config init

    let mut owner = info.sender.clone();
    if let Some(owner_msg) = msg.owner {
        owner = deps.api.addr_validate(owner_msg.as_str())?;
    };

    let commission = msg.commission;
    let accepted_token = msg.accepted_token;

    CONFIG.save(
        deps.storage,
        &Config {
            owner,
            commission,
            accepted_token,
        },
    )?;

    // Highest Bid init
    let highest_bid = HighestBid {
        address: info.sender,
        bid: Uint128::from(0u128),
    };

    HIGHEST_BID.save(deps.storage, &highest_bid)?;

    // Bidding open at start
    IS_BIDDING_CLOSED.save(deps.storage, &false)?;

    Ok(Response::new().add_attribute("Instantiate", "Instantiate OK"))
}
