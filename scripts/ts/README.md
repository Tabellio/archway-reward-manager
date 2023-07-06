# Pantheon Scripts

These scripts are used to help manage the Pantheon.

## Prerequisites

- [Node.js](https://nodejs.org/en/)
- [Yarn](https://yarnpkg.com/)
- [Rust](https://www.rust-lang.org/tools/install)

## Getting Started

Install the dependencies:

```bash
yarn install
```

Copy the `.env.example` file to `.env` and update the variables.

```bash
cp .env.example .env
```

You need 3 mnemonics for each actor. **Admin**, **User** and **User2**. Update the `ADMIN_MNEMONIC`, `USER_MNEMONIC` and `USER2_MNEMONIC` variables in the `.env` file.

Now you are ready to run the scripts!

### Upload Script

> **Note**: You need to compile your contract before uploading it to the blockchain. Refer to the [Getting Started](../../README.md#getting-started) section for more information.

After getting your two wasm files, update the `FACTORY_WASM_FILE_PATH` and `CUSTOM_WASM_FILE_PATH` variables in the `.env` file.

To upload the contracts to the blockchain, use the following command:

```bash
yarn upload
```

After the script is executed, you will see the contract code IDs in the terminal. Copy the code IDs and update the `FACTORY_CONTRACT_CODE_ID` and `CUSTOM_CONTRACT_CODE_ID` variables in the `.env` file.

You are now ready to use the start script!

### Start Script

> **Note**: You first need to upload your contracts to the blockchain. Refer to the [Upload Script](#upload-script) section for more information.

Start script is used to execute multiple operations at once. It is used to try out Pantheon.

1. It creates a mutable factory contract with 25% User and 75% User2 shares.
2. It updates the shares to be 35% and 65% respectively.
3. It adds a the custom contract provided in the repository for managing rewards.
4. It executes multiple transactions on the custom contract for accumulating rewards.
5. It executes two transactions for withdrawing rewards and distributing the native tokens.

To run the script, use the following command:

```bash
yarn start
```
