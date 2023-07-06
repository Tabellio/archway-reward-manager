pub mod helpers;
use helpers::*;

use cosmwasm_std::{coins, Addr, Uint128};
use cw_multi_test::Executor;
use pantheon_splitter::msg::ExecuteMsg;

#[test]
fn test_happy_path() {
    let mut app = mock_app();
    let factory_addr = proper_instantiate_with_shares(&mut app);

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
