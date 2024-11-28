import {
  Contract,
  ContractFactory
} from "ethers"
import { ethers } from "hardhat"
import { Verifier } from "../typechain-types"

const main = async (): Promise<any> => {
  const Verifier: ContractFactory = await ethers.getContractFactory("Verifier")
  const verifier: Contract = await Verifier.deploy() as Contract

  const address = await verifier.getAddress()
  console.log(`Verifier deployed to: ${address}`)
}

main()
  .then(() => process.exit(0))
  .catch(error => {
    console.error(error)
    process.exit(1)
  })
