use archway_bindings::{ArchwayMsg, ArchwayQuery};
use archway_reward_manager_factory::{msg::InstantiateMsg, state::Share};
use cosmwasm_std::{testing::MockApi, Addr, Coin, MemoryStorage, Uint128};
use cw_multi_test::{
    custom_app, App, BankKeeper, Contract, ContractWrapper, Executor, FailingModule, WasmKeeper,
};

pub fn factory_contract() -> Box<dyn Contract<ArchwayMsg, ArchwayQuery>> {
    let contract = ContractWrapper::new(
        archway_reward_manager_factory::contract::execute,
        archway_reward_manager_factory::contract::instantiate,
        archway_reward_manager_factory::contract::query,
    );
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
        "Archway Reward Manager",
        None,
    )
    .unwrap()
}
