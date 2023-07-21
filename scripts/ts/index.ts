import { toBinary } from "@cosmjs/cosmwasm-stargate"
import { getClientsAndAccounts } from "./wallet"
import dotenv from "dotenv"

dotenv.config()

const FACTORY_CONTRACT_CODE_ID = process.env.FACTORY_CONTRACT_CODE_ID || ""
const SPLITTER_CONTRACT_CODE_ID = process.env.SPLITTER_CONTRACT_CODE_ID || ""
const CUSTOM_CONTRACT_CODE_ID = process.env.CUSTOM_CONTRACT_CODE_ID || ""

;(async () => {
  const { adminClient, adminAccount, userClient, userAccount, user2Account } =
    await getClientsAndAccounts()

  const createFactory = async () => {
    const res = await adminClient.instantiate(
      adminAccount.address,
      Number(FACTORY_CONTRACT_CODE_ID),
      {
        splitter_code_id: Number(SPLITTER_CONTRACT_CODE_ID),
      },
      "Pantheon Factory",
      "auto",
      {
        admin: adminAccount.address,
      }
    )

    console.log("\n🟠 Factory Contract Address: ", res.contractAddress, "\n")

    return res.contractAddress
  }

  const updateFactoryRewardsMetadata = async (contractAddress: string) => {
    const res = await adminClient.setContractMetadata(
      adminAccount.address,
      {
        contractAddress,
        ownerAddress: adminAccount.address,
        rewardsAddress: adminAccount.address,
      },
      "auto"
    )

    console.log(
      "\n🟠 Update Factory Contract Rewards Metadata TxHash: ",
      res.transactionHash,
      "\n"
    )
  }

  const createNewSpliter = async (contractAddress: string) => {
    const res = await userClient.execute(
      userAccount.address,
      contractAddress,
      {
        create_splitter: {
          shares: [
            { recipient: userAccount.address, percentage: "0.25" },
            { recipient: user2Account.address, percentage: "0.75" },
          ],
          mutable: true,
          label: "My Splitter Contract",
        },
      },
      "auto"
    )

    let address = res.events
      .filter((event) => event.type === "instantiate")[0]
      .attributes.filter(
        (attribute) => attribute.key === "_contract_address"
      )[0].value

    console.log("\n🟠 Splitter Contract Address: ", address, "\n")

    return address
  }

  const updateShares = async (contractAddress: string) => {
    let res = await userClient.execute(
      userAccount.address,
      contractAddress,
      {
        update_shares: {
          shares: [
            { recipient: userAccount.address, percentage: "0.35" },
            { recipient: user2Account.address, percentage: "0.65" },
          ],
        },
      },
      "auto"
    )

    console.log("\n🟠 Update Shares TxHash: ", res.transactionHash, "\n")
  }

  const addCustomContract = async (contractAddress: string) => {
    let res = await userClient.execute(
      userAccount.address,
      contractAddress,
      {
        add_custom_contract: {
          code_id: Number(CUSTOM_CONTRACT_CODE_ID),
          msg: toBinary({}),
        },
      },
      "auto"
    )

    console.log("\n🟠 Add Custom Contract TxHash: ", res.transactionHash, "\n")

    console.log(
      "\n🟠 Custom Contract Address: ",
      res.events
        .filter((event) => event.type === "instantiate")[0]
        .attributes.filter(
          (attribute) => attribute.key === "_contract_address"
        )[0].value,
      "\n"
    )

    return res.events
      .filter((event) => event.type === "instantiate")[0]
      .attributes.filter(
        (attribute) => attribute.key === "_contract_address"
      )[0].value
  }

  const executeCustomContract = async (contractAddress: string) => {
    let res = await adminClient.executeMultiple(
      adminAccount.address,
      [
        {
          contractAddress,
          msg: {
            increment: {},
          },
        },
        {
          contractAddress,
          msg: {
            increment: {},
          },
        },
        {
          contractAddress,
          msg: {
            increment: {},
          },
        },
        {
          contractAddress,
          msg: {
            increment: {},
          },
        },
        {
          contractAddress,
          msg: {
            increment: {},
          },
        },
      ],
      "auto"
    )

    console.log(
      "\n🟠 Execute Custom Contract TxHash: ",
      res.transactionHash,
      "\n"
    )
  }

  const distributeSplitterRewards = async (contractAddress: string) => {
    let rewardsBalance = await adminClient.getAllRewardsRecords(contractAddress)

    console.log(
      "\n🟠 Rewards Balance: ",
      JSON.stringify(rewardsBalance, null, 2),
      "\n"
    )

    let userBalance = await adminClient.getBalance(
      userAccount.address,
      "aconst"
    )
    let user2Balance = await adminClient.getBalance(
      user2Account.address,
      "aconst"
    )

    console.log(
      "\n🟠 User Balance: ",
      JSON.stringify(userBalance, null, 2),
      "\n"
    )
    console.log(
      "\n🟠 User2 Balance: ",
      JSON.stringify(user2Balance, null, 2),
      "\n"
    )

    let res = await userClient.executeMultiple(
      userAccount.address,
      [
        {
          contractAddress,
          msg: {
            withdraw_rewards: {},
          },
        },
        {
          contractAddress,
          msg: {
            distribute_native_tokens: {},
          },
        },
      ],
      "auto"
    )

    console.log("\n🟠 Distribute Rewards TxHash: ", res.transactionHash, "\n")

    userBalance = await adminClient.getBalance(userAccount.address, "aconst")
    user2Balance = await adminClient.getBalance(user2Account.address, "aconst")

    console.log(
      "\n🟠 User Balance: ",
      JSON.stringify(userBalance, null, 2),
      "\n"
    )
    console.log(
      "\n🟠 User2 Balance: ",
      JSON.stringify(user2Balance, null, 2),
      "\n"
    )
  }

  // Create factory
  const factoryContractAddress = await createFactory()

  // Update factory contract rewards metadata
  await updateFactoryRewardsMetadata(factoryContractAddress)

  // Create new splitter
  const splitterContractAddress = await createNewSpliter(factoryContractAddress)

  // Update shares of users
  await updateShares(splitterContractAddress)

  // Add custom contract to factory
  const customContractAddress = await addCustomContract(splitterContractAddress)

  // Execute custom contract multiple times to generate rewards
  await executeCustomContract(customContractAddress)

  // Distribute splitter rewards to users based on their shares
  await distributeSplitterRewards(splitterContractAddress)
})()
