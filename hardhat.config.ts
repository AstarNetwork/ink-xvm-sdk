import dotenv from "dotenv";
dotenv.config();
import { HardhatUserConfig } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";

const config: HardhatUserConfig = {
  solidity: {
    compilers: [
      {
        version: "0.8.10",
        settings: {
          optimizer: {
            enabled: true,
            runs: 200,
          },
        },
      },
    ],
  },
  networks: {
    local: {
      url: "http://127.0.0.1:9944",
      chainId: 4369,
      accounts: ["0x01ab6e801c06e59ca97a14fc0a1978b27fa366fc87450e0b65459dd3515b7391"],
    },
    shibuya: {
      url: "https://shibuya.public.blastapi.io",
      chainId: 81,
      gas: 10000000, // tx gas limit
      accounts: [process.env.ACCOUNT_PRIVATE_KEY_EVM as string],
    }
  },
  paths: {
    sources: "./solidity",
  },
  mocha: {
    timeout: 100000000,
  },
};

export default config;
