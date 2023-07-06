use std::str::FromStr;

use archway_bindings::{ArchwayMsg, ArchwayQuery};
use cosmwasm_std::{testing::MockApi, Addr, Coin, Decimal, MemoryStorage, Uint128};
use cw_multi_test::{
    custom_app, App, BankKeeper, Contract, ContractWrapper, Executor, FailingModule, WasmKeeper,
};
use pantheon_splitter::{msg::InstantiateMsg, state::Share};

pub fn factory_contract() -> Box<dyn Contract<ArchwayMsg, ArchwayQuery>> {
    let contract = ContractWrapper::new(
        pantheon_splitter::contract::execute,
        pantheon_splitter::contract::instantiate,
        pantheon_splitter::contract::query,
    )
    .with_reply(pantheon_splitter::contract::reply);
    Box::new(contract)
}

pub const ADMIN: &str = "admin";
pub const USER: &str = "user";
pub const USER2: &str = "user2";
pub const USER3: &str = "user3";

pub const DENOM: &str = "aconst";

pub fn mock_app() -> App<
    BankKeeper,
    MockApi,
    MemoryStorage,
    FailingModule<ArchwayMsg, ArchwayQuery, cosmwasm_std::Empty>,
    WasmKeeper<ArchwayMsg, ArchwayQuery>,
> {
    custom_app::<ArchwayMsg, ArchwayQuery, _>(|router, _, storage| {
        router
            .bank
            .init_balance(
                storage,
                &Addr::unchecked(ADMIN),
                vec![Coin {
                    denom: DENOM.to_string(),
                    amount: Uint128::new(1_000_000_000_000_000_000_000_000_000_000),
                }],
            )
            .unwrap();
    })
}

pub fn proper_instantiate(
    app: &mut App<
        BankKeeper,
        MockApi,
        MemoryStorage,
        FailingModule<ArchwayMsg, ArchwayQuery, cosmwasm_std::Empty>,
        WasmKeeper<ArchwayMsg, ArchwayQuery>,
    >,
    shares: Vec<Share>,
    mutable: bool,
) -> Addr {
    let code_id = app.store_code(factory_contract());

    app.instantiate_contract(
        code_id,
        Addr::unchecked(ADMIN),
        &InstantiateMsg { shares, mutable },
        &vec![],
        "Pantheon Splitter",
        None,
    )
    .unwrap()
}

pub fn proper_instantiate_with_shares(
    app: &mut App<
        BankKeeper,
        MockApi,
        MemoryStorage,
        FailingModule<ArchwayMsg, ArchwayQuery, cosmwasm_std::Empty>,
        WasmKeeper<ArchwayMsg, ArchwayQuery>,
    >,
) -> Addr {
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
            mutable: true,
        },
        &vec![],
        "Pantheon Splitter",
        None,
    )
    .unwrap()
}
