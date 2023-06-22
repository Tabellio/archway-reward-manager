pub mod helpers;
use helpers::*;

use std::str::FromStr;

use archway_reward_manager_factory::{msg::ExecuteMsg, state::Share};
use cosmwasm_std::{coins, Addr, Decimal, Uint128};
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

    let factory_addr = proper_instantiate(&mut app, shares.clone(), true);

    // Send some tokens to the contract for simulating the reward withdrawal
    // Normally this will be the WithdrawRewards message
    app.send_tokens(
        Addr::unchecked(ADMIN),
        factory_addr.clone(),
        &coins(12_560_000_000_000_000_000_000, "aconst"),
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked(ADMIN),
        factory_addr.clone(),
        &ExecuteMsg::DistributeNativeTokens {},
        &vec![],
    )
    .unwrap();

    let res = app.wrap().query_balance(USER, "aconst").unwrap();
    assert_eq!(res.amount, Uint128::new(6_531_200_000_000_000_000_000));

    let res = app.wrap().query_balance(USER2, "aconst").unwrap();
    assert_eq!(res.amount, Uint128::new(3_140_000_000_000_000_000_000));

    let res = app.wrap().query_balance(USER3, "aconst").unwrap();
    assert_eq!(res.amount, Uint128::new(2_888_800_000_000_000_000_000))
}
