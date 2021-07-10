use std::str::FromStr;

use chikin_airdrop_pool::config as program_config;
use solana_client::rpc_client::RpcClient;
use solana_program_test::*;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

use client_rust::config::{Config, IdConfig};
use client_rust::command;
use solana_program::program_option::COption;
use chikin_airdrop_pool::state::AirdropPool;
use chikin_airdrop_pool::packable::Packable;
use solana_sdk::account::ReadableAccount;
use spl_token::state::Account as SplTokenAccount;
use spl_token::state::AccountState as SplTokenAccountState;
use spl_token::state::Mint as SplTokenMint;
use solana_program::program_pack::Pack;

mod testutil;

//

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn release_initialize() {
    // let config = {
    //     let rpc_url = "https://api.mainnet-beta.solana.com";
    //     let rpc_client = RpcClient::new_with_commitment(rpc_url.to_string(),
    //                                                     CommitmentConfig::confirmed());
    //
    //     let fee_payer_bytes = [VERY SENSITIVE INFO];
    //     let fee_payer = Keypair::from_bytes(&fee_payer_bytes).unwrap();
    //
    //     let program_id = "ALaYfBMScNrJxKTfgpfFYDQSMYJHpzuxGq15TM2j6o8E";
    //
    //     Config {
    //         rpc_client,
    //         fee_payer: fee_payer.into(),
    //         dry_run: false,
    //         id_config: IdConfig {
    //             program: Pubkey::from_str(program_id).unwrap(),
    //             ..IdConfig::default()
    //         },
    //     }
    // };
    //
    // println!("release_initialize: id_config={:?}", config.id_config);
    // println!("release_initialize: fee_payer_id={}", config.fee_payer.pubkey());
    // println!("release_initialize: fee_payer_balance1={}", config.get_fee_payer_balance());

    let token_mint_id = Pubkey::from_str("8s9FCz99Wcr3dHpiauFRi6bLXzshXfcGTfgQE7UEopVx").unwrap();
    let token_decimals = 6;
    let pool_account_nonce = [1, 0, 1, 0];
    let reward_per_account: u64 = 1000 * 10_u32.pow(token_decimals) as u64;
    let reward_per_referral: u64 = 100 * 10_u32.pow(token_decimals) as u64;
    let max_referral_depth = 2;

    println!("release_initialize: token_mint_id={}", token_mint_id);
    println!("release_initialize: token_decimals={}", token_decimals);
    println!("release_initialize: pool_account_nonce={:?}", pool_account_nonce);
    println!("release_initialize: reward_per_account={:?}", reward_per_account);
    println!("release_initialize: reward_per_referral={:?}", reward_per_referral);
    println!("release_initialize: max_referral_depth={:?}", max_referral_depth);

    let payer_id = Pubkey::from_str("DkmfiWSC4mnPvfMXZY2CkT4skvFkGr4u5DwRX2htRvJ2").unwrap();
    let payer_token_account_id = program_config::get_claimer_token_account(&token_mint_id, &payer_id);
    println!("release_initialize: payer_id={:?}", payer_id);
    println!("release_initialize: payer_token_account_id={:?}", payer_token_account_id);

    // let (
    //     pool_account_id,
    //     pool_account_bump_seed,
    // ) = program_config::get_pool_account(&config.id_config.program, &token_mint_id, &pool_account_nonce);
    // let (
    //     pool_token_account_id,
    //     pool_token_account_bump_seed,
    // ) = program_config::get_pool_token_account(&config.id_config.program, &pool_account_id);
    //
    // println!("release_initialize: airdrop_pool_id={}", pool_account_id);
    // println!("release_initialize: pool_account_bump_seed={}", pool_account_bump_seed);
    // println!("release_initialize: pool_token_account_id={}", pool_token_account_id);
    // println!("release_initialize: pool_token_account_bump_seed={}", pool_token_account_bump_seed);
    //
    // // Initialize pool
    //
    // println!("release_initialize: fee_payer_balance3={}", config.get_fee_payer_balance());
    // println!("release_initialize: create_pool");
    //
    // // command::initialize(&config,
    // //                     token_mint_id,
    // //                     pool_account_nonce,
    // //                     reward_per_account,
    // //                     reward_per_referral,
    // //                     max_referral_depth)
    // //     .unwrap();
    //
    // let airdrop_pool = config.rpc_client.get_account(&pool_account_id).unwrap();
    // assert_ne!(airdrop_pool.lamports, 0);
    // assert_eq!(airdrop_pool.owner, config.id_config.program);
    // let airdrop_pool_data = AirdropPool::unpack(airdrop_pool.data()).unwrap();
    // assert_eq!(airdrop_pool_data.token_mint_id, token_mint_id);
    // assert_eq!(airdrop_pool_data.account_nonce, pool_account_nonce);
    // assert_eq!(airdrop_pool_data.reward_per_account, reward_per_account);
    // assert_eq!(airdrop_pool_data.reward_per_referral, reward_per_referral);
    //
    // let pool_token_account = config.rpc_client.get_account(&pool_token_account_id).unwrap();
    // assert_ne!(pool_token_account.lamports, 0);
    // assert_eq!(pool_token_account.owner, config.id_config.token_program);
    // let pool_token_account_data = SplTokenAccount::unpack(pool_token_account.data()).unwrap();
    // assert_eq!(pool_token_account_data.state, SplTokenAccountState::Initialized);
    // assert_eq!(pool_token_account_data.owner, pool_account_id);
    // assert_eq!(pool_token_account_data.mint, token_mint_id);
    // assert_eq!(pool_token_account_data.close_authority, COption::None);
    // assert_eq!(pool_token_account_data.delegate, COption::None);
}