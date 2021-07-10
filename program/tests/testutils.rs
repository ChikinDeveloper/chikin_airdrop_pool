use solana_program_test::*;
use solana_sdk::account::Account;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use spl_token;
use spl_token::state::Account as SplTokenAccount;

use chikin_airdrop_pool::config;
use chikin_airdrop_pool::state::AirdropClaimer;
use chikin_airdrop_pool::state::AirdropPool;
use chikin_airdrop_pool::packable::Packable;
use solana_sdk::signature::{Keypair, Signer};

pub struct ProgramInfo {
    pub pool_account_id: Pubkey,
    pub pool_token_account_id: Pubkey,
}

impl ProgramInfo {
    pub fn create(program_test: &mut ProgramTest,
                  program_id: &Pubkey,
                  token_program_id: Pubkey,
                  token_mint_id: Pubkey,
                  pool_account_nonce: [u8; 4],
                  reward_per_account: u64,
                  reward_per_referral: u64,
                  max_referral_depth: u8,
    ) -> ProgramInfo {
        let (account_id, _) = config::get_pool_account(&program_id, &token_mint_id, &pool_account_nonce);
        let token_account_id = config::get_pool_token_account(&program_id, &account_id).0;

        let account_state = AirdropPool {
            token_program_id,
            token_mint_id,
            account_nonce: pool_account_nonce,
            reward_per_account,
            reward_per_referral,
            max_referral_depth,
        };

        let token_account_state = SplTokenAccount {
            mint: token_mint_id,
            amount: 10 * reward_per_account,
            state: spl_token::state::AccountState::Initialized,
            owner: account_id.clone(),
            ..SplTokenAccount::default()
        };

        let data_packed: Vec<u8> = account_state.pack();
        program_test.add_account(
            account_id,
            Account {
                lamports: 5,
                data: data_packed,
                owner: program_id.clone(),
                ..Account::default()
            },
        );

        let mut data_packed = vec![0; SplTokenAccount::LEN];
        token_account_state.pack_into_slice(&mut data_packed);
        program_test.add_account(
            token_account_id,
            Account {
                lamports: 5,
                data: data_packed,
                owner: spl_token::id(),
                ..Account::default()
            },
        );

        ProgramInfo {
            pool_account_id: account_id,
            pool_token_account_id: token_account_id,
        }
    }
}


pub struct UserInfo {
    pub wallet: Keypair,
    pub account: Pubkey,
    pub token_account: Pubkey,
}

impl UserInfo {
    pub fn create(program_test: &mut ProgramTest,
                  program_id: Pubkey,
                  token_mint: Pubkey,
                  pool_id: Pubkey) -> UserInfo {
        let wallet_keypair = Keypair::new();
        program_test.add_account(
            wallet_keypair.pubkey(),
            Account {
                lamports: 10_000_000,
                data: vec![],
                ..Account::default()
            },
        );

        let token_account_id = {
            let id = config::get_claimer_token_account(&token_mint, &wallet_keypair.pubkey());
            let data = SplTokenAccount {
                mint: token_mint,
                amount: 0,
                state: spl_token::state::AccountState::Initialized,
                owner: wallet_keypair.pubkey(),
                ..SplTokenAccount::default()
            };
            assert!(data.delegate.is_none());
            let mut data_packed = vec![0; spl_token::state::Account::LEN];
            data.pack_into_slice(&mut data_packed);
            program_test.add_account(
                id,
                Account {
                    lamports: 5,
                    data: data_packed,
                    owner: spl_token::id(),
                    ..Account::default()
                },
            );
            id
        };

        let account_id = {
            let id = config::get_claimer_account(&program_id, &pool_id, &wallet_keypair.pubkey()).0;
            // let data = AirdropClaimer::default();
            // let data_packed = data.pack();
            // program_test.add_account(
            //     id,
            //     Account {
            //         lamports: 5,
            //         data: data_packed,
            //         owner: program_id,
            //         ..Account::default()
            //     },
            // );
            id
        };

        UserInfo { wallet: wallet_keypair, account: account_id, token_account: token_account_id }
    }

    pub async fn debug(&self, tag: &str, banks_client: &mut BanksClient) {
        let account = banks_client
            .get_account(self.account)
            .await
            .expect("user.account")
            .expect("user.account not found");

        let token_account = banks_client
            .get_account(self.token_account)
            .await
            .expect("user.account")
            .expect("user.account not found");

        let account_state: AirdropClaimer = AirdropClaimer::unpack(&account.data).unwrap();
        let token_account_state = SplTokenAccount::unpack(&token_account.data).unwrap();

        println!("debug_user({}) : account.key={}, account.owner={}", tag, self.account, account.owner);
        println!("debug_user({}) : token_account.key={}, token_account.owner={}", tag, self.token_account, token_account.owner);
        println!("debug_user({}) : account_state={:?}", tag, account_state);
        println!("debug_user({}) : token_account_state={:?}", tag, token_account_state);
    }
}

