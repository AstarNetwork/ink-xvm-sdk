import * as polkadotCryptoUtils from "@polkadot/util-crypto";
import {blake2AsU8a, decodeAddress} from "@polkadot/util-crypto";
import {ethers} from "hardhat";
import {HardhatEthersSigner} from "@nomicfoundation/hardhat-ethers/src/signers";
import {ApiPromise, Keyring, WsProvider} from "@polkadot/api";
import {KeyringPair} from "@polkadot/keyring/types";
import {u8aConcat, u8aToHex} from "@polkadot/util";
import {claimEvmAddress, contractCall, DECIMALS, deployContract, transferNative} from "./helper";
import {readFile} from "node:fs/promises";
import {ContractPromise} from "@polkadot/api-contract/promise";
import {expect} from "chai";
import BN from 'bn.js';
import {H160, WeightV2} from "@polkadot/types/interfaces";

describe("PSP22Wrapper Tests", function () {
    let api: ApiPromise;
    let erc20Contract: any
    let signer: HardhatEthersSigner;
    let alice: KeyringPair;
    let bob: KeyringPair;
    let psp22_h160: any;
    let psp22Contract: ContractPromise;
    let gasLimit: WeightV2;
    let wsProvider: any;
    let bobH160: any;

    beforeEach("Setup env", async function () {
        const keyring = new Keyring({type: "sr25519", ss58Format: 5});

        const {chainId} = await ethers.provider.getNetwork();
        if (chainId == 4369) {
            console.log("Running on local")
            wsProvider = new WsProvider("ws://127.0.0.1:9944");
            alice = keyring.addFromUri("//Alice");
            bob = keyring.addFromUri("//Bob");
        } else if (chainId == 81) {
            console.log("Running on Shibuya")
            wsProvider = new WsProvider("wss://rpc.shibuya.astar.network");
            alice = keyring.addFromUri(process.env.SUBSTRATE_MNEMO as string);
            bob = keyring.addFromUri("//Bob");
        }

        api = await ApiPromise.create({provider: wsProvider});

        gasLimit = api.registry.createType(
            'WeightV2',
            {
                refTime: 100_000_000_000,
                proofSize: 600_000,
            });

        // Fund Bob Substrate Address
        const {data: balance} = await api.query.system.account(bob.address);
        if (balance.free.toBigInt() < 5n * DECIMALS) {
            console.log("Funding Bob SS58:", bob.address)
            await transferNative(api, bob.address, alice)
        }

        // Get Bob EVM Address
        // If it has been unified, use the unified address (it should have been funded)
        // if not get the default address and fund it if needed
        const bobUnifiedAddress = await api.query.unifiedAccounts.nativeToEvm<H160>(bob.address)
        if (bobUnifiedAddress.toString() !== '') {
            bobH160 = bobUnifiedAddress.toString()
        } else {
            const addr = u8aConcat('evm:', decodeAddress(bob.address, false))
            bobH160 = u8aToHex(blake2AsU8a(addr, 256).subarray(0, 20))
            const bob_account_id = polkadotCryptoUtils.evmToAddress(
                bobH160, 5
            );
            const {data: balanceEvm} = await api.query.system.account(bob_account_id);
            if (balanceEvm.free.toBigInt() < 5n * DECIMALS) {
                console.log("Funding Bob EVM:", bob_account_id)
                await transferNative(api, bob_account_id, alice)
            }
        }

        let signers = await ethers.getSigners();
        signer = signers[0]

        console.log("ADDRESS BOB EVM", bobH160)
        console.log("ADDRESS BOB SUBSTRATE", bob.address)
        console.log("ADDRESS ALICE EVM", signer.address.toString())
        console.log("ADDRESS ALICE SUB", alice.address.toString())

        // Unify Alice EVM and Alice Substrate
        const aliceUnifiedAddress = await api.query.unifiedAccounts.nativeToEvm(alice.address.toString())
        if (aliceUnifiedAddress.toString() === '') {
            await claimEvmAddress(api, signer, chainId, alice)
        }

        // Deploy ERC20 Contract
        erc20Contract = await ethers.deployContract("TokenTKN");
        await erc20Contract.waitForDeployment();
        const erc20ContractAddress = await erc20Contract.getAddress()
        console.log("ERC20 Contract deployed to:", erc20ContractAddress);

        // Deploy PSP22 Wrapper Contract
        const compiledContractFile = await readFile("./ink/target/ink/xvm_sdk_psp22_wrapper/xvm_sdk_psp22_wrapper.contract");
        const compiledContract = JSON.parse(compiledContractFile.toString("utf8"));
        psp22Contract = await deployContract(api, alice, erc20ContractAddress, compiledContract);
        console.log("PSP22 Wrapper Contract deployed to:", psp22Contract.address.toString());

        // Get the H160 address of the PSP22 Wrapper Contract to approve spending of ERC20 tokens
        const data = u8aConcat('evm:', decodeAddress(psp22Contract.address, false))
        psp22_h160 = u8aToHex(blake2AsU8a(data, 256).subarray(0, 20))

        console.log("PSP22 H160", psp22_h160)
    });

    it("Deposit works", async function () {
        // Assert - Alice balance of ERC20
        const aliceBalance = await erc20Contract.connect(signer).balanceOf(signer.address);
        expect(aliceBalance).to.equal('1000000000000000000000');

        // Arrange - Approve spending of ERC20 tokens
        const transaction = await erc20Contract.connect(signer).approve(psp22_h160, "1000000000000000000000000", {
            gasLimit: 400000
        });
        await transaction.wait()

        // Act -  Deposit ERC20 tokens - PSP22 tokens gets minted
        await contractCall(api, psp22Contract, 'deposit', ['100000000000000000000'], alice);

        // Assert - that PSP22 tokens were minted
        expect((await psp22Contract.query['psp22::balanceOf'](alice.address, {
            gasLimit,
            storageDepositLimit: null
        }, alice.address)).output?.toHuman()?.Ok.replace(/,/g, '')).to.equal(new BN('100000000000000000000').toString());

        // Assert - Alice balance is 0
        expect(await erc20Contract.balanceOf(signer.address)).to.equal('900000000000000000000');

        // Assert - PSP222 contract has ERC20 tokens
        expect(await erc20Contract.balanceOf(psp22_h160)).to.equal('100000000000000000000');
    });

    it("Deposit Transfer Withdraw works", async function () {
        // Arrange - Approve spending of ERC20 tokens
        const transaction = await erc20Contract.connect(signer).approve(psp22_h160, "1000000000000000000000000", {
            gasLimit: 400000
        });
        await transaction.wait()

        // Act -  Deposit ERC20 tokens - PSP22 tokens gets minted
        await contractCall(api, psp22Contract, 'deposit', ['100000000000000000000'], alice);

        // Assert - that PSP22 tokens were minted
        expect((await psp22Contract.query['psp22::balanceOf'](alice.address, {
            gasLimit,
            storageDepositLimit: null
        }, alice.address)).output?.toHuman()?.Ok.replace(/,/g, '')).to.equal(new BN('100000000000000000000').toString());

        // Act -  Transfer PSP22 tokens to Bob
        await contractCall(api, psp22Contract, 'psp22::transfer', [bob.address, '100000000000000000000', ''], alice);

        // Assert - Bob gets the PSP22 tokens
        expect((await psp22Contract.query['psp22::balanceOf'](alice.address, {
            gasLimit,
            storageDepositLimit: null
        }, bob.address)).output?.toHuman()?.Ok.replace(/,/g, '')).to.equal(new BN('100000000000000000000').toString());

        // Assert - PSP222 contract has ERC20 tokens
        expect(await erc20Contract.balanceOf(psp22_h160)).to.equal('100000000000000000000');

        // Act - Bob withdraws ERC20 tokens
        await contractCall(api, psp22Contract, 'withdraw', ['100000000000000000000'], bob);

        // Assert - Bob spends all PSP22 tokens
        expect((await psp22Contract.query['psp22::balanceOf'](alice.address, {
            gasLimit,
            storageDepositLimit: null
        }, bob.address)).output?.toHuman()?.Ok.replace(/,/g, '')).to.equal(new BN('0').toString());

        expect(await erc20Contract.balanceOf(bobH160)).to.equal('100000000000000000000');
    });
});