# Archway Reward Manager

Archway Reward Manager is a platform to make reward and payment related process easier to manage in Archway Network.

> **Known Issue**: Normally the first time if we want to update a contract's reward metadata, we have to do it with the contract's admin address. In our case when we are creating a custom contract through the factory we set the admin as the factory contract address.
>
> For some reason this metadata update does not work. The idea and code is there but needs to be debugged to fix this issue.

## Features

- **User Shares**: Create multiple share structures to distribute rewards and payments to different parties.

- **Custom Contracts**: Create new contracts through the split contract to capture rewards on the split contract, making it earn rewards for every transaction on the custom contract.

- **Contract Lock**: Lock the contract to prevent any further changes to the contract making it more secure.

- **Reward and Payment Distribution**: Distribute the accumulated rewards on the factory contract or custom contracts and the payments to the users.

## Getting Started

To compile the contracts and create the wasm files, run the following command:

```bash
./scripts/optimize.sh
```

To run the tests, run the following command:

```bash
cargo test
```

To try out the contracts take a look at the [TS scripts README file](./scripts/ts/README.md)

## License

This project is licensed under the [Apache License, Version 2.0](./LICENSE). Feel free to use, modify, and distribute this project in accordance with the terms of the license.

## Contact

For any questions or inquiries about the Subscription Hub, please reach out to the project maintainers:

- Maintainer Name: [findolor](https://github.com/findolor)
