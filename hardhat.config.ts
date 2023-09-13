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
      accounts: ["0x9e963df48eb2aeb329ff7a03991ac20a93130e619130a8431ce02bbee2b0a4ea"],
    }
  },
  paths: {
    sources: "./solidity",
  }
};

export default config;
