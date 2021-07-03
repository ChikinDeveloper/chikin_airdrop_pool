use borsh::BorshDeserialize;
use borsh::BorshSchema;
use borsh::BorshSerialize;
use solana_program::instruction::AccountMeta;
use solana_program::instruction::Instruction;
use solana_program::pubkey::Pubkey;

use crate::config;
use crate::packable::Packable;

// TODO Find why rust thinks it's dead code

#[repr(C)]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub enum AirdropPoolInstruction {
    Initialize {
        #[allow(dead_code)]
        pool_account_nonce: [u8; 4],
        #[allow(dead_code)]
        reward_per_account: u64,
        #[allow(dead_code)]
        reward_per_referral: u64,
        #[allow(dead_code)]
        max_referral_depth: u32,
    },
    Claim {
        #[allow(dead_code)]
        referrer: Option<Pubkey>,
    },
}

impl AirdropPoolInstruction {
    pub fn initialize(
        payer: Pubkey,
        program: Pubkey,
        rent_sysvar: Pubkey,
        system_program: Pubkey,
        token_program: Pubkey,
        token_mint: Pubkey,
        pool_account_nonce: [u8; 4],
        reward_per_account: u64,
        reward_per_referral: u64,
        max_referral_depth: u32,
    ) -> Instruction {
        let (pool_account, _) = config::get_pool_account(&program, &token_mint, &pool_account_nonce);
        let (pool_token_account, _) = config::get_pool_token_account(&program, &pool_account);

        let object = AirdropPoolInstruction::Initialize {
            pool_account_nonce,
            reward_per_account,
            reward_per_referral,
            max_referral_depth,
        };
        let data: Vec<u8> = object.pack();

        let accounts = vec![
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(program, false),
            AccountMeta::new_readonly(rent_sysvar, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(token_mint, false),
            AccountMeta::new(pool_account, false),
            AccountMeta::new(pool_token_account, false),
        ];

        Instruction::new_with_bytes(program, &data, accounts)
    }

    pub fn claim(
        program: Pubkey,
        rent_sysvar: Pubkey,
        system_program: Pubkey,
        token_program: Pubkey,
        token_mint: Pubkey,
        pool_account: Pubkey,
        claimer_wallet: Pubkey,
        referrer_wallet_list: &[Pubkey],
    ) -> Instruction {
        let (pool_token_account, _) = config::get_pool_token_account(&program, &pool_account);
        let (claimer_account, _) = config::get_claimer_account(&program, &pool_account, &claimer_wallet);
        let claimer_token_account = config::get_claimer_token_account(&token_mint, &claimer_wallet);

        let object = AirdropPoolInstruction::Claim {
            referrer: referrer_wallet_list.first().cloned(),
        };
        let data: Vec<u8> = object.pack();

        let mut accounts = vec![
            AccountMeta::new_readonly(program, false),
            AccountMeta::new_readonly(rent_sysvar, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(token_mint, false),
            AccountMeta::new(pool_account, false),
            AccountMeta::new(pool_token_account, false),
            AccountMeta::new(claimer_wallet, true),
            AccountMeta::new(claimer_account, false),
            AccountMeta::new(claimer_token_account, false),
        ];

        for referrer_wallet in referrer_wallet_list {
            let (referrer_account, _) = config::get_claimer_account(&program, &pool_account, &referrer_wallet);
            let referrer_token_account = config::get_claimer_token_account(&token_mint, &referrer_wallet);
            accounts.push(AccountMeta::new(referrer_wallet.clone(), false));
            accounts.push(AccountMeta::new(referrer_account, false));
            accounts.push(AccountMeta::new(referrer_token_account, false));
        }

        Instruction::new_with_bytes(program, &data, accounts)
    }
}

implement_packable!(AirdropPoolInstruction, 34);
