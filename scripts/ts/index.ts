import { toBinary } from "@cosmjs/cosmwasm-stargate"
import { getClientsAndAccounts } from "./wallet"
import dotenv from "dotenv"

dotenv.config()

const FACTORY_CONTRACT_CODE_ID = process.env.FACTORY_CONTRACT_CODE_ID || ""
const CUSTOM_CONTRACT_CODE_ID = process.env.CUSTOM_CONTRACT_CODE_ID || ""

;(async () => {
  const { adminClient, adminAccount, userAccount, user2Account } =
    await getClientsAndAccounts()

  const createNewFactory = async () => {
    const res = await adminClient.instantiate(
      adminAccount.address,
      Number(FACTORY_CONTRACT_CODE_ID),
      {
        shares: [
          { recipient: userAccount.address, percentage: "0.25" },
          { recipient: user2Account.address, percentage: "0.75" },
        ],
        mutable: true,
      },
      "Archway Reward Manager Factory",
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
        ownerAddress: contractAddress,
        rewardsAddress: contractAddress,
      },
      "auto"
    )

    console.log(
      "\n🟠 Update Factory Contract Rewards Metadata TxHash: ",
      res.transactionHash,
      "\n"
    )
  }

  const updateShares = async (contractAddress: string) => {
    let res = await adminClient.execute(
      adminAccount.address,
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
    let res = await adminClient.execute(
      adminAccount.address,
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

  const distributeRewards = async (contractAddress: string) => {
    let rewardsBalance = adminClient.getAllRewardsRecords(contractAddress)

    console.log(
      "\n🟠 Rewards Balance: ",
      JSON.stringify(rewardsBalance, null, 2),
      "\n"
    )

    let userBalance = adminClient.getBalance(userAccount.address, "aconst")
    let user2Balance = adminClient.getBalance(user2Account.address, "aconst")

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

    let res = await adminClient.executeMultiple(
      adminAccount.address,
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

    userBalance = adminClient.getBalance(userAccount.address, "aconst")
    user2Balance = adminClient.getBalance(user2Account.address, "aconst")

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

  // Create new factory
  const factoryContractAddress = await createNewFactory()

  // Update factory contract rewards metadata
  await updateFactoryRewardsMetadata(factoryContractAddress)

  // Update shares of users
  await updateShares(factoryContractAddress)

  // Add custom contract to factory
  const customContractAddress = await addCustomContract(factoryContractAddress)

  // Execute custom contract multiple times to generate rewards
  await executeCustomContract(customContractAddress)

  // Distribute rewards to users based on their shares
  await distributeRewards(factoryContractAddress)
})()
