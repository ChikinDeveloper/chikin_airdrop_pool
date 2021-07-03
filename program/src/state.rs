use {
    solana_program::{
        program_error::ProgramError,
        program_pack::{IsInitialized, Pack, Sealed},
        pubkey::Pubkey,
    },
    std::io::Write,
};

use crate::config;
use std::convert::TryInto;

// ProgramState

#[derive(Debug)]
pub struct AirdropPool {
    pub token_program_id: Pubkey,
    pub token_mint_id: Pubkey,
    pub account_id: Pubkey,
    pub token_account_id: Pubkey,
    pub pool_account_nonce: [u8; 4],
    pub reward_per_account: u64,
    pub reward_per_referral: u64,
    pub max_referral_depth: u32,
    pub is_initialized: bool,
}

impl Sealed for AirdropPool {}

impl IsInitialized for AirdropPool {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for AirdropPool {
    const LEN: usize = 153;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        dst[0..32].as_mut().write(&self.token_program_id.to_bytes()).unwrap();
        dst[32..64].as_mut().write(&self.token_mint_id.to_bytes()).unwrap();
        dst[64..96].as_mut().write(&self.account_id.to_bytes()).unwrap();
        dst[96..128].as_mut().write(&self.token_account_id.to_bytes()).unwrap();
        dst[128..132].as_mut().write(&self.pool_account_nonce).unwrap();
        dst[132..140].as_mut().write(&self.reward_per_account.to_be_bytes()).unwrap();
        dst[140..148].as_mut().write(&self.reward_per_referral.to_be_bytes()).unwrap();
        dst[148..152].as_mut().write(&self.max_referral_depth.to_be_bytes()).unwrap();
        dst[152] = self.is_initialized as u8;
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let result = AirdropPool {
            token_program_id: Pubkey::new(&src[0..32]),
            token_mint_id: Pubkey::new(&src[32..64]),
            account_id: Pubkey::new(&src[64..96]),
            token_account_id: Pubkey::new(&src[96..128]),
            pool_account_nonce: src[128..132].try_into().unwrap(),
            reward_per_account: u64::from_be_bytes(src[132..140].try_into().unwrap()),
            reward_per_referral: u64::from_be_bytes(src[140..148].try_into().unwrap()),
            max_referral_depth: u32::from_be_bytes(src[148..152].try_into().unwrap()),
            is_initialized: match src[152] {
                0 => false,
                1 => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },
        };
        Ok(result)
    }
}


// UserState

#[derive(Debug, Default)]
pub struct AirdropClaimer {
    pub claimed: bool,
    pub token_account: Pubkey,
    pub referrer_account: Pubkey,
}

impl AirdropClaimer {
    pub fn has_referrer(&self) -> bool {
        config::is_valid_pubkey(&self.referrer_account)
    }
}

impl Sealed for AirdropClaimer {}

impl Pack for AirdropClaimer {
    const LEN: usize = 65;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        dst[0] = self.claimed as u8;
        dst[1..33].as_mut().write(&self.token_account.to_bytes()).unwrap();
        dst[33..65].as_mut().write(&self.referrer_account.to_bytes()).unwrap();
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let result = AirdropClaimer {
            claimed: match src[0] {
                0 => false,
                1 => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },
            token_account: Pubkey::new(&src[1..33]),
            referrer_account: Pubkey::new(&src[33..65]),
        };
        Ok(result)
    }
}