use solana_client::client_error::ClientError;
use solana_client::rpc_client::RpcClient;
use solana_program::program_pack::Pack;
use solana_program::sysvar::Sysvar;
use solana_program_test::*;

mod testutil;

use chikin_airdrop_pool::config as program_config;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

//

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_get_id() {
    let program_id = Pubkey::from_str("ALaYfBMScNrJxKTfgpfFYDQSMYJHpzuxGq15TM2j6o8E").unwrap();
    let token_mint_id = Pubkey::from_str("8s9FCz99Wcr3dHpiauFRi6bLXzshXfcGTfgQE7UEopVx").unwrap();
    let pool_account_nonce = [1, 0, 1, 0];
    let claimer_wallet_id = Pubkey::from_str("DkmfiWSC4mnPvfMXZY2CkT4skvFkGr4u5DwRX2htRvJ2").unwrap();

    let (pool_account_id, _pool_account_bump_seed) = program_config::get_pool_account(&program_id, &token_mint_id, &pool_account_nonce);
    let (pool_token_account_id, _pool_token_account_bump_seed) = program_config::get_pool_token_account(&program_id, &pool_account_id);
    let (claimer_account_id, _claimer_account_bump_seed) = program_config::get_claimer_account(&program_id, &pool_account_id, &claimer_wallet_id);
    let claimer_token_account_id = program_config::get_claimer_token_account(&token_mint_id, &claimer_wallet_id);

    assert_eq!(pool_account_id, Pubkey::from_str("25sXXVsBY5Qx5QQ5w8563BmqibgkjwHBvKDBVFP52dCQ").unwrap());
    assert_eq!(pool_token_account_id, Pubkey::from_str("7NbJf1oXinHBYq3BF528xcUUmQ9786G8xZZFAB5jGe58").unwrap());
    assert_eq!(claimer_account_id, Pubkey::from_str("3NxLy8h8CwZYYt7K8ZnqhZehrirLPKuZdyvBD1vPhS1A").unwrap());
    assert_eq!(claimer_token_account_id, Pubkey::from_str("Esi6Z7reZt9NjZ2TeTFRXcTez1XA7764dE9bZoKCdjTb").unwrap());
}