use solana_program;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke_signed;
use solana_program::program_pack::Pack;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use spl_token;

use crate::packable::Packable;
use crate::state::AirdropClaimer;
use crate::state::AirdropPool;

pub fn init_pool_account<'a>(
    funder: &AccountInfo<'a>,
    program: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    token_mint: &AccountInfo<'a>,
    pool_account: &AccountInfo<'a>,
    pool_token_account: &AccountInfo<'a>,
    rent: &Rent,
    pool_account_nonce: [u8; 4],
    reward_per_account: u64,
    reward_per_referral: u64,
    max_referral_depth: u32,
    pool_account_bump_seed: u8,
    pool_token_account_bump_seed: u8,
) -> ProgramResult {
    // Create account
    invoke_signed(
        &system_instruction::create_account(
            funder.key,
            pool_account.key,
            rent.minimum_balance(AirdropPool::PACKED_SIZE).max(1),
            AirdropPool::PACKED_SIZE as u64,
            program.key,
        ),
        &[
            funder.clone(),
            pool_account.clone(),
            system_program.clone(),
        ],
        &[
            pool_account_seeds!(program.key, token_mint.key, &pool_account_nonce, pool_account_bump_seed),
            pool_token_account_seeds!(program.key, pool_account.key, pool_token_account_bump_seed),
        ],
    )?;

    // Initialize account
    AirdropPool {
        token_program_id: token_program.key.clone(),
        token_mint_id: token_mint.key.clone(),
        account_id: pool_account.key.clone(),
        token_account_id: pool_token_account.key.clone(),
        is_initialized: 1,
        pool_account_nonce,
        reward_per_account,
        reward_per_referral,
        max_referral_depth,
    }.pack_into(&mut &mut pool_account.data.borrow_mut()[..])?;

    Ok(())
}

pub fn init_pool_token_account<'a>(
    funder: &AccountInfo<'a>,
    program: &AccountInfo<'a>,
    rent_sysvar: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    token_mint: &AccountInfo<'a>,
    pool_account: &AccountInfo<'a>,
    pool_token_account: &AccountInfo<'a>,
    rent: &Rent,
    pool_account_nonce: [u8; 4],
    pool_account_bump_seed: u8,
    pool_token_account_bump_seed: u8,
) -> ProgramResult {
    // Create account
    invoke_signed(
        &system_instruction::create_account(
            funder.key,
            pool_token_account.key,
            rent.minimum_balance(spl_token::state::Account::LEN).max(1),
            spl_token::state::Account::LEN as u64,
            token_program.key,
        ),
        &[
            funder.clone(),
            rent_sysvar.clone(),
            pool_token_account.clone(),
            system_program.clone(),
            token_program.clone(),
        ],
        &[
            pool_account_seeds!(program.key, token_mint.key, &pool_account_nonce, pool_account_bump_seed),
            pool_token_account_seeds!(program.key, pool_account.key, pool_token_account_bump_seed),
        ],
    )?;

    // Initialize account
    invoke_signed(
        &spl_token::instruction::initialize_account(
            token_program.key,
            pool_token_account.key,
            token_mint.key,
            pool_account.key,
        )?,
        &[
            rent_sysvar.clone(),
            pool_token_account.clone(),
            pool_account.clone(),
            token_mint.clone(),
            token_program.clone(),
        ],
        &[
            pool_account_seeds!(program.key, token_mint.key, &pool_account_nonce, pool_account_bump_seed),
            pool_token_account_seeds!(program.key, pool_account.key, pool_token_account_bump_seed),
        ],
    )?;

    Ok(())
}

pub fn init_claimer_account<'a>(
    funder: &AccountInfo<'a>,
    program: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    pool_account: &AccountInfo<'a>,
    claimer_wallet: &AccountInfo<'a>,
    claimer_account: &AccountInfo<'a>,
    rent: &Rent,
    claimer_account_bump_seed: u8,
) -> ProgramResult {
    // Create account
    invoke_signed(
        &system_instruction::create_account(
            funder.key,
            claimer_account.key,
            rent.minimum_balance(AirdropClaimer::PACKED_SIZE).max(1),
            AirdropClaimer::PACKED_SIZE as u64,
            program.key,
        ),
        &[
            funder.clone(),
            claimer_account.clone(),
            system_program.clone(),
        ],
        &[
            claimer_account_seeds!(program.key, pool_account.key, claimer_wallet.key, claimer_account_bump_seed),
        ],
    )?;

    // Initialize account
    AirdropClaimer {
        referrer_wallet: None,
        claimed: 0,
    }.pack_into(&mut &mut claimer_account.data.borrow_mut()[..])?;

    Ok(())
}

pub fn transfer_to<'a>(
    program: AccountInfo<'a>,
    token_program: AccountInfo<'a>,
    token_mint: AccountInfo<'a>,
    pool_account: AccountInfo<'a>,
    pool_token_account: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    pool_account_state: &AirdropPool,
    amount: u64,
    pool_account_bump_seed: u8,
) -> ProgramResult {
    let ix = spl_token::instruction::transfer(
        token_program.key,
        pool_token_account.key,
        destination.key,
        pool_account.key,
        &[pool_account.key],
        amount,
    )?;
    invoke_signed(
        &ix,
        &[pool_token_account.clone(), destination.clone(), pool_account.clone(), token_program.clone()],
        &[
            pool_account_seeds!(program.key, token_mint.key, &pool_account_state.pool_account_nonce, pool_account_bump_seed),
        ],
    )
}