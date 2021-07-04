use std::convert::TryFrom;
use std::str::FromStr;
use std::thread::sleep;
use std::time::Duration;

use chikin_airdrop_pool::config as program_config;
use chikin_airdrop_pool::packable::Packable;
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
use spl_token::state::Account as SplTokenAccount;
use spl_token::state::AccountState as SplTokenAccountState;
use spl_token::state::Mint as SplTokenMint;

use client_rust::client;
use client_rust::command;
use client_rust::config::{Config, IdConfig};

mod testutil;

//

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_initialize_and_claim() {
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

    println!("test_initialize: fee_payer_balance1={}", config.get_fee_payer_balance());
    println!("test_initialize: create test token");
    let test_token = testutil::test_token::TestToken::create(&config);
    println!("test_initialize: test_token.mint_authority={}", test_token.mint_authority.pubkey());
    println!("test_initialize: test_token.mint={}", test_token.mint.pubkey());
    println!("test_initialize: fee_payer_balance2={}", config.get_fee_payer_balance());

    let pool_account_nonce = [1, 0, 1, 0];
    let reward_per_account = 500;
    let reward_per_referral = 100;
    let max_referral_depth = 2;
    let (
        pool_account_id,
        _,
    ) = program_config::get_pool_account(&config.id_config.program, &test_token.mint.pubkey(), &pool_account_nonce);
    let (
        pool_token_account_id,
        pool_token_account_nonce,
    ) = program_config::get_pool_token_account(&config.id_config.program, &pool_account_id);

    println!("test_initialize: id_config={:?}", config.id_config);
    println!("test_initialize: token_mint_id={}", test_token.mint.pubkey());
    println!("test_initialize: airdrop_pool_id={}", pool_account_id);
    println!("test_initialize: pool_token_account_id={}", pool_token_account_id);

    // Initialize pool
    println!("test_initialize: fee_payer_balance3={}", config.get_fee_payer_balance());
    println!("test_initialize: create_pool");
    command::initialize(&config,
                        test_token.mint.pubkey(),
                        pool_account_nonce,
                        reward_per_account,
                        reward_per_referral,
                        max_referral_depth)
        .unwrap();

    let airdrop_pool = config.rpc_client.get_account(&pool_account_id).unwrap();
    assert_ne!(airdrop_pool.lamports, 0);
    assert_eq!(airdrop_pool.owner, config.id_config.program);
    let airdrop_pool_data = AirdropPool::unpack(airdrop_pool.data()).unwrap();
    assert_eq!(airdrop_pool_data.is_initialized, 1);
    assert_eq!(airdrop_pool_data.token_account_id, pool_token_account_id);
    assert_eq!(airdrop_pool_data.account_id, pool_account_id);
    assert_eq!(airdrop_pool_data.pool_account_nonce, pool_account_nonce);

    let pool_token_account = config.rpc_client.get_account(&pool_token_account_id).unwrap();
    assert_ne!(pool_token_account.lamports, 0);
    assert_eq!(pool_token_account.owner, config.id_config.token_program);
    let pool_token_account_data = SplTokenAccount::unpack(pool_token_account.data()).unwrap();
    assert_eq!(pool_token_account_data.state, SplTokenAccountState::Initialized);
    assert_eq!(pool_token_account_data.owner, pool_account_id);
    assert_eq!(pool_token_account_data.mint, test_token.mint.pubkey());
    assert_eq!(pool_token_account_data.close_authority, COption::None);
    assert_eq!(pool_token_account_data.delegate, COption::None);

    // Mint some token to pool token account
    println!("test_initialize: fee_payer_balance4={}", config.get_fee_payer_balance());
    println!("test_initialize: mint some token to pool token account");
    test_token.mint(&config, 10000, &pool_token_account_id);

    //
    println!("test_initialize: fee_payer_balance5={}", config.get_fee_payer_balance());
    let test_claimer_1 = testutil::TestClaimer::create(&config, &test_token.mint.pubkey(), 10_000_000);
    println!("test_initialize: test_claimer_1.wallet={}", test_claimer_1.wallet.pubkey());
    println!("test_initialize: test_claimer_1.token_account={}", test_claimer_1.token_account);

    testutil::debug_token_account("CLUCK claimer_token_account before", &config, &test_claimer_1.token_account);
    testutil::debug_token_account("CLUCK pool_token_account before", &config, &pool_token_account_id);

    command::claim(&config, test_token.mint.pubkey(), pool_account_id, &test_claimer_1.wallet, None).unwrap();

    testutil::debug_token_account("CLUCK claimer_token_account after ", &config, &test_claimer_1.token_account);
    testutil::debug_token_account("CLUCK pool_token_account after", &config, &pool_token_account_id);
    println!("test_initialize: fee_payer_balance6={}", config.get_fee_payer_balance());
}