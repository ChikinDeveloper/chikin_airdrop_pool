use solana_program;
use solana_program::account_info::AccountInfo;
use solana_program::account_info::next_account_info;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::invoke_signed;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;
use spl_token;
use spl_token::state::Account as SplTokenAccount;

use crate::config;
use crate::error::AirdropPoolError;
use crate::instruction::AirdropPoolInstruction;
use crate::packable::Packable;
use crate::state::{AirdropClaimer, AirdropPool};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    assert_eq!(instruction_data.len(), AirdropPoolInstruction::PACKED_SIZE);
    let instruction: AirdropPoolInstruction = AirdropPoolInstruction::unpack(instruction_data)?;
    match instruction {
        AirdropPoolInstruction::Initialize {
            pool_account_nonce,
            reward_per_account,
            reward_per_referral,
            max_referral_depth,
        } => {
            process_initialize(program_id,
                               accounts,
                               pool_account_nonce,
                               reward_per_account,
                               reward_per_referral,
                               max_referral_depth)
        }
        AirdropPoolInstruction::Claim { referrer } => {
            process_claim(program_id, accounts, referrer)
        }
    }
}

pub fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    pool_account_nonce: [u8; 4],
    reward_per_account: u64,
    reward_per_referral: u64,
    max_referral_depth: u8,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let funder = next_account_info(accounts_iter)?;
    let program = next_account_info(accounts_iter)?;
    let rent_sysvar = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let token_mint = next_account_info(accounts_iter)?;
    let pool_account = next_account_info(accounts_iter)?;
    let pool_token_account = next_account_info(accounts_iter)?;

    // println!("process_initialize: funder={}, (owner={})", funder.key, funder.owner);
    // println!("process_initialize: program={}, (owner={})", program.key, program.owner);
    // println!("process_initialize: rent_sysvar={}, (owner={})", rent_sysvar.key, rent_sysvar.owner);
    // println!("process_initialize: system_program={}, (owner={})", system_program.key, system_program.owner);
    // println!("process_initialize: token_program={}, (owner={})", token_program.key, token_program.owner);
    // println!("process_initialize: token_mint={}, (owner={})", token_mint.key, token_mint.owner);
    // println!("process_initialize: pool_account={}, (owner={})", pool_account.key, pool_account.owner);
    // println!("process_initialize: pool_token_account={}, (owner={})", pool_token_account.key, pool_token_account.owner);

    //

    let rent = Rent::from_account_info(rent_sysvar)?;

    //

    let (pool_account_id, pool_account_bump_seed) = config::get_pool_account(program.key, token_mint.key, &pool_account_nonce);
    let (pool_token_account_id, pool_token_account_bump_seed) = config::get_pool_token_account(program.key, pool_account.key);

    //

    if program.key != program_id {
        return Err(AirdropPoolError::ProgramKeyMismatch.into());
    }
    if rent_sysvar.key != &solana_program::sysvar::rent::id() {
        return Err(AirdropPoolError::RentSysvarKeyMismatch.into());
    }
    if system_program.key != &solana_program::system_program::id() {
        return Err(AirdropPoolError::SystemProgramKeyMismatch.into());
    }
    if pool_account.key != &pool_account_id {
        return Err(AirdropPoolError::PoolAccountKeyMismatch.into());
    }
    if pool_token_account.key != &pool_token_account_id {
        return Err(AirdropPoolError::PoolTokenAccountKeyMismatch.into());
    }

    // Initialize program account

    init_pool_account(funder,
                      program,
                      system_program,
                      token_program,
                      token_mint,
                      pool_account,
                      &rent,
                      pool_account_nonce,
                      reward_per_account,
                      reward_per_referral,
                      max_referral_depth,
                      pool_account_bump_seed)
        .map_err(|_| AirdropPoolError::InitPoolAccountFailed)?;

    // Initialize program token account

    init_pool_token_account(funder,
                            program,
                            rent_sysvar,
                            system_program,
                            token_program,
                            token_mint,
                            pool_account,
                            pool_token_account,
                            &rent,
                            pool_token_account_bump_seed)
        .map_err(|_| AirdropPoolError::InitPoolTokenAccountFailed)?;

    Ok(())
}

