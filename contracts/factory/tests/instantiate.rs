pub mod helpers;
use helpers::*;

use cosmwasm_std::Addr;
use cw_multi_test::Executor;

use pantheon_factory::msg::{InstantiateMsg, QueryMsg};

#[test]
fn test_happy_path() {
    let mut app = mock_app();

    let factory_code_id = app.store_code(factory_contract());
    let splitter_code_id = app.store_code(splitter_contract());

    app.instantiate_contract(
        factory_code_id,
        Addr::unchecked(ADMIN),
        &InstantiateMsg { splitter_code_id },
        &vec![],
        "Pantheon Factory",
        None,
    )
    .unwrap();

    let res = app.contract_data(&Addr::unchecked("contract0")).unwrap();
    assert_eq!(res.code_id as u64, factory_code_id);
    assert_eq!(res.creator, Addr::unchecked(ADMIN));
    assert_eq!(res.admin, None);
    assert_eq!(res.label, "Pantheon Factory");

    let res: u64 = app
        .wrap()
        .query_wasm_smart("contract0", &QueryMsg::SplitterCodeID {})
        .unwrap();
    assert_eq!(res, splitter_code_id);
}
