pub mod helpers;
use helpers::*;

use std::str::FromStr;

use cosmwasm_std::{Addr, Decimal};
use cw_multi_test::Executor;
use pantheon_splitter::{
    msg::{ExecuteMsg, QueryMsg},
    ContractError,
};

use pantheon_utils::Share;

#[test]
fn test_happy_path() {
    let mut app = mock_app();

    let shares = vec![
        Share {
            recipient: USER.to_string(),
            percentage: Decimal::from_str("0.52").unwrap(),
        },
        Share {
            recipient: USER2.to_string(),
            percentage: Decimal::from_str("0.25").unwrap(),
        },
        Share {
            recipient: USER3.to_string(),
            percentage: Decimal::from_str("0.23").unwrap(),
        },
    ];

    let factory_addr = proper_instantiate(&mut app, shares, true);

    let new_shares = vec![
        Share {
            recipient: USER2.to_string(),
            percentage: Decimal::from_str("0.38").unwrap(),
        },
        Share {
            recipient: USER3.to_string(),
            percentage: Decimal::from_str("0.62").unwrap(),
        },
    ];

    app.execute_contract(
        Addr::unchecked(ADMIN),
        factory_addr.clone(),
        &ExecuteMsg::UpdateShares { shares: new_shares },
        &vec![],
    )
    .unwrap();

    let res: Vec<Share> = app
        .wrap()
        .query_wasm_smart(
            factory_addr,
            &QueryMsg::Shares {
                start_after: None,
                limit: None,
            },
        )
        .unwrap();
    assert_eq!(res.len(), 2);
    assert_eq!(res[0].recipient, USER2.to_string());
    assert_eq!(res[0].percentage, Decimal::from_str("0.38").unwrap());
    assert_eq!(res[1].recipient, USER3.to_string());
    assert_eq!(res[1].percentage, Decimal::from_str("0.62").unwrap());
}

#[test]
fn test_locked_contract() {
    let mut app = mock_app();

    let shares = vec![
        Share {
            recipient: USER.to_string(),
            percentage: Decimal::from_str("0.52").unwrap(),
        },
        Share {
            recipient: USER2.to_string(),
            percentage: Decimal::from_str("0.25").unwrap(),
        },
        Share {
            recipient: USER3.to_string(),
            percentage: Decimal::from_str("0.23").unwrap(),
        },
    ];

    let factory_addr = proper_instantiate(&mut app, shares, false);

    let new_shares = vec![
        Share {
            recipient: USER2.to_string(),
            percentage: Decimal::from_str("0.38").unwrap(),
        },
        Share {
            recipient: USER3.to_string(),
            percentage: Decimal::from_str("0.62").unwrap(),
        },
    ];

    let err = app
        .execute_contract(
            Addr::unchecked(ADMIN),
            factory_addr.clone(),
            &ExecuteMsg::UpdateShares { shares: new_shares },
            &vec![],
        )
        .unwrap_err();
    assert_eq!(
        err.source().unwrap().to_string(),
        ContractError::ContractNotMutable {}.to_string()
    )
}

#[test]
fn test_invalid_admin() {
    let mut app = mock_app();

    let shares = vec![
        Share {
            recipient: USER.to_string(),
            percentage: Decimal::from_str("0.52").unwrap(),
        },
        Share {
            recipient: USER2.to_string(),
            percentage: Decimal::from_str("0.25").unwrap(),
        },
        Share {
            recipient: USER3.to_string(),
            percentage: Decimal::from_str("0.23").unwrap(),
        },
    ];

    let factory_addr = proper_instantiate(&mut app, shares, true);

    let new_shares = vec![
        Share {
            recipient: USER2.to_string(),
            percentage: Decimal::from_str("0.38").unwrap(),
        },
        Share {
            recipient: USER3.to_string(),
            percentage: Decimal::from_str("0.62").unwrap(),
        },
    ];

    let err = app
        .execute_contract(
            Addr::unchecked(USER),
            factory_addr.clone(),
            &ExecuteMsg::UpdateShares { shares: new_shares },
            &vec![],
        )
        .unwrap_err();
    assert_eq!(
        err.source().unwrap().to_string(),
        ContractError::Unauthorized {}.to_string()
    )
}

#[test]
fn test_percentage_limit_exceeded() {
    let mut app = mock_app();

    let shares = vec![
        Share {
            recipient: USER.to_string(),
            percentage: Decimal::from_str("0.52").unwrap(),
        },
        Share {
            recipient: USER2.to_string(),
            percentage: Decimal::from_str("0.25").unwrap(),
        },
        Share {
            recipient: USER3.to_string(),
            percentage: Decimal::from_str("0.23").unwrap(),
        },
    ];

    let factory_addr = proper_instantiate(&mut app, shares, true);

    let new_shares = vec![
        Share {
            recipient: USER2.to_string(),
            percentage: Decimal::from_str("2.38").unwrap(),
        },
        Share {
            recipient: USER3.to_string(),
            percentage: Decimal::from_str("0.62").unwrap(),
        },
    ];

    let err = app
        .execute_contract(
            Addr::unchecked(ADMIN),
            factory_addr.clone(),
            &ExecuteMsg::UpdateShares { shares: new_shares },
            &vec![],
        )
        .unwrap_err();
    assert_eq!(
        err.source().unwrap().to_string(),
        ContractError::PercentageLimitExceeded {}.to_string()
    )
}

#[test]
fn test_percentage_limit_not_met() {
    let mut app = mock_app();

    let shares = vec![
        Share {
            recipient: USER.to_string(),
            percentage: Decimal::from_str("0.52").unwrap(),
        },
        Share {
            recipient: USER2.to_string(),
            percentage: Decimal::from_str("0.25").unwrap(),
        },
        Share {
            recipient: USER3.to_string(),
            percentage: Decimal::from_str("0.23").unwrap(),
        },
    ];

    let factory_addr = proper_instantiate(&mut app, shares, true);

    let new_shares = vec![
        Share {
            recipient: USER2.to_string(),
            percentage: Decimal::from_str("0.10").unwrap(),
        },
        Share {
            recipient: USER3.to_string(),
            percentage: Decimal::from_str("0.62").unwrap(),
        },
    ];

    let err = app
        .execute_contract(
            Addr::unchecked(ADMIN),
            factory_addr.clone(),
            &ExecuteMsg::UpdateShares { shares: new_shares },
            &vec![],
        )
        .unwrap_err();
    assert_eq!(
        err.source().unwrap().to_string(),
        ContractError::PercentageLimitNotMet {}.to_string()
    )
}
