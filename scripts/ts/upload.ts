import fs from "fs"
import { getClientsAndAccounts } from "./wallet"
import dotenv from "dotenv"
;(async () => {
  dotenv.config()

  const { adminClient, adminAccount } = await getClientsAndAccounts()

  const FACTORY_WASM_FILE_PATH = process.env.FACTORY_WASM_FILE_PATH || ""
  const SPLITTER_WASM_FILE_PATH = process.env.SPLITTER_WASM_FILE_PATH || ""
  // const CUSTOM_WASM_FILE_PATH = process.env.CUSTOM_WASM_FILE_PATH || ""

  // Read the files as Buffers
  const factoryFileBuffer = fs.readFileSync(FACTORY_WASM_FILE_PATH)
  const splitterFileBuffer = fs.readFileSync(SPLITTER_WASM_FILE_PATH)
  // const customFileBuffer = fs.readFileSync(CUSTOM_WASM_FILE_PATH)

  // Convert the Buffer to Uint8Array
  const factoryWasmArray = new Uint8Array(factoryFileBuffer.buffer)
  const splitterWasmArray = new Uint8Array(splitterFileBuffer.buffer)
  // const customWasmArray = new Uint8Array(customFileBuffer.buffer)

  let factoryRes = await adminClient.upload(
    adminAccount.address,
    factoryWasmArray,
    "auto"
  )
  let splitterRes = await adminClient.upload(
    adminAccount.address,
    splitterWasmArray,
    "auto"
  )
  // let customRes = await adminClient.upload(
  //   adminAccount.address,
  //   customWasmArray,
  //   "auto"
  // )

  console.log("\n🟠 Factory Contract Code ID: ", factoryRes.codeId, "\n")
  console.log("\n🟠 Splitter Contract Code ID: ", splitterRes.codeId, "\n")
  // console.log("\n🟠 Custom Contract Code ID: ", customRes.codeId, "\n")
})()
