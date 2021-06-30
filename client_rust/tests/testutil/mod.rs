use std::convert::TryFrom;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

use chikin_airdrop::config as program_config;
use chikin_airdrop::state::{ChikinAirdropPool, ChikinAirdropUser};
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
use spl_token::state::Account as SplTokenAccount;
use spl_token::state::AccountState as SplTokenAccountState;
use spl_token::state::Mint as SplTokenMint;

use client_rust::client;
use client_rust::command;
use client_rust::config::{Config, IdConfig};

// TestUser

pub struct TestAccount {
    pub keypair: Keypair,
}

impl TestAccount {
    pub fn new(rpc_client: &RpcClient, lamports: u64) -> TestAccount {
        let keypair = Keypair::new();

        let retry_count = 30;
        for i in 0..retry_count {
            rpc_client.request_airdrop(&keypair.pubkey(), lamports).unwrap();
            let balance = rpc_client.get_balance(&keypair.pubkey()).unwrap();
            if lamports == balance { break; }
            sleep(Duration::from_millis(500));
        }

        let balance = rpc_client.get_balance(&keypair.pubkey()).unwrap();
        assert_eq!(balance, lamports);
        TestAccount { keypair }
    }

    pub fn create_token_account(&self, token_program: Pubkey, token_mint: Pubkey) -> Pubkey {
        println!("create_spl_token_account(token_mint={})", keypair.pubkey());

        let token_account_id = Pubkey::new_unique();
        let minimum_balance_for_rent_exemption = config.rpc_client
            .get_minimum_balance_for_rent_exemption(SplTokenAccount::LEN).unwrap();

        let instructions = vec![
            solana_sdk::system_instruction::create_account(
                &self.keypair.pubkey(),
                &token_account_id,
                minimum_balance_for_rent_exemption,
                SplTokenAccount::LEN as u64,
                &token_program),
            spl_token::instruction::initialize_account(
                &token_program,
                &token_account_id,
                &token_mint,
                &self.keypair.pubkey()).unwrap(),
        ];

        let mut transaction = Transaction::new_with_payer(
            &instructions,
            Some(&self.keypair.pubkey()),
        );

        let mut signers = vec![
            self.keypair.pubkey(),
            token_account_id,
        ];
        let (recent_blockhash, _) = config.rpc_client.get_recent_blockhash().unwrap();
        config.check_fee_payer_balance(1).unwrap(); // TODO
        signers.sort_by_key(|e| e.pubkey());
        signers.dedup();
        transaction.sign(&signers, recent_blockhash);
        config.send_transaction(transaction).unwrap();

        return token_account_id
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

pub fn create_spl_token(config: &Config,
                        token_mint: &Keypair) -> Result<(), ProgramError> {
    println!("create_spl_token(token_mint={})", token_mint.pubkey());

    let minimum_balance_for_rent_exemption = config.rpc_client
        .get_minimum_balance_for_rent_exemption(SplTokenMint::LEN).unwrap();
    let freeze_authority_pubkey = None;

    let instructions = vec![
        solana_sdk::system_instruction::create_account(
            &config.fee_payer.pubkey(),
            &token_mint.pubkey(),
            minimum_balance_for_rent_exemption,
            SplTokenMint::LEN as u64,
            &spl_token::id()),
        spl_token::instruction::initialize_mint(
            &spl_token::id(),
            &token_mint.pubkey(),
            &config.id_config.program,
            freeze_authority_pubkey,
            6).unwrap(),
    ];

    let mut transaction = Transaction::new_with_payer(
        &instructions,
        Some(&config.fee_payer.pubkey()),
    );

    let mut signers = vec![
        config.fee_payer.as_ref(),
        token_mint,
    ];
    let (recent_blockhash, _) = config.rpc_client.get_recent_blockhash().unwrap();
    config.check_fee_payer_balance(1).unwrap(); // TODO
    signers.sort_by_key(|e| e.pubkey());
    signers.dedup();
    transaction.sign(&signers, recent_blockhash);
    config.send_transaction(transaction).unwrap();

    Ok(())
}

pub fn debug_program_account(tag: &str, config: &Config) {
    let (program_account_id, _) = program_config::get_program_account_id(&config.id_config.program);
    let program_account = client::get_airdrop_pool(&config.rpc_client, &program_account_id).unwrap();
    println!("{} : {:?}", tag, program_account);
}

pub fn debug_token_account(tag: &str, config: &Config, pubkey: &Pubkey) {
    let account = config.rpc_client.get_account(pubkey).unwrap();
    let account = spl_token::state::Account::unpack(account.data()).unwrap();
    println!("{} : {:?}", tag, account);
}

pub fn get_airdrop_pool(config: &Config) -> ChikinAirdropPool {
    let (program_account_id, _) = program_config::get_program_account_id(&config.id_config.program);
    client::get_airdrop_pool(&config.rpc_client, &program_account_id).unwrap()
}

pub fn get_token_account(config: &Config, pubkey: &Pubkey) -> SplTokenAccount {
    let account = config.rpc_client.get_account(pubkey).unwrap();
    SplTokenAccount::unpack(account.data()).unwrap()
}