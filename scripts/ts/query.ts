import { getClientsAndAccounts } from "./wallet"
import dotenv from "dotenv"

dotenv.config()
;(async () => {
  const { adminClient, adminAccount } = await getClientsAndAccounts()

  let rew = await adminClient.getAllRewardsRecords(adminAccount.address)
  console.log(
    "\n🟠 Outstanding Rewards Records: ",
    JSON.stringify(rew, null, 2),
    "\n"
  )

  await adminClient.withdrawContractRewards(adminAccount.address, 0, "auto")
})()
