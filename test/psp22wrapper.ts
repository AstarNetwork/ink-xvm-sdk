import * as polkadotCryptoUtils from "@polkadot/util-crypto";
import {ethers} from "hardhat";
import {HardhatEthersSigner} from "@nomicfoundation/hardhat-ethers/src/signers";
import {ApiPromise, Keyring, WsProvider} from "@polkadot/api";
import {KeyringPair} from "@polkadot/keyring/types";
import {u8aToHex, u8aConcat} from "@polkadot/util";
import {claimEvmAddress, contractCall, deployContract, transferNative} from "./helper";
import {readFile} from "node:fs/promises";
import {CodePromise, ContractPromise} from "@polkadot/api-contract/promise";
import {expect} from "chai";
import BN from 'bn.js';
import {blake2AsU8a, decodeAddress} from "@polkadot/util-crypto";
import {WeightV2} from "@polkadot/types/interfaces";

describe("PSP22Wrapper Tests", function () {
    let api: ApiPromise;
    let erc20Contract: any
    let signer: HardhatEthersSigner;
    let alice: KeyringPair;
    let psp22_h160: any;
    let psp22Contract: ContractPromise;
    let gasLimit: WeightV2;

    before("Setup env", async function () {
        const wsProvider = new WsProvider("ws://127.0.0.1:9944");
        api = await ApiPromise.create({provider: wsProvider});

        gasLimit = api.registry.createType(
            'WeightV2',
            {
                refTime: 100_000_000_000,
                proofSize: 600_000,
            });


        const keyring = new Keyring({type: "sr25519", ss58Format: 5});
        alice = keyring.addFromUri("//Alice");
        console.log("ADDRESS ALICE SUB", alice.address.toString())

        signer = await ethers.getSigner("0xaaafB3972B05630fCceE866eC69CdADd9baC2771");
        console.log("ADDRESS ALICE EVM", signer.address.toString())

        //Transfer Native Token to Fund Alice EVM address
        const alice_h160 = u8aToHex(polkadotCryptoUtils.addressToEvm(alice.address, true));
        const alith32 = polkadotCryptoUtils.evmToAddress(
            signer.address, 5
        );
        await transferNative(api, alice_h160, alice)

        // Unify the two addresses
        const {chainId} = await ethers.provider.getNetwork();
        await claimEvmAddress(api, signer, chainId, alice)

        // Deploy ERC20 contract
        const erc20ContractFactory = await ethers.getContractFactory("Token");
        erc20Contract = await erc20ContractFactory.connect(signer).deploy();
        const erc20ContractAddress = await erc20Contract.getAddress()
        console.log("ERC20 Contract deployed to:", erc20ContractAddress);

        // Transfer Native token to active contract AccountId (because of Existential deposit)
/*        const erc20_account_id = polkadotCryptoUtils.evmToAddress(
            erc20ContractAddress , 5
        );
        await transferNative(api, erc20_account_id, alice)*/

        // Deploy PSP22 Wrapper Contract
        const compiledContractFile = await readFile("./ink/target/ink/xvm_sdk_psp22_wrapper/xvm_sdk_psp22_wrapper.contract");
        const compiledContract = JSON.parse(compiledContractFile.toString("utf8"));
        psp22Contract = await deployContract(api, alice, erc20ContractAddress, compiledContract);
        console.log("PSP22 Wrapper Contract deployed to:", psp22Contract.address.toString());

        // Get the H160 address of the PSP22 Wrapper Contract to approve spending of ERC20 tokens
        const data = u8aConcat('evm:', decodeAddress(psp22Contract.address, false))
        psp22_h160 = u8aToHex(blake2AsU8a(data, 256).subarray(0, 20))
    });

    it("Deposit works", async function () {
        // Arrange - Approve spending of ERC20 tokens
        await erc20Contract.connect(signer).approve(psp22_h160, "1000000000000000000000000", {
            gasLimit: 400000
        });

        // Act -  Deposit ERC20 tokens - PSP22 tokens gets minted
        await contractCall(api, psp22Contract, 'deposit', ['10000000000000000000000'], alice);

        // Assert - that PSP22 tokens were minted
        expect((await psp22Contract.query['psp22::balanceOf'](alice.address, {
            gasLimit,
            storageDepositLimit: null
        }, alice.address)).output?.toHuman()?.Ok.replace(/,/g, '')).to.equal(new BN('10000000000000000000000').toString());
    });
});