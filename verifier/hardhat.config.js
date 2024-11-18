require("@nomicfoundation/hardhat-ignition-ethers");
require("@nomicfoundation/hardhat-toolbox");
require("dotenv").config({ path: "../.env" });

module.exports = {
  defaultNetwork: "sepolia",
  solidity: {
    version: "0.8.28",
    settings: {
      optimizer: {
        enabled: true,
        runs: 1000,
      },
      evmVersion: "cancun",
    },
  },
  networks: {
    sepolia: {
      chainId: 11155111,
      url: `https://eth-sepolia.g.alchemy.com/v2/${process.env.ALCHEMY_API_KEY}`,
      accounts: [process.env.PRIV_KEY],
    }
  }
};
