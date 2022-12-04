use bidding_contract::msg::{BidExecuteMsg, BidInstantiateMsg, BidMigrateMsg, BidQueryMsg};
use cosmwasm_schema::write_api;

fn main() {
    write_api! {
        instantiate: BidInstantiateMsg,
        execute: BidExecuteMsg,
        query: BidQueryMsg,
        migrate: BidMigrateMsg,
    }
}
