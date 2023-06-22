import { makeCosmoshubPath } from "@cosmjs/amino"
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing"
import { LedgerSigner } from "@cosmjs/ledger-amino"
import TransportNodeHid from "@ledgerhq/hw-transport-node-hid"
import { SigningArchwayClient } from "@archwayhq/arch3-core"
import dotenv from "dotenv"

dotenv.config()

const TESTNET_RPC = "https://rpc.constantine.archway.tech:443"

const PREFIX = "archway"

const ADMIN_MNEMONIC = process.env.ADMIN_MNEMONIC || ""
const USER_MNEMONIC = process.env.USER_MNEMONIC || ""
const USER2_MNEMONIC = process.env.USER2_MNEMONIC || ""

export const getSigner = async (mnemonic: string) => {
  const signer = await DirectSecp256k1HdWallet.fromMnemonic(mnemonic, {
    hdPaths: [makeCosmoshubPath(0)],
    prefix: PREFIX,
  })
  return signer
}

export const getLedgerSigner = async () => {
  const transport = await TransportNodeHid.create()
  return new LedgerSigner(transport, {
    hdPaths: [makeCosmoshubPath(0)],
    prefix: PREFIX,
  })
}

export const getClientsAndAccounts = async () => {
  const adminSigner = await getSigner(ADMIN_MNEMONIC)
  const adminAccount = (await adminSigner.getAccounts())[0]

  const userSigner = await getSigner(USER_MNEMONIC)
  const userAccount = (await userSigner.getAccounts())[0]

  const user2Signer = await getSigner(USER2_MNEMONIC)
  const user2Account = (await user2Signer.getAccounts())[0]

  const adminClient = await SigningArchwayClient.connectWithSigner(
    TESTNET_RPC,
    adminSigner
  )
  const userClient = await SigningArchwayClient.connectWithSigner(
    TESTNET_RPC,
    userSigner
  )
  const user2Client = await SigningArchwayClient.connectWithSigner(
    TESTNET_RPC,
    user2Signer
  )

  return {
    adminAccount,
    adminClient,
    userAccount,
    userClient,
    user2Account,
    user2Client,
  }
}