pub fn process_claim(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    referrer: Option<Pubkey>,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // Get accounts

    let program = next_account_info(accounts_iter)?;
    if program.key != program_id {
        return Err(AirdropPoolError::ProgramKeyMismatch.into());
    }

    let rent_sysvar = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let token_mint = next_account_info(accounts_iter)?;
    let pool_account = next_account_info(accounts_iter)?;
    let pool_token_account = next_account_info(accounts_iter)?;
    let claimer_wallet = next_account_info(accounts_iter)?;
    let claimer_account = next_account_info(accounts_iter)?;
    let claimer_token_account = next_account_info(accounts_iter)?;

    // println!("process_claim: program={}, (owner={})", program.key, program.owner);
    // println!("process_claim: token_program={}, (owner={})", token_program.key, token_program.owner);
    // println!("process_claim: token_mint={}, (owner={})", token_mint.key, token_mint.owner);
    // println!("process_claim: pool_account={}, (owner={})", pool_account.key, pool_account.owner);
    // println!("process_claim: pool_token_account={}, (owner={})", pool_token_account.key, pool_token_account.owner);
    // println!("process_claim: user_wallet={}, (owner={})", claimer_wallet.key, claimer_wallet.owner);
    // println!("process_claim: user_account={}, (owner={})", claimer_account.key, claimer_account.owner);
    // println!("process_claim: user_token_account={}, (owner={})", claimer_token_account.key, claimer_token_account.owner);
    // println!("process_claim: referrer={:?}", referrer);

    // Unpack states

    let rent = Rent::from_account_info(rent_sysvar)?;
    let pool_account_state: AirdropPool = AirdropPool::unpack(*pool_account.data.borrow())?;
    let pool_token_account_state = SplTokenAccount::unpack(*pool_token_account.data.borrow())?;

    //

    let (pool_account_id, pool_account_bump_seed) = config::get_pool_account(program.key, token_mint.key, &pool_account_state.account_nonce);
    let (pool_token_account_id, _) = config::get_pool_token_account(program.key, pool_account.key);
    let (claimer_account_id, claimer_account_bump_seed) = config::get_claimer_account(program.key, pool_account.key, claimer_wallet.key);

    // Validate keys

    if program.key != program_id {
        return Err(AirdropPoolError::ProgramKeyMismatch.into());
    }
    if rent_sysvar.key != &solana_program::sysvar::rent::id() {
        return Err(AirdropPoolError::RentSysvarKeyMismatch.into());
    }
    if pool_account.key != &pool_account_id {
        return Err(AirdropPoolError::PoolAccountKeyMismatch.into());
    }
    if pool_account.owner != program_id {
        return Err(AirdropPoolError::PoolAccountOwnerMismatch.into());
    }
    if pool_token_account.key != &pool_token_account_id {
        return Err(AirdropPoolError::PoolTokenAccountKeyMismatch.into());
    }
    if claimer_account.key != &claimer_account_id {
        return Err(AirdropPoolError::UserAccountKeyMismatch.into());
    }
    if claimer_token_account.key != &config::get_claimer_token_account(token_mint.key, claimer_wallet.key) {
        return Err(AirdropPoolError::UserTokenAccountKeyMismatch.into());
    }

    // Validate state

    if pool_token_account_state.amount < pool_account_state.reward_per_account {
        return Err(AirdropPoolError::InsufficientBalance.into());
    }

    // Reward referrers

    {
        let mut depth = 1;
        let mut expected_referrer_wallet_id_option = referrer;
        let mut referrer_wallet: &AccountInfo;
        let mut referrer_account: &AccountInfo;
        let mut referrer_token_account: &AccountInfo;
        let mut referrer_account_state: AirdropClaimer;

        while let Some(expected_referrer_wallet_id) = expected_referrer_wallet_id_option {
            if depth > pool_account_state.max_referral_depth { break; }

            referrer_wallet = next_account_info(accounts_iter)?;
            referrer_account = next_account_info(accounts_iter)?;
            referrer_token_account = next_account_info(accounts_iter)?;

            if referrer_wallet.key != &expected_referrer_wallet_id {
                return Err(AirdropPoolError::ReferrerWalletKeyMismatch.into());
            }
            if referrer_account.key != &config::get_claimer_account(program_id, pool_account.key, referrer_wallet.key).0 {
                return Err(AirdropPoolError::ReferrerAccountKeyMismatch.into());
            }
            if referrer_token_account.key != &config::get_claimer_token_account(token_mint.key, referrer_wallet.key) {
                return Err(AirdropPoolError::ReferrerTokenAccountKeyMismatch.into());
            }

            referrer_account_state = AirdropClaimer::unpack(&referrer_account.data.borrow())?;

            if referrer_account_state.claimed == 0 {
                return Err(AirdropPoolError::ReferrerDidNotClaim.into());
            }

            transfer_to(program.clone(),
                        token_program.clone(),
                        token_mint.clone(),
                        pool_account.clone(),
                        pool_token_account.clone(),
                        referrer_token_account.clone(),
                        &pool_account_state,
                        pool_account_state.reward_per_referral,
                        pool_account_bump_seed)
                .map_err(|_| AirdropPoolError::TransferToReferrerFailed)?;


            expected_referrer_wallet_id_option = referrer_account_state.referrer_wallet;
            depth += 1;
        }
    }

    // println!("Init claimer");
    init_claimer_account(claimer_wallet,
                         program,
                         system_program,
                         pool_account,
                         claimer_wallet,
                         claimer_account,
                         &rent,
                         claimer_account_bump_seed)
        .map_err(|_| AirdropPoolError::InitClaimerAccountFailed)?;

    // println!("Update claimer account");
    let mut claimer_account_state: AirdropClaimer = AirdropClaimer::unpack(*claimer_account.data.borrow())?;
    claimer_account_state.claimed = 1;
    claimer_account_state.referrer_wallet = referrer.clone();
    claimer_account_state.pack_into(&mut &mut claimer_account.data.borrow_mut()[..])?;

    // println!("Reward claimer");
    let mut claimer_reward = pool_account_state.reward_per_account;
    if referrer.is_some() {
        claimer_reward += pool_account_state.reward_per_referral;
    }
    transfer_to(program.clone(),
                token_program.clone(),
                token_mint.clone(),
                pool_account.clone(),
                pool_token_account.clone(),
                claimer_token_account.clone(),
                &pool_account_state,
                claimer_reward,
                pool_account_bump_seed)
        .map_err(|_| AirdropPoolError::TransferToUserFailed)?;

    Ok(())
}

// Utils

pub fn init_pool_account<'a>(
    funder: &AccountInfo<'a>,
    program: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    token_mint: &AccountInfo<'a>,
    pool_account: &AccountInfo<'a>,
    rent: &Rent,
    pool_account_nonce: [u8; 4],
    reward_per_account: u64,
    reward_per_referral: u64,
    max_referral_depth: u8,
    pool_account_bump_seed: u8,
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
        ],
    )?;

    // Initialize account
    AirdropPool {
        token_program_id: token_program.key.clone(),
        token_mint_id: token_mint.key.clone(),
        account_nonce: pool_account_nonce,
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
            // pool_account_seeds!(program.key, token_mint.key, &pool_account_nonce, pool_account_bump_seed),
            // pool_token_account_seeds!(program.key, pool_account.key, pool_token_account_bump_seed),
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
            pool_account_seeds!(program.key, token_mint.key, &pool_account_state.account_nonce, pool_account_bump_seed),
        ],
    )
}