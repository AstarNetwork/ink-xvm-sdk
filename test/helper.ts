import {KeyringPair} from "@polkadot/keyring/types";
import { ApiPromise } from "@polkadot/api";

export const DECIMALS = 1_000_000_000_000_000_000n;

export async function transferNative(api: ApiPromise, to: any, alice: KeyringPair) {
    const unsub = await api.tx.balances.transferKeepAlive(to, 1000n * DECIMALS)
        .signAndSend(alice, {nonce: -1}, ({status}) => {
            if (status.isFinalized) {
                unsub();
            }
        });
}