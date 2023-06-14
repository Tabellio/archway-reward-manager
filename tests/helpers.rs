use archway_bindings::{ArchwayMsg, ArchwayQuery, ArchwayResult};
use archway_reward_manager::ContractError;
use cosmwasm_std::{testing::MockApi, Addr, Coin, Empty, MemoryStorage, Uint128};
use cw_multi_test::{
    custom_app, App, AppBuilder, BankKeeper, Contract, ContractWrapper, Executor, FailingModule,
    WasmKeeper,
};

pub fn archway_reward_manager_contract() -> Box<dyn Contract<ArchwayMsg, ArchwayQuery>> {
    let contract = ContractWrapper::new(
        archway_reward_manager::contract::execute,
        archway_reward_manager::contract::instantiate,
        archway_reward_manager::contract::query,
    );
    Box::new(contract)
}

pub const ADMIN: &str = "admin";
pub const USER: &str = "user";
pub const USER2: &str = "user2";
pub const USER3: &str = "user3";

pub const DENOM: &str = "uconst";

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
                    amount: Uint128::new(1_000_000),
                }],
            )
            .unwrap();
    })
}
