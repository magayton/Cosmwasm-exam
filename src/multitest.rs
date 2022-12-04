#![cfg(test)]

use crate::contract::{execute, instantiate, query};
use crate::error::BidError;
use crate::msg::{BidExecuteMsg, BidInstantiateMsg, BidQueryMsg};
use crate::state::HighestBid;
use cosmwasm_std::{coin, coins, Addr, Empty, Uint128};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};

fn bidding_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(execute, instantiate, query);
    Box::new(contract)
}

#[test]
fn test_execute_full_workflow() {
    // Scenario : 3 bidders
    // First we test every errors
    // Then every bider become the highest bider
    // bider1 become the highest again, and bider3 finally win

    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked("bidder1"), coins(35, "atom"))
            .unwrap();
        router
            .bank
            .init_balance(storage, &Addr::unchecked("bidder2"), coins(20, "atom"))
            .unwrap();
        router
            .bank
            .init_balance(storage, &Addr::unchecked("bidder3"), coins(50, "atom"))
            .unwrap();
        router
            .bank
            .init_balance(storage, &Addr::unchecked("owner"), coins(5, "atom"))
            .unwrap();
        router
            .bank
            .init_balance(storage, &Addr::unchecked("baddenom"), coins(5, "notatom"))
            .unwrap();
        router
            .bank
            .init_balance(storage, &Addr::unchecked("bidderpoor"), coins(1, "atom"))
            .unwrap();
    });

    let contract_id = app.store_code(bidding_contract());

    let contract_addr = app
        .instantiate_contract(
            contract_id,
            Addr::unchecked("instantiator"),
            &BidInstantiateMsg {
                owner: Some("owner".to_string()),
                commission: Uint128::from(2u64),
                accepted_token: coin(0, "atom"),
            },
            &[],
            "Bidding Contract",
            None,
        )
        .unwrap();

    let err = app
        .execute_contract(
            Addr::unchecked("owner"),
            contract_addr.clone(),
            &BidExecuteMsg::Bid {},
            &coins(5, "atom"),
        )
        .unwrap_err();

    assert_eq!(BidError::OwnerCantBid {}, err.downcast().unwrap());

    let err = app
        .execute_contract(
            Addr::unchecked("baddenom"),
            contract_addr.clone(),
            &BidExecuteMsg::Bid {},
            &coins(5, "notatom"),
        )
        .unwrap_err();

    assert_eq!(BidError::WrongToken {}, err.downcast().unwrap());

    let err = app
        .execute_contract(
            Addr::unchecked("bidderpoor"),
            contract_addr.clone(),
            &BidExecuteMsg::Bid {},
            &coins(1, "atom"),
        )
        .unwrap_err();

    assert_eq!(BidError::BidUnderCommission {}, err.downcast().unwrap());

    // First wave of errors done
    // Remain : close and bid too low

    // First Bid + verify with query
    app.execute_contract(
        Addr::unchecked("bidder1"),
        contract_addr.clone(),
        &BidExecuteMsg::Bid {},
        &coins(10, "atom"),
    )
    .unwrap();

    let resp: u128 = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &BidQueryMsg::GetTotalBidAddr {
                address: Addr::unchecked("bidder1"),
            },
        )
        .unwrap();

    assert_eq!(resp, 10u128);

    let resp: HighestBid = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &BidQueryMsg::GetHighestBid {})
        .unwrap();

    assert_eq!(
        resp,
        HighestBid {
            address: Addr::unchecked("bidder1"),
            bid: Uint128::from(10u128)
        }
    );

    // Error bid too low

    let resp = app
        .execute_contract(
            Addr::unchecked("bidder2"),
            contract_addr.clone(),
            &BidExecuteMsg::Bid {},
            &coins(10, "atom"),
        )
        .unwrap_err();

    assert_eq!(BidError::BidTooLow {}, resp.downcast().unwrap());

    // 2nd and 3rd Bid : verify state every time

    app.execute_contract(
        Addr::unchecked("bidder2"),
        contract_addr.clone(),
        &BidExecuteMsg::Bid {},
        &coins(20, "atom"),
    )
    .unwrap();

    let resp: u128 = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &BidQueryMsg::GetTotalBidAddr {
                address: Addr::unchecked("bidder2"),
            },
        )
        .unwrap();

    assert_eq!(resp, 20u128);

    let resp: HighestBid = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &BidQueryMsg::GetHighestBid {})
        .unwrap();

    assert_eq!(
        resp,
        HighestBid {
            address: Addr::unchecked("bidder2"),
            bid: Uint128::from(20u128)
        }
    );

    app.execute_contract(
        Addr::unchecked("bidder3"),
        contract_addr.clone(),
        &BidExecuteMsg::Bid {},
        &coins(25, "atom"),
    )
    .unwrap();

    let resp: u128 = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &BidQueryMsg::GetTotalBidAddr {
                address: Addr::unchecked("bidder3"),
            },
        )
        .unwrap();

    assert_eq!(resp, 25u128);

    let resp: HighestBid = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &BidQueryMsg::GetHighestBid {})
        .unwrap();

    assert_eq!(
        resp,
        HighestBid {
            address: Addr::unchecked("bidder3"),
            bid: Uint128::from(25u128)
        }
    );

    //4th bid : Use already token bided to calculate full bid

    app.execute_contract(
        Addr::unchecked("bidder1"),
        contract_addr.clone(),
        &BidExecuteMsg::Bid {},
        &coins(20, "atom"),
    )
    .unwrap();

    let resp: u128 = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &BidQueryMsg::GetTotalBidAddr {
                address: Addr::unchecked("bidder1"),
            },
        )
        .unwrap();

    assert_eq!(resp, 30u128);

    let resp: HighestBid = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &BidQueryMsg::GetHighestBid {})
        .unwrap();

    assert_eq!(
        resp,
        HighestBid {
            address: Addr::unchecked("bidder1"),
            bid: Uint128::from(30u128)
        }
    );

    //5th bid : highest bid

    app.execute_contract(
        Addr::unchecked("bidder3"),
        contract_addr.clone(),
        &BidExecuteMsg::Bid {},
        &coins(20, "atom"),
    )
    .unwrap();

    let resp: u128 = app
        .wrap()
        .query_wasm_smart(
            contract_addr.clone(),
            &BidQueryMsg::GetTotalBidAddr {
                address: Addr::unchecked("bidder3"),
            },
        )
        .unwrap();

    assert_eq!(resp, 45u128);

    let resp: HighestBid = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &BidQueryMsg::GetHighestBid {})
        .unwrap();

    assert_eq!(
        resp,
        HighestBid {
            address: Addr::unchecked("bidder3"),
            bid: Uint128::from(45u128)
        }
    );

    // Verify owner balance before close
    // 5 bids : 15 atoms in balance (10 commission + 5 from init balance)

    assert_eq!(
        app.wrap()
            .query_all_balances(Addr::unchecked("owner"))
            .unwrap(),
        coins(15, "atom")
    );

    // Close

    app.execute_contract(
        Addr::unchecked("owner"),
        contract_addr.clone(),
        &BidExecuteMsg::Close {},
        &[],
    )
    .unwrap();

    let resp: bool = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &BidQueryMsg::IsBiddingClosed {})
        .unwrap();
    assert!(resp);

    // error : bid close

    let err = app
        .execute_contract(
            Addr::unchecked("bidder1"),
            contract_addr.clone(),
            &BidExecuteMsg::Bid {},
            &coins(5, "atom"),
        )
        .unwrap_err();

    assert_eq!(BidError::BiddingClosed {}, err.downcast().unwrap());

    // Verify owner balance with commission transfered + close
    // Owner has 15 atom from commission + init balance
    // highest bid is 45 - 4 commission already in balance, so 15 + 41 = 56

    let resp: HighestBid = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &BidQueryMsg::GetHighestBid {})
        .unwrap();

    assert_eq!(
        resp,
        HighestBid {
            address: Addr::unchecked("bidder3"),
            bid: Uint128::from(45u128)
        }
    );

    assert_eq!(
        app.wrap()
            .query_all_balances(Addr::unchecked("owner"))
            .unwrap(),
        coins(56, "atom")
    );

    // Verify bidders balance before retract

    assert_eq!(
        app.wrap()
            .query_all_balances(Addr::unchecked("bidder1"))
            .unwrap(),
        coins(5, "atom")
    );

    assert_eq!(
        app.wrap()
            .query_all_balances(Addr::unchecked("bidder2"))
            .unwrap(),
        []
    );

    assert_eq!(
        app.wrap()
            .query_all_balances(Addr::unchecked("bidder3"))
            .unwrap(),
        coins(5, "atom")
    );

    //RETRACT
    // bidder1 had 35 atoms and bidded 2 times with 30 atoms, he will retract 26 (2*2 commission)
    // 26+5 = 31

    app.execute_contract(
        Addr::unchecked("bidder1"),
        contract_addr.clone(),
        &BidExecuteMsg::Retract { receiver: None },
        &[],
    )
    .unwrap();

    assert_eq!(
        app.wrap()
            .query_all_balances(Addr::unchecked("bidder1"))
            .unwrap(),
        coins(31, "atom")
    );

    //bidder2 retract to bidderpoor address
    //bidder2 sent all his funds to the bid contract, bidding 1 time 20 atom
    //bidderpoor add 1 atom, and will get 20 - 2x1 (commission) = 1 + 18 atoms

    app.execute_contract(
        Addr::unchecked("bidder2"),
        contract_addr.clone(),
        &BidExecuteMsg::Retract {
            receiver: Some(Addr::unchecked("bidderpoor")),
        },
        &[],
    )
    .unwrap();

    assert_eq!(
        app.wrap()
            .query_all_balances(Addr::unchecked("bidder2"))
            .unwrap(),
        []
    );

    assert_eq!(
        app.wrap()
            .query_all_balances(Addr::unchecked("bidderpoor"))
            .unwrap(),
        coins(19, "atom")
    );

    //bidder 3 tries to retract : Error

    let resp = app
        .execute_contract(
            Addr::unchecked("bidder3"),
            contract_addr,
            &BidExecuteMsg::Retract { receiver: None },
            &[],
        )
        .unwrap_err();

    assert_eq!(BidError::WinnerCantRetract {}, resp.downcast().unwrap())
}

