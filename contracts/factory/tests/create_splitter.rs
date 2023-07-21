pub mod helpers;
use helpers::*;

use std::str::FromStr;

use cosmwasm_std::{Addr, Decimal};
use cw_multi_test::Executor;

use pantheon_factory::msg::ExecuteMsg;
use pantheon_utils::Share;

// #[test]
fn test_happy_path() {
    let mut app = mock_app();
    let factory_address = proper_instantiate(&mut app);

    let shares = vec![Share {
        recipient: USER.to_string(),
        percentage: Decimal::from_str("1").unwrap(),
    }];

    app.execute_contract(
        Addr::unchecked(USER),
        factory_address.clone(),
        &ExecuteMsg::CreateSplitter {
            shares,
            mutable: false,
            label: "My First Splitter".to_string(),
        },
        &vec![],
    )
    .unwrap();

    let res = app.contract_data(&Addr::unchecked("contract3")).unwrap();
    assert_eq!(res.code_id as u64, 2);
    assert_eq!(res.creator, factory_address);
    assert_eq!(res.admin, Some(Addr::unchecked(USER)));
    assert_eq!(res.label, "Pantheon Splitter");
}
