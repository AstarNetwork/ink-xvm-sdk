import {KeyringPair} from "@polkadot/keyring/types";
import {ApiPromise} from "@polkadot/api";
import {Abi, CodePromise, ContractPromise} from "@polkadot/api-contract";
import {WeightV2} from "@polkadot/types/interfaces";

export const DECIMALS = 1_000_000_000_000_000_000n;

export async function transferNative(api: ApiPromise, to: any, alice: KeyringPair) {
    const unsub = await api.tx.balances.transferKeepAlive(to, 1000n * DECIMALS)
        .signAndSend(alice, {nonce: -1}, ({status}) => {
            if (status.isFinalized) {
                unsub();
            }
        });
}

export async function claimEvmAddress(api: ApiPromise, signer: any, chainId: bigint, alice: KeyringPair) {
    const signature = await buildSignature(signer, alice.publicKey, api, chainId)
    const unsub = await api.tx.unifiedAccounts.claimEvmAddress(signer.address, signature)
        .signAndSend(alice, {nonce: -1}, ({status}) => {
            if (status.isFinalized) {
                unsub();
            }
        });
}

export async function deployContract(api: ApiPromise, deployer: KeyringPair, erc20ContractAddress: string, contractRaw: string) {
    const contractAbi = new Abi(contractRaw);
    const estimatedGas = api.registry.createType(
        'WeightV2',
        {
            refTime: 70_000_000_000,
            proofSize: 100_000,
        });

    const code = new CodePromise(api, contractAbi, contractAbi.info.source.wasm);
    const tx = code.tx.new({gasLimit: estimatedGas}, erc20ContractAddress)
    let finish = false;
    let promise: ContractPromise;
    const unsub = await tx.signAndSend(deployer, {nonce: -1}, ({contract, status,}) => {
        if (status.isFinalized) {
            let address = contract.address.toString();
            promise = new ContractPromise(api, contractAbi, address);
            finish = true;
            unsub();
        }
    });

    while (!finish) {
        await waitFor(1);
    }
    return promise;
}

async function buildSignature(signer: any, substrateAddress: any, api: ApiPromise, chainId: bigint) {
    return await signer.signTypedData({
        chainId,
        name: "Astar EVM Claim",
        version: "1",
        salt: await api.query.system.blockHash(0) // genisis hash
    }, {
        Claim: [
            {name: 'substrateAddress', type: 'bytes'}
        ],
    }, {
        substrateAddress
    })
}

export async function contractCall(api: ApiPromise, contract: ContractPromise, tx: any, params: any[], signer: any) {
    const gasLimit: WeightV2 = api.registry.createType(
        'WeightV2',
        {
            refTime: 100_000_000_000,
            proofSize: 600_000,
        });

    const {gasRequired, result} = await contract.query[tx](
        signer.address,
        {
            gasLimit,
            storageDepositLimit: null
        },
        '10000000000000000'
    )

    if (result.isErr) {
        let error = ''
        if (result.asErr.isModule) {
            const dispatchError = api.registry.findMetaError(result.asErr.asModule)
            error = dispatchError.docs.length ? dispatchError.docs.concat().toString() : dispatchError.name
        } else {
            error = result.asErr.toString()
        }

        console.error(error)
        return
    }

    const estimatedGas = api.registry.createType(
        'WeightV2',
        {
            refTime: gasRequired.refTime.toBn().muln(2),
            proofSize: gasRequired.proofSize.toBn().muln(2),
        }
    )

    let finish = false;
    const unsub = await contract.tx[tx](
        {
            gasLimit: estimatedGas,
            storageDepositLimit: null,
        },
        params
    )
        .signAndSend(signer, (res) => {
            if (res.status.isFinalized) {
                finish = true;
                unsub()
            }
        })

    while (!finish) {
        await waitFor(1);
    }
}

export async function waitFor(ms: any) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}