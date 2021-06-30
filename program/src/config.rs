use solana_program::pubkey::Pubkey;

pub const REWARD_PER_ACCOUNT: AmountType = 1000;
pub const REWARD_PER_REFERRAL: AmountType = 1000;
pub const MAX_REFERRAL_DEPTH: u32 = 2;

pub type AmountType = u64;
pub type DurationType = u64;
pub type TimestampType = u64;

#[inline(always)]
pub fn is_valid_pubkey(pubkey: &Pubkey) -> bool {
    pubkey.ne(&Pubkey::default())
}

#[inline(always)]
pub fn get_pool_account(program_id: &Pubkey, token_mint_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[
        &program_id.to_bytes(),
        &token_mint_id.to_bytes(),
        "pool_account".as_bytes(),
    ], program_id)
}

#[macro_export]
macro_rules! pool_account_seeds {
    ($program_id:expr, $token_mint_id:expr, $bump_seed:expr) => {
        &[
            $program_id.as_ref(),
            $token_mint_id.as_ref(),
            "pool_account".as_bytes(),
            &[$bump_seed],
        ]
    };
}

#[inline(always)]
pub fn get_pool_token_account(program_id: &Pubkey, pool_account_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[
        &program_id.to_bytes(),
        &pool_account_id.to_bytes(),
        "pool_token_account".as_bytes(),
    ], program_id)
}

#[macro_export]
macro_rules! pool_token_account_seeds {
    ($program_id:expr, $pool_account_id:expr, $bump_seed:expr) => {
        &[
            $program_id.as_ref(),
            $pool_account_id.as_ref(),
            "pool_token_account".as_bytes(),
            &[$bump_seed],
        ]
    };
}

pub fn get_claimer_account(program_id: &Pubkey,
                           pool_id: &Pubkey,
                           claimer_token_account_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[
        &program_id.to_bytes(),
        &pool_id.to_bytes(),
        &claimer_token_account_id.to_bytes(),
        "claimer_account".as_bytes(),
    ], program_id)
}

#[macro_export]
macro_rules! claimer_account_seeds {
    ($program_id:expr, $pool_id:expr, $claimer_token_account_id: expr, $bump_seed:expr) => {
        &[
            $program_id.as_ref(),
            $pool_id.as_ref(),
            $claimer_token_account_id.as_ref(),
            "claimer_account".as_bytes(),
            &[$bump_seed],
        ]
    };
}