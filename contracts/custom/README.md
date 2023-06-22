# CosmWasm Template Contract

This is a simple template from the CosmWasm [repository](https://github.com/CosmWasm/cw-template.git).

We will use this contract to demonstrate how to prepare a custom contract for the factory contract.

## Prerequisites

1. Add [archway-bindings](https://crates.io/crates/archway-bindings) dependency to your `Cargo.toml` file.

```toml
[dependencies]
archway-bindings = "0.1.0"
```

2. Use the new `ArchwayResult` and `ArchwayQuery` in your contract code. Take a look at this [sample repository](https://github.com/archway-network/archway-bindings/blob/main/contracts/increment/src/contract.rs) from Archway to see how to use them.

3. Add [archway-rewards-manager-utils]() dependency to your `Cargo.toml` file.

```toml
[dependencies]
archway-rewards-manager-utils = <add github link here>
```

4. Add the following execute message to your contract's execute messages.

```rust
#[cw_serde]
pub enum ExecuteMsg {
    // ... other execute messages
    UpdateRewardMetadata {
        owner_address: Option<String>,
        rewards_address: Option<String>,
    },
}
```

5. Import `ArchwayRewardManagerUtils` and add the following implementation under your `ExecuteMsg` enum. Update if you already have an implementation.

```rust
// Import the util message from the archway-rewards-manager-utils package
use archway_reward_manager_utils::ExecuteMsg as ArchwayRewardManagerUtils;

#[cw_serde]
pub enum ExecuteMsg {
    // ... other execute messages
    UpdateRewardMetadata {
        owner_address: Option<String>,
        rewards_address: Option<String>,
    },
}

// Add or update the implementation if needed
impl From<ExecuteMsg> for ArchwayRewardManagerUtils {
    fn from(msg: ExecuteMsg) -> Self {
        match msg {
            ExecuteMsg::UpdateRewardMetadata {
                owner_address,
                rewards_address,
            } => ArchwayRewardManagerUtils::UpdateRewardMetadata {
                owner_address,
                rewards_address,
            },
            _ => unreachable!("Cannot convert {:?} to ArchwayRewardManagerUtils", msg),
        }
    }
}
```

6. Add the following execute method to your contract logic

```rust
fn execute_update_reward_metadata(
    _deps: DepsMut<ArchwayQuery>,
    _env: Env,
    _info: MessageInfo,
    owner_address: Option<String>,
    rewards_address: Option<String>,
) -> ArchwayResult<ContractError> {
    // ... write custom logic if needed

    let msg = ArchwayMsg::UpdateContractMetadata {
        owner_address,
        rewards_address,
    };

    Ok(Response::new()
        .add_message(msg)
        .add_attribute("method", "update_rewards_address"))
}
```

You are now ready to use the contract with the factory contract!
