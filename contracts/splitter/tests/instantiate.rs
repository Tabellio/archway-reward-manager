pub mod helpers;
use helpers::*;

use std::str::FromStr;

use cosmwasm_std::{Addr, Decimal};
use cw_multi_test::Executor;

use pantheon_splitter::{
    msg::{InstantiateMsg, QueryMsg},
    state::Share,
    ContractError,
};

#[test]
fn test_happy_path() {
    let mut app = mock_app();

    let code_id = app.store_code(factory_contract());

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

    app.instantiate_contract(
        code_id,
        Addr::unchecked(ADMIN),
        &InstantiateMsg {
            shares,
            mutable: false,
        },
        &vec![],
        "Pantheon Splitter",
        None,
    )
    .unwrap();

    let res = app.contract_data(&Addr::unchecked("contract0")).unwrap();
    assert_eq!(res.code_id as u64, code_id);
    assert_eq!(res.creator, Addr::unchecked(ADMIN));
    assert_eq!(res.admin, None);
    assert_eq!(res.label, "Pantheon Splitter");

    let res: Vec<Share> = app
        .wrap()
        .query_wasm_smart(
            Addr::unchecked("contract0"),
            &QueryMsg::Shares {
                start_after: None,
                limit: None,
            },
        )
        .unwrap();
    assert_eq!(res.len(), 3);
    assert_eq!(res[0].recipient, USER.to_string());
    assert_eq!(res[0].percentage, Decimal::from_str("0.52").unwrap());
    assert_eq!(res[1].recipient, USER2.to_string());
    assert_eq!(res[1].percentage, Decimal::from_str("0.25").unwrap());
    assert_eq!(res[2].recipient, USER3.to_string());
    assert_eq!(res[2].percentage, Decimal::from_str("0.23").unwrap());
}

#[test]
fn test_percentage_limit_exceeded() {
    let mut app = mock_app();

    let code_id = app.store_code(factory_contract());

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
            percentage: Decimal::from_str("0.30").unwrap(),
        },
    ];

    let err = app
        .instantiate_contract(
            code_id,
            Addr::unchecked(ADMIN),
            &InstantiateMsg {
                shares,
                mutable: false,
            },
            &vec![],
            "Pantheon Splitter",
            None,
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

    let code_id = app.store_code(factory_contract());

    let shares = vec![
        Share {
            recipient: USER.to_string(),
            percentage: Decimal::from_str("0.52").unwrap(),
        },
        Share {
            recipient: USER2.to_string(),
            percentage: Decimal::from_str("0.25").unwrap(),
        },
    ];

    let err = app
        .instantiate_contract(
            code_id,
            Addr::unchecked(ADMIN),
            &InstantiateMsg {
                shares,
                mutable: false,
            },
            &vec![],
            "Pantheon Splitter",
            None,
        )
        .unwrap_err();
    assert_eq!(
        err.source().unwrap().to_string(),
        ContractError::PercentageLimitNotMet {}.to_string()
    )
}
