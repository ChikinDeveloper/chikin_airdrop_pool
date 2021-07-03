use {
    solana_program::{
        program_error::ProgramError,
        program_pack::{Sealed},
        pubkey::Pubkey,
    },
};
use crate::config;
use solana_program::instruction::{Instruction, AccountMeta};
use std::convert::TryInto;
use std::io::Write;

#[derive(Debug)]
pub enum AirdropPoolInstruction {
    Initialize {
        pool_account_nonce: [u8; 4],
        reward_per_account: u64,
        reward_per_referral: u64,
        max_referral_depth: u32,
    },
    Claim { referrer: Pubkey },
}

impl Sealed for AirdropPoolInstruction {}

impl AirdropPoolInstruction {
    pub const SIZE: usize = 33;

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
           pool_account_nonce, reward_per_account, reward_per_referral, max_referral_depth
        };
        let mut data = [0; AirdropPoolInstruction::SIZE];
        object.pack_into(&mut data);
        
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
        token_program: Pubkey,
        token_mint: Pubkey,
        pool_account: Pubkey,
        user_wallet: Pubkey,
        referrer_wallet_list: &[Pubkey],
    ) -> Instruction {
        let (pool_token_account, _) = config::get_pool_token_account(&program, &pool_account);
        let (user_account, _) = config::get_user_account(&program, &pool_account, &user_wallet);
        let user_token_account = config::get_user_token_account(&token_mint, &user_wallet);

        let object = AirdropPoolInstruction::Claim {
            referrer: referrer_wallet_list.first().map(|e| e.clone()).unwrap_or_else(|| Pubkey::default()),
        };
        let mut data = [0; AirdropPoolInstruction::SIZE];
        object.pack_into(&mut data);

        let mut accounts = vec![
            AccountMeta::new_readonly(program, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(token_mint, false),
            AccountMeta::new(pool_account, false),
            AccountMeta::new(pool_token_account, false),
            AccountMeta::new(user_wallet, false),
            AccountMeta::new(user_account, false),
            AccountMeta::new(user_token_account, false),
        ];

        for referrer_wallet in referrer_wallet_list {
            let (referrer_account, _) = config::get_user_account(&program, &pool_account, &referrer_wallet);
            let referrer_token_account = config::get_user_token_account(&token_mint, &referrer_wallet);
            accounts.push(AccountMeta::new(referrer_wallet.clone(), false));
            accounts.push(AccountMeta::new(referrer_account, false));
            accounts.push(AccountMeta::new(referrer_token_account, false));
        }

        Instruction::new_with_bytes(program, &data, accounts)
    }

    pub fn pack_into(&self, dst: &mut [u8]) {
        match self {
            AirdropPoolInstruction::Initialize {
                pool_account_nonce,
                reward_per_account,
                reward_per_referral,
                max_referral_depth,
            } => {
                dst[0] = 0;
                dst[1..5].as_mut().write(pool_account_nonce).unwrap();
                dst[5..13].as_mut().write(&reward_per_account.to_be_bytes()).unwrap();
                dst[13..21].as_mut().write(&reward_per_referral.to_be_bytes()).unwrap();
                dst[21..25].as_mut().write(&max_referral_depth.to_be_bytes()).unwrap();
            },
            AirdropPoolInstruction::Claim { referrer } => {
                dst[0] = 1;
                dst[1..33].copy_from_slice(&referrer.to_bytes());
            },
        }
    }

    pub fn unpack(src: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = src.split_first().ok_or(ProgramError::InvalidArgument)?;
        let result = match tag {
            0 => {
                AirdropPoolInstruction::Initialize {
                    pool_account_nonce: rest[0..4].try_into().unwrap(),
                    reward_per_account: u64::from_be_bytes(src[4..12].try_into().unwrap()),
                    reward_per_referral: u64::from_be_bytes(src[12..20].try_into().unwrap()),
                    max_referral_depth: u32::from_be_bytes(src[20..24].try_into().unwrap()),
                }
            },
            1 => {
                AirdropPoolInstruction::Claim { referrer: Pubkey::new(rest) }
            },
            _ => return Err(ProgramError::InvalidArgument),
        };
        Ok(result)
    }
}
