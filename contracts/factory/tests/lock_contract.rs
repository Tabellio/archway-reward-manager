pub mod helpers;
use helpers::*;

use archway_reward_manager_factory::{
    msg::{ExecuteMsg, QueryMsg},
    state::Config,
    ContractError,
};
use cosmwasm_std::Addr;
use cw_multi_test::Executor;

#[test]
fn test_happy_path() {
    let mut app = mock_app();
    let factory_addr = proper_instantiate_with_shares(&mut app);

    app.execute_contract(
        Addr::unchecked(ADMIN),
        factory_addr.clone(),
        &ExecuteMsg::LockContract {},
        &vec![],
    )
    .unwrap();

    let res: Config = app
        .wrap()
        .query_wasm_smart(factory_addr.clone(), &QueryMsg::Config {})
        .unwrap();
    assert_eq!(res.mutable, false);

    let err = app
        .execute_contract(
            Addr::unchecked(ADMIN),
            factory_addr.clone(),
            &ExecuteMsg::UpdateShares { shares: vec![] },
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
    let factory_addr = proper_instantiate_with_shares(&mut app);

    let err = app
        .execute_contract(
            Addr::unchecked(USER),
            factory_addr.clone(),
            &ExecuteMsg::LockContract {},
            &vec![],
        )
        .unwrap_err();
    assert_eq!(
        err.source().unwrap().to_string(),
        ContractError::Unauthorized {}.to_string()
    )
}