#[test]
fn test_query_bidding_closed() {
    let mut app = App::default();

    let contract_id = app.store_code(bidding_contract());

    let contract_addr = app
        .instantiate_contract(
            contract_id,
            Addr::unchecked("instantiator"),
            &BidInstantiateMsg {
                owner: Some("owner".to_string()),
                commission: Uint128::from(5u64),
                accepted_token: coin(0, "atom"),
            },
            &[],
            "Bidding Contract",
            None,
        )
        .unwrap();

    let resp: bool = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &BidQueryMsg::IsBiddingClosed {})
        .unwrap();
    assert!(!resp);

    let resp = app
        .execute_contract(
            Addr::unchecked("notowner"),
            contract_addr.clone(),
            &BidExecuteMsg::Close {},
            &[],
        )
        .unwrap_err();

    assert_eq!(BidError::Unauthorized {}, resp.downcast().unwrap());

    app.execute_contract(
        Addr::unchecked("owner"),
        contract_addr.clone(),
        &BidExecuteMsg::Close {},
        &[],
    )
    .unwrap();

    let resp: bool = app
        .wrap()
        .query_wasm_smart(contract_addr, &BidQueryMsg::IsBiddingClosed {})
        .unwrap();
    assert!(resp);
}

#[test]
fn test_query_get_denom() {
    let mut app = App::default();

    let contract_id = app.store_code(bidding_contract());

    let contract_addr = app
        .instantiate_contract(
            contract_id,
            Addr::unchecked("instantiator"),
            &BidInstantiateMsg {
                owner: Some("owner".to_string()),
                commission: Uint128::from(5u64),
                accepted_token: coin(0, "atom"),
            },
            &[],
            "Bidding Contract",
            None,
        )
        .unwrap();

    let resp: String = app
        .wrap()
        .query_wasm_smart(contract_addr, &BidQueryMsg::GetAcceptedDenom {})
        .unwrap();
    assert_eq!("atom".to_string(), resp);
}

