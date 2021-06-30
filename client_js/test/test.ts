import {
  Keypair, // Account,
  Connection,
  PublicKey,
  SystemProgram,
  Transaction,
} from '@solana/web3.js';
import {AccountLayout, Token, TOKEN_PROGRAM_ID} from '@solana/spl-token';

// import {TokenSwap, CurveType, TOKEN_SWAP_PROGRAM_ID} from '../src';
// import {sendAndConfirmTransaction} from '../src/util/send-and-confirm-transaction';
import {newAccountWithLamports} from './util/new-account-with-lamports';
// import {sleep} from '../src/util/sleep';
import { url } from '../src/config';

let connection: Connection;
async function getConnection(): Promise<Connection> {
  if (connection) return connection;

  connection = new Connection(url, 'recent');
  const version = await connection.getVersion();

  console.log('Connection to cluster established:', url, version);
  return connection;
}

async function createProgram(): Promise<void> {
  const connection = await getConnection();
  const payer = await newAccountWithLamports(connection, 1000000000);
  const creator = await newAccountWithLamports(connection, 1000000000);

  const programKeypair = new Keypair();
  const [program_account_keypair, program_account_nonce] = 
    await PublicKey.findProgramAddress(
      [programKeypair.publicKey.toBuffer(), Buffer.from("account")],
      programKeypair.publicKey,
    );

  const tokenMint = await Token.createMint(
    connection,
    payer,
    creator.publicKey,
    null,
    6,
    programKeypair.publicKey
  );

  const creatorTokenAccount = await tokenMint.createAccount(creator.publicKey);

  const program = null; // TODO
}