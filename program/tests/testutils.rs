use {
    solana_program_test::*,
    solana_sdk::{
        account::Account,
        program_pack::Pack,
        pubkey::Pubkey,
    },
};
use spl_token;
use spl_token::state::Account as SplTokenAccount;
use chikin_airdrop_pool::config;
use chikin_airdrop_pool::state::AirdropPool;
use chikin_airdrop_pool::state::AirdropClaimer;

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
                  max_referral_depth: u32,
    ) -> ProgramInfo {
        let (account_id, _) = config::get_pool_account(&program_id, &token_mint_id, &pool_account_nonce);
        let token_account_id = config::get_pool_token_account(&program_id, &account_id).0;

        let account_state = AirdropPool {
            token_program_id,
            token_mint_id,
            account_id,
            token_account_id,
            pool_account_nonce,
            reward_per_account,
            reward_per_referral,
            max_referral_depth,
            is_initialized: true,
        };

        let token_account_state = SplTokenAccount {
            mint: token_mint_id,
            amount: 10 * reward_per_account,
            state: spl_token::state::AccountState::Initialized,
            owner: account_id.clone(),
            ..SplTokenAccount::default()
        };

        let mut data_packed = vec![0; AirdropPool::LEN];
        account_state.pack_into_slice(&mut data_packed);
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


#[derive(Clone)]
pub struct UserInfo {
    pub wallet: Pubkey,
    pub account: Pubkey,
    pub token_account: Pubkey,
}

impl UserInfo {
    pub fn create(program_test: &mut ProgramTest,
                  program_id: Pubkey,
                  token_mint: Pubkey,
                  pool_id: Pubkey) -> UserInfo {
        let wallet_id = Pubkey::new_unique();
        let token_account_id = {
            let id = config::get_user_token_account(&token_mint, &wallet_id);
            let data = SplTokenAccount {
                mint: token_mint,
                amount: 0,
                state: spl_token::state::AccountState::Initialized,
                owner: wallet_id,
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
            let id = config::get_user_account(&program_id, &pool_id, &wallet_id).0;
            let data = AirdropClaimer::default();
            let mut data_packed = vec![0; AirdropClaimer::LEN];
            data.pack_into_slice(&mut data_packed);
            program_test.add_account(
                id,
                Account {
                    lamports: 5,
                    data: data_packed,
                    owner: program_id,
                    ..Account::default()
                },
            );
            id
        };

        UserInfo { wallet: wallet_id, account: account_id, token_account: token_account_id }
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

        let account_state = AirdropClaimer::unpack_unchecked(&account.data).unwrap();
        let token_account_state = SplTokenAccount::unpack(&token_account.data).unwrap();

        println!("debug_user({}) : account.key={}, account.owner={}", tag, self.account, account.owner);
        println!("debug_user({}) : token_account.key={}, token_account.owner={}", tag, self.token_account, token_account.owner);
        println!("debug_user({}) : account_state={:?}", tag, account_state);
        println!("debug_user({}) : token_account_state={:?}", tag, token_account_state);
    }
}

