pub mod helpers;
use helpers::*;

use std::str::FromStr;

use archway_reward_manager::{
    msg::{ExecuteMsg, QueryMsg},
    state::{Config, Share},
    ContractError,
};
use cosmwasm_std::{Addr, Decimal};
use cw_multi_test::Executor;

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

    let archway_reward_manager_addr = proper_instantiate(&mut app, shares.clone(), true);

    app.execute_contract(
        Addr::unchecked(ADMIN),
        archway_reward_manager_addr.clone(),
        &ExecuteMsg::LockContract {},
        &vec![],
    )
    .unwrap();

    let res: Config = app
        .wrap()
        .query_wasm_smart(archway_reward_manager_addr.clone(), &QueryMsg::Config {})
        .unwrap();
    assert_eq!(res.mutable, false);

    let err = app
        .execute_contract(
            Addr::unchecked(ADMIN),
            archway_reward_manager_addr.clone(),
            &ExecuteMsg::UpdateShares { shares },
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

    let archway_reward_manager_addr = proper_instantiate(&mut app, shares.clone(), true);

    let err = app
        .execute_contract(
            Addr::unchecked(USER),
            archway_reward_manager_addr.clone(),
            &ExecuteMsg::LockContract {},
            &vec![],
        )
        .unwrap_err();
    assert_eq!(
        err.source().unwrap().to_string(),
        ContractError::Unauthorized {}.to_string()
    )
}
