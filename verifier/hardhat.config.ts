import { task } from "hardhat/config"
import { SignerWithAddress } from "@nomiclabs/hardhat-ethers/signers"
import { BigNumberish } from "ethers"
import "@nomicfoundation/hardhat-toolbox";
import "@nomicfoundation/hardhat-chai-matchers";

task("accounts", "Prints the list of accounts", async (args, hre): Promise<void> => {
  const accounts = await hre.ethers.getSigners()
  accounts.forEach((account: any): void => {
    console.log(account.address)
  })
})

task("balances", "Prints the list of ETH account balances", async (args, hre): Promise<void> => {
  const accounts = await hre.ethers.getSigners()
  for(const account of accounts){
    const balance: BigNumberish = await hre.ethers.provider.getBalance(
        account.address
    );
    console.log(`${account.address} has balance ${balance.toString()}`);
  }
})

export default {
  solidity: {
    compilers: [
      {
        version: "0.8.25"
      }
    ]
  },
  networks: {
    localnet: {
      url: 'http://localhost:8545',
      accounts: [
        'bcdf20249abf0ed6d944c0288fad489e33f66b3960d9e6229c1cd214ed3bbe31',
        '39725efee3fb28614de3bacaffe4cc4bd8c436257e2c8bb887c4b5c4be45e76d',
        '53321db7c1e331d93a11a41d16f004d7ff63972ec8ec7c25db329728ceeb1710',
        'ab63b23eb7941c1251757e24b3d2350d2bc05c3c388d06f8fe6feafefb1e8c70',
      ],
    },
    // mainnet config...
    // testnet config...
  },
};
