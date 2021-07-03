use std::convert::TryFrom;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

use chikin_airdrop_pool::config as program_config;
use chikin_airdrop_pool::state::{AirdropPool, AirdropClaimer};
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

mod testutil;

//

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_initialize() {
    let config = {
        let rpc_url = "http://localhost:8899";
        let rpc_client = RpcClient::new_with_commitment(rpc_url.to_string(),
                                                        CommitmentConfig::confirmed());
        let fee_payer = testutil::new_account_with_lamports(&rpc_client, 10_000_000);
        let program_id = "3K1Td3DmxWt2rxT1H4furqWJyZu3nuc7QQs6W5rtHY3P";

        Config {
            rpc_client,
            fee_payer: fee_payer.into(),
            dry_run: false,
            id_config: IdConfig {
                program: Pubkey::from_str(program_id).unwrap(),
                ..IdConfig::default()
            },
        }
    };

    let token_mint = Keypair::new();
    let (
        airdrop_pool_id,
        airdrop_pool_nonce,
    ) = program_config::get_pool_account(&config.id_config.program, &token_mint.pubkey());
    let (
        pool_token_account_id,
        pool_token_account_nonce,
    ) = program_config::get_pool_token_account(&config.id_config.program, &airdrop_pool_id);

    println!("test_initialize: id_config={:?}", config.id_config);
    println!("test_initialize: token_mint_id={}", token_mint.pubkey());
    println!("test_initialize: airdrop_pool_id={}", airdrop_pool_id);
    println!("test_initialize: pool_token_account_id={}", pool_token_account_id);

    // Initialize token mint
    println!("test_initialize: create_spl_token");
    testutil::create_spl_token(&config, &token_mint)
        .unwrap();

    // Initialize pool
    println!("test_initialize: create_pool");
    command::create(&config,
                    token_mint.pubkey()/*,
                    Some(airdrop_pool_id),
                    Some(pool_token_account_id)*/)
        .unwrap();

    let airdrop_pool = config.rpc_client.get_account(&airdrop_pool_id).unwrap();
    assert_ne!(airdrop_pool.lamports, 0);
    assert_eq!(airdrop_pool.owner, config.id_config.program);
    let airdrop_pool_data = AirdropPool::unpack(airdrop_pool.data()).unwrap();
    assert_eq!(airdrop_pool_data.is_initialized, true);
    assert_eq!(airdrop_pool_data.token_account_id, pool_token_account_id);
    assert_eq!(airdrop_pool_data.account_id, airdrop_pool_id);
    assert_eq!(airdrop_pool_data.account_nonce, airdrop_pool_nonce);

    let pool_token_account = config.rpc_client.get_account(&pool_token_account_id).unwrap();
    assert_ne!(pool_token_account.lamports, 0);
    assert_eq!(pool_token_account.owner, config.id_config.token_program);
    let pool_token_account_data = SplTokenAccount::unpack(pool_token_account.data()).unwrap();
    assert_eq!(pool_token_account_data.state, SplTokenAccountState::Initialized);
    assert_eq!(pool_token_account_data.owner, airdrop_pool_id);
    assert_eq!(pool_token_account_data.mint, token_mint.pubkey());
    assert_eq!(pool_token_account_data.close_authority, COption::None);
    assert_eq!(pool_token_account_data.delegate, COption::None);

    let test_wallet_keypair = testutil::TestWallet::new(&config.rpc_client, 10_000_000);
    let test_wallet_token_account_id = test_wallet_keypair.create_token_account(&config,
                                                                                spl_token::id(),
                                                                                token_mint.pubkey());

    testutil::debug_token_account("CLUCK wallet_before", &config, &test_wallet_token_account_id);

    command::claim(&config, token_mint.pubkey(), test_wallet_token_account_id, None).unwrap();

    testutil::debug_token_account("CLUCK wallet_after ", &config, &test_wallet_token_account_id);
}