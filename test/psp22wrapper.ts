import * as polkadotCryptoUtils from "@polkadot/util-crypto";
import {ethers } from "hardhat";
import {HardhatEthersSigner} from "@nomicfoundation/hardhat-ethers/src/signers";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import {expect} from "chai";
import { KeyringPair } from "@polkadot/keyring/types";
import {hexToU8a, u8aToHex} from "@polkadot/util";
import {transferNative} from "./helper";
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

    const keyring = new Keyring({ type: "sr25519", ss58Format: 5 });
    alice = keyring.addFromUri("//Alice");
    console.log("ADDRESS ALICE SUB", alice.address.toString())

    signer = await ethers.getSigner("0xaaafB3972B05630fCceE866eC69CdADd9baC2771");
    console.log("ADDRESS ALICE EVM", signer.address.toString())

    //Transfer Native Token to Fund address
    const alice_h160 = u8aToHex(polkadotCryptoUtils.addressToEvm(alice.address, true));
    alith32 = polkadotCryptoUtils.evmToAddress(
        signer.address , 5
    );
    console.log("ADDRESS ALICE32", alith32.toString())
    await transferNative(api, alice_h160, alice)

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

    const gasLimit = 100000n * 1000000n;
    const tx = psp22Wrapper.tx.new({gasLimit, storageDepositLimit: null}, erc20ContractAddress)
    let address;
    const unsub = await tx.signAndSend(alice, {nonce: -1}, ({ contract, status }) => {
        if (status.isInBlock || status.isFinalized) {
            address = contract.address.toString();
            console.log(address)
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