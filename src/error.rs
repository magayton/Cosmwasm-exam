use cosmwasm_std::{DecimalRangeExceeded, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum BidError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Decimal(#[from] DecimalRangeExceeded),

    #[error("Bidding is closed")]
    BiddingClosed {},

    #[error("Token not accepted")]
    WrongToken {},

    #[error("Owner can not bid")]
    OwnerCantBid {},

    #[error("Bid is under commission")]
    BidUnderCommission {},

    #[error("Bid is not the new highest bid")]
    BidTooLow {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Nothing to retract")]
    NothingToRetract {},

    #[error("Bidding not close")]
    BiddingNotClose {},

    #[error("Winner can't retract")]
    WinnerCantRetract {},
}
