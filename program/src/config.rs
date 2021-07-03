use solana_program::pubkey::Pubkey;
use spl_associated_token_account;

pub type AmountType = u64;
pub type DurationType = u64;
pub type TimestampType = u64;

#[inline(always)]
pub fn is_valid_pubkey(pubkey: &Pubkey) -> bool {
    pubkey.ne(&Pubkey::default())
}

#[inline(always)]
pub fn get_pool_account(program: &Pubkey, token_mint: &Pubkey, nonce: &[u8]) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[
        &program.to_bytes(),
        &token_mint.to_bytes(),
        "pool_account".as_bytes(),
        nonce,
    ], program)
}

#[macro_export]
macro_rules! pool_account_seeds {
    ($program:expr, $token_mint:expr, $nonce: expr, $bump_seed:expr) => {
        &[
            $program.as_ref(),
            $token_mint.as_ref(),
            "pool_account".as_bytes(),
            $nonce,
            &[$bump_seed],
        ]
    };
}

#[inline(always)]
pub fn get_pool_token_account(program: &Pubkey, pool_account: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[
        &program.to_bytes(),
        &pool_account.to_bytes(),
        "pool_token_account".as_bytes(),
    ], program)
}

#[macro_export]
macro_rules! pool_token_account_seeds {
    ($program:expr, $pool_account:expr, $bump_seed:expr) => {
        &[
            $program.as_ref(),
            $pool_account.as_ref(),
            "pool_token_account".as_bytes(),
            &[$bump_seed],
        ]
    };
}

pub fn get_user_account(program: &Pubkey,
                        pool_account: &Pubkey,
                        user_wallet: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[
        &program.to_bytes(),
        &pool_account.to_bytes(),
        &user_wallet.to_bytes(),
        "claimer_account".as_bytes(),
    ], program)
}

#[macro_export]
macro_rules! user_account_seeds {
    ($program:expr, $pool_account:expr, $user_wallet: expr, $bump_seed:expr) => {
        &[
            $program.as_ref(),
            $pool_account.as_ref(),
            $user_wallet.as_ref(),
            "claimer_account".as_bytes(),
            &[$bump_seed],
        ]
    };
}

pub fn get_user_token_account(token_mint: &Pubkey, user_wallet: &Pubkey) -> Pubkey {
    return spl_associated_token_account::get_associated_token_address(token_mint, user_wallet);
}