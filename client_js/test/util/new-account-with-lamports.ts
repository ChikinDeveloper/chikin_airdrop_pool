// @flow

import { Keypair, Connection } from '@solana/web3.js';

import { sleep } from './sleep';

export async function newAccountWithLamports(
    connection: Connection,
    lamports: number = 1000000,
): Promise<Keypair> {
    const keypair = new Keypair();

    let retries = 30;
    await connection.requestAirdrop(keypair.publicKey, lamports);
    for (; ;) {
        await sleep(500);
        if (lamports == (await connection.getBalance(keypair.publicKey))) {
            return keypair;
        }
        if (--retries <= 0) {
            break;
        }
    }

    throw new Error(`Airdrop of ${lamports} failed`);
}