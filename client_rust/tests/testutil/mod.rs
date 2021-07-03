use std::convert::TryFrom;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

use chikin_airdrop_pool::config as program_config;
use chikin_airdrop_pool::state::{AirdropClaimer, AirdropPool};
use solana_client::client_error::ClientError;
use solana_client::rpc_client::RpcClient;
use solana_program::program_pack::Pack;
use solana_program::sysvar::Sysvar;
use solana_program_test::*;
use solana_sdk::account::ReadableAccount;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::program_error::ProgramError;
use solana_sdk::program_option::COption;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::rent::Rent;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use spl_associated_token_account;
use spl_token::state::Account as SplTokenAccount;
use spl_token::state::AccountState as SplTokenAccountState;
use spl_token::state::Mint as SplTokenMint;

use client_rust::client;
use client_rust::command;
use client_rust::config::{Config, IdConfig};

pub mod test_token;

// TestUser

pub struct TestClaimer {
    pub wallet: Keypair,
    pub token_account: Pubkey,
}

impl TestClaimer {
    pub fn create(config: &Config, token_mint: &Pubkey, lamports: u64) -> TestClaimer {
        let wallet = Keypair::new();

        // Request airdrop
        let retry_count = 30;
        for i in 0..retry_count {
            config.rpc_client.request_airdrop(&wallet.pubkey(), lamports).unwrap();
            let balance = config.rpc_client.get_balance(&wallet.pubkey()).unwrap();
            if lamports == balance { break; }
            sleep(Duration::from_millis(500));
        }

        // Create token account
        let mut transaction = Transaction::new_with_payer(
            &vec![
                spl_associated_token_account::create_associated_token_account(
                    &wallet.pubkey(),
                    &wallet.pubkey(),
                    token_mint),
            ],
            Some(&wallet.pubkey()),
        );
        let mut signers = vec![&wallet];
        let (recent_blockhash, _) = config.rpc_client.get_recent_blockhash().unwrap();
        config.check_fee_payer_balance(1).unwrap(); // TODO
        signers.sort_by_key(|e| e.pubkey());
        signers.dedup();
        transaction.sign(&signers, recent_blockhash);
        config.send_transaction(transaction).unwrap();
        let token_account = spl_associated_token_account::get_associated_token_address(&wallet.pubkey(), &token_mint);

        TestClaimer { wallet, token_account }
    }
}

// Utils

pub fn new_account_with_lamports(rpc_client: &RpcClient, lamports: u64) -> Keypair {
    let result = Keypair::new();

    let retry_count = 30;
    for i in 0..retry_count {
        rpc_client.request_airdrop(&result.pubkey(), lamports).unwrap();
        let balance = rpc_client.get_balance(&result.pubkey()).unwrap();
        if lamports == balance { break; }
        sleep(Duration::from_millis(500));
    }

    let balance = rpc_client.get_balance(&result.pubkey()).unwrap();
    assert_eq!(balance, lamports);
    result
}

pub fn debug_pool_account(tag: &str, config: &Config, pubkey: &Pubkey) {
    let program_account = client::get_airdrop_pool(&config.rpc_client, pubkey).unwrap();
    println!("{} : {:?}", tag, program_account);
}

pub fn debug_token_account(tag: &str, config: &Config, pubkey: &Pubkey) {
    let account = config.rpc_client.get_account(pubkey).unwrap();
    let account = spl_token::state::Account::unpack(account.data()).unwrap();
    println!("{} : {:?}", tag, account);
}

pub fn get_token_account(config: &Config, pubkey: &Pubkey) -> SplTokenAccount {
    let account = config.rpc_client.get_account(pubkey).unwrap();
    SplTokenAccount::unpack(account.data()).unwrap()
}