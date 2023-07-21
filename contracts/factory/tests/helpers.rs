use archway_bindings::{ArchwayMsg, ArchwayQuery};
use cosmwasm_std::{testing::MockApi, Addr, Coin, MemoryStorage, Uint128};
use cw_multi_test::{
    custom_app, App, BankKeeper, Contract, ContractWrapper, Executor, FailingModule, WasmKeeper,
};
use pantheon_factory::msg::InstantiateMsg;

pub fn factory_contract() -> Box<dyn Contract<ArchwayMsg, ArchwayQuery>> {
    let contract = ContractWrapper::new(
        pantheon_factory::contract::execute,
        pantheon_factory::contract::instantiate,
        pantheon_factory::contract::query,
    );
    Box::new(contract)
}
pub fn splitter_contract() -> Box<dyn Contract<ArchwayMsg, ArchwayQuery>> {
    let contract = ContractWrapper::new(
        pantheon_splitter::contract::execute,
        pantheon_splitter::contract::instantiate,
        pantheon_splitter::contract::query,
    );
    Box::new(contract)
}

pub const ADMIN: &str = "admin";
pub const USER: &str = "user";

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
                    amount: Uint128::new(1),
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
) -> Addr {
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
    .unwrap()
}
