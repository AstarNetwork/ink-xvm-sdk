import * as polkadotCryptoUtils from "@polkadot/util-crypto";
import {ethers } from "hardhat";
import {HardhatEthersSigner} from "@nomicfoundation/hardhat-ethers/src/signers";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import {expect} from "chai";
import { KeyringPair } from "@polkadot/keyring/types";
import {hexToU8a} from "@polkadot/util/hex/toU8a";
import {transferNative} from "./helper";
import {Wallet} from "ethers";
import { CodePromise } from '@polkadot/api-contract';
import { readFile } from "node:fs/promises";

describe("PSP22Wrapper Tests", function () {
let api: ApiPromise;
let erc20Contract: any
let signer: HardhatEthersSigner;
let alith32: any
let alice: KeyringPair;

before("Setup env", async function () {
    const wsProvider = new WsProvider("ws://127.0.0.1:9944");
    api = await ApiPromise.create({ provider: wsProvider });

    const keyring = new Keyring({ type: "ethereum", ss58Format: 5 });
    alice = keyring.addFromSeed(hexToU8a("0x9e963df48eb2aeb329ff7a03991ac20a93130e619130a8431ce02bbee2b0a4ea"));
    console.log("ADDRESS ALICE SUB", alice.address.toString())

    const provider = ethers.getDefaultProvider("https://127.0.0.1:9944");
    signer = await ethers.getSigner(alice.address);
    console.log("ADDRESS ALICE EVM", signer.address.toString())

    //Transfer Native Token to Fund address²
    const keyringSS58 = new Keyring({ type: "sr25519", ss58Format: 5 });
    const aliceSS58 = keyringSS58.addFromUri("//Alice");
    alith32 = polkadotCryptoUtils.evmToAddress(
        signer.address , 5
    );
    console.log("ADDRESS ALICE32", alith32.toString())
    await transferNative(api, alith32, aliceSS58)

    // Deploy ERC20
    erc20Contract = await ethers.getContractFactory("Token");
    erc20Contract =  await erc20Contract.connect(signer).deploy();
    const erc20ContractAddress = await erc20Contract.getAddress()
    console.log("ERC20 Contract deployed to:", erc20ContractAddress);

    const compiledContractFile = await readFile("./ink/target/ink/xvm_sdk_psp22_wrapper/xvm_sdk_psp22_wrapper.contract");
    const compiledContract = JSON.parse(compiledContractFile.toString("utf8"));
    const abi = compiledContract;
    const wasm = compiledContract.source?.wasm;

    const psp22Wrapper = new CodePromise(api, abi, wasm);
    const tx = psp22Wrapper.tx.new({}, erc20ContractAddress)
    let address;
    const unsub = await tx.signAndSend(alice, ({ contract, status }) => {
        if (status.isInBlock || status.isFinalized) {
            address = contract.address.toString();
            unsub();
        }
    });
    console.log(address)
});

it("Deposit works", async function () {
    await erc20Contract.connect(signer).approve(erc20Contract, "1000000000000000000000000", {
        gasLimit: 400000
    });
});


});