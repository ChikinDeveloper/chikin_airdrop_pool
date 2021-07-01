use solana_program;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke_signed;
use solana_program::program_pack::Pack;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;
use spl_token;

use crate::config;
use crate::error::AirdropPoolError;
use crate::state::AirdropPool;

pub fn init_pool_account<'a>(
    funder: &AccountInfo<'a>,
    program: &AccountInfo<'a>,
    rent_sysvar: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    token_mint: &AccountInfo<'a>,
    pool_account: &AccountInfo<'a>,
    pool_token_account: &AccountInfo<'a>,
) -> ProgramResult {
    let (
        account_id,
        account_bump_seed,
    ) = config::get_pool_account(program.key, token_mint.key);
    let (
        _,
        pool_token_account_bump_seed,
    ) = config::get_pool_token_account(program.key, &account_id);

    if pool_account.key != &account_id {
        return Err(AirdropPoolError::PoolAccountKeyMismatch.into());
    }

    let rent = Rent::from_account_info(rent_sysvar)?;

    // Create account
    // println!("init_pool_account: pool_account={}, data.len={}", pool_account.key, pool_account.data.borrow().len());
    invoke_signed(
        &system_instruction::create_account(
            funder.key,
            pool_account.key,
            rent.minimum_balance(AirdropPool::LEN).max(1),
            AirdropPool::LEN as u64,
            program.key,
        ),
        &[
            funder.clone(),
            pool_account.clone(),
            system_program.clone(),
        ],
        &[
            pool_account_seeds!(program.key, token_mint.key, account_bump_seed),
            pool_token_account_seeds!(program.key, account_id, pool_token_account_bump_seed),
        ],
    )?;

    // Initialize account
    let data = AirdropPool {
        token_program_id: token_program.key.clone(),
        token_mint_id: token_mint.key.clone(),
        account_id: pool_account.key.clone(),
        token_account_id: pool_token_account.key.clone(),
        is_initialized: true,
        account_nonce: config::get_pool_account(program.key, token_mint.key).1,
    };

    data.pack_into_slice(&mut &mut pool_account.data.borrow_mut()[..]);

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
) -> ProgramResult {
    let (
        account_id,
        account_bump_seed,
    ) = config::get_pool_account(program.key, token_mint.key);
    let (
        _,
        pool_token_account_bump_seed,
    ) = config::get_pool_token_account(program.key, &account_id);

    let rent = Rent::from_account_info(rent_sysvar)?;

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
            pool_account_seeds!(program.key, token_mint.key, account_bump_seed),
            pool_token_account_seeds!(program.key, account_id, pool_token_account_bump_seed),
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
            pool_account_seeds!(program.key, token_mint.key, account_bump_seed),
            pool_token_account_seeds!(program.key, account_id, pool_token_account_bump_seed),
        ],
    )?;

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
            pool_account_seeds!(program.key, token_mint.key, pool_account_state.account_nonce),
        ],
    )
    /*token_transfer(token_program.clone(),
                   pool_token_account.clone(),
                   destination.clone(),
                   pool_account.clone(),
                   &[
                       pool_account_seeds!(program.key, pool_account.key, pool_account_state.account_nonce),
                   ],
                   amount)*/
}

/*/// Issue a spl_token `Transfer` instruction.
pub fn token_transfer<'a>(
    token_program: AccountInfo<'a>,
    source: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    authority: AccountInfo<'a>,
    signers: &[&[&[u8]]],
    amount: u64,
) -> Result<(), ProgramError> {
    let ix = spl_token::instruction::transfer(
        token_program.key,
        source.key,
        destination.key,
        authority.key,
        &[authority.key],
        amount,
    )?;
    invoke_signed(
        &ix,
        &[source, destination, authority, token_program],
        signers,
    )
}*/