#[test]
fn test_query_get_winning_bidder_instant_close() {
    let mut app = App::default();

    let contract_id = app.store_code(bidding_contract());

    let contract_addr = app
        .instantiate_contract(
            contract_id,
            Addr::unchecked("instantiator"),
            &BidInstantiateMsg {
                owner: Some("owner".to_string()),
                commission: Uint128::from(5u64),
                accepted_token: coin(0, "atom"),
            },
            &[],
            "Bidding Contract",
            None,
        )
        .unwrap();

    let resp: Addr = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &BidQueryMsg::GetWinningBidder {})
        .unwrap();

    assert_eq!(resp, Addr::unchecked("nowinneryet"));

    app.execute_contract(
        Addr::unchecked("owner"),
        contract_addr.clone(),
        &BidExecuteMsg::Close {},
        &[],
    )
    .unwrap();

    let resp: Addr = app
        .wrap()
        .query_wasm_smart(contract_addr, &BidQueryMsg::GetWinningBidder {})
        .unwrap();

    assert_eq!(resp, Addr::unchecked("instantiator"));
}

#[test]
fn test_query_get_winning_bidder_bid_and_close() {
    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &Addr::unchecked("bidder"), coins(10, "atom"))
            .unwrap()
    });

    let contract_id = app.store_code(bidding_contract());

    let contract_addr = app
        .instantiate_contract(
            contract_id,
            Addr::unchecked("instantiator"),
            &BidInstantiateMsg {
                owner: Some("owner".to_string()),
                commission: Uint128::from(1u64),
                accepted_token: coin(0, "atom"),
            },
            &[],
            "Bidding Contract",
            None,
        )
        .unwrap();

    app.execute_contract(
        Addr::unchecked("bidder"),
        contract_addr.clone(),
        &BidExecuteMsg::Bid {},
        &coins(4, "atom"),
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked("owner"),
        contract_addr.clone(),
        &BidExecuteMsg::Close {},
        &[],
    )
    .unwrap();

    let resp: Addr = app
        .wrap()
        .query_wasm_smart(contract_addr, &BidQueryMsg::GetWinningBidder {})
        .unwrap();

    assert_eq!(resp, Addr::unchecked("bidder"));
}
