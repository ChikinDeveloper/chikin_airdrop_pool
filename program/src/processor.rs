use solana_program::{
    self,
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_token::state::Account as SplTokenAccount;

use crate::config;
use crate::error::AirdropPoolError;
use crate::instruction::AirdropPoolInstruction;
use crate::state::{AirdropClaimer, AirdropPool};
use crate::utils;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    assert_eq!(instruction_data.len(), AirdropPoolInstruction::SIZE);
    let instruction = AirdropPoolInstruction::unpack(&instruction_data)?;
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
        },
        AirdropPoolInstruction::Claim { referrer } => {
            process_claim(program_id, accounts, referrer)
        },
    }
}

pub fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    pool_account_nonce: [u8; 4],
    reward_per_account: u64,
    reward_per_referral: u64,
    max_referral_depth: u32,
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

    if program.key != program_id {
        return Err(AirdropPoolError::ProgramKeyMismatch.into());
    }
    if pool_account.key != &config::get_pool_account(program.key, token_mint.key, &pool_account_nonce).0 {
        return Err(AirdropPoolError::PoolAccountKeyMismatch.into());
    }
    if pool_token_account.key != &config::get_pool_token_account(program.key, pool_account.key).0 {
        return Err(AirdropPoolError::PoolTokenAccountKeyMismatch.into());
    }

    // Initialize program account

    utils::init_pool_account(funder,
                             program,
                             rent_sysvar,
                             system_program,
                             token_program,
                             token_mint,
                             pool_account,
                             pool_token_account,
                             pool_account_nonce,
                             reward_per_account,
                             reward_per_referral,
                             max_referral_depth)?;

    // Initialize program token account

    utils::init_pool_token_account(funder,
                                   program,
                                   rent_sysvar,
                                   system_program,
                                   token_program,
                                   token_mint,
                                   pool_account,
                                   pool_token_account,
                                   pool_account_nonce)?;

    Ok(())
}

pub fn process_claim(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    referrer: Pubkey,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // Get accounts

    let program = next_account_info(accounts_iter)?;
    if program.key != program_id {
        return Err(AirdropPoolError::ProgramKeyMismatch.into());
    }

    let token_program = next_account_info(accounts_iter)?;
    let token_mint = next_account_info(accounts_iter)?;
    let pool_account = next_account_info(accounts_iter)?;
    let pool_token_account = next_account_info(accounts_iter)?;
    let user_wallet = next_account_info(accounts_iter)?;
    let user_account = next_account_info(accounts_iter)?;
    let user_token_account = next_account_info(accounts_iter)?;

    println!("process_claim: program={}, (owner={})", program.key, program.owner);
    println!("process_claim: token_program={}, (owner={})", token_program.key, token_program.owner);
    println!("process_claim: token_mint={}, (owner={})", token_mint.key, token_mint.owner);
    println!("process_claim: pool_account={}, (owner={})", pool_account.key, pool_account.owner);
    println!("process_claim: pool_token_account={}, (owner={})", pool_token_account.key, pool_token_account.owner);
    println!("process_claim: user_wallet={}, (owner={})", user_wallet.key, user_wallet.owner);
    println!("process_claim: user_account={}, (owner={})", user_account.key, user_account.owner);
    println!("process_claim: user_token_account={}, (owner={})", user_token_account.key, user_token_account.owner);
    println!("process_claim: referrer={}", referrer);

    // Validate keys
    if program.key != program_id {
        return Err(AirdropPoolError::ProgramKeyMismatch.into());
    }
    if pool_account.owner != program_id {
        return Err(AirdropPoolError::PoolAccountOwnerMismatch.into());
    }
    // TODO REMOVE CANT ANYMORE
    // if pool_account.key != &config::get_pool_account(program.key, token_mint.key).0 {
    //     return Err(AirdropPoolError::PoolAccountKeyMismatch.into());
    // }
    if pool_token_account.key != &config::get_pool_token_account(program.key, pool_account.key).0 {
        return Err(AirdropPoolError::PoolTokenAccountKeyMismatch.into());
    }
    if user_account.key != &config::get_user_account(program.key, pool_account.key, user_wallet.key).0 {
        return Err(AirdropPoolError::UserAccountKeyMismatch.into());
    }
    if user_token_account.key != &config::get_user_token_account(token_mint.key, user_wallet.key) {
        return Err(AirdropPoolError::UserTokenAccountKeyMismatch.into());
    }

    // Unpack states
    let pool_account_state = AirdropPool::unpack(*pool_account.data.borrow())?;
    let pool_token_account_state = SplTokenAccount::unpack(*pool_token_account.data.borrow())?;
    let mut user_account_state = AirdropClaimer::unpack_unchecked(*user_account.data.borrow())?;

    // Validate state
    if pool_token_account_state.amount < pool_account_state.reward_per_account {
        return Err(AirdropPoolError::InsufficientBalance.into());
    }
    if user_account_state.claimed {
        return Err(AirdropPoolError::AlreadyClaimed.into());
    }

    // Reward referrers

    if config::is_valid_pubkey(&referrer) {
        let mut depth = 1;
        let mut expected_referrer_wallet_id = referrer;
        let mut referrer_wallet: &AccountInfo;
        let mut referrer_account: &AccountInfo;
        let mut referrer_token_account: &AccountInfo;
        let mut referrer_account_state: AirdropClaimer;

        while config::is_valid_pubkey(&referrer) {
            if depth > pool_account_state.max_referral_depth { break; }

            referrer_wallet = next_account_info(accounts_iter)?;
            referrer_account = next_account_info(accounts_iter)?;
            referrer_token_account = next_account_info(accounts_iter)?;

            if referrer_wallet.key != &expected_referrer_wallet_id {
                return Err(AirdropPoolError::ReferrerWalletKeyMismatch.into());
            }
            if referrer_account.key != &config::get_user_account(program_id, pool_account.key, referrer_wallet.key).0 {
                return Err(AirdropPoolError::ReferrerAccountKeyMismatch.into());
            }
            if referrer_token_account.key != &config::get_user_token_account(token_mint.key, referrer_wallet.key) {
                return Err(AirdropPoolError::ReferrerTokenAccountKeyMismatch.into());
            }

            referrer_account_state = AirdropClaimer::unpack_from_slice(&referrer_account.data.borrow())?;

            if !referrer_account_state.claimed {
                return Err(AirdropPoolError::ReferrerDidNotClaim.into());
            }

            utils::transfer_to(program.clone(),
                               token_program.clone(),
                               token_mint.clone(),
                               pool_account.clone(),
                               pool_token_account.clone(),
                               referrer_token_account.clone(),
                               &pool_account_state,
                               pool_account_state.reward_per_referral)
                .map_err(|_| AirdropPoolError::TransferToReferrerFailed)?;

            if !referrer_account_state.has_referrer() { break; }

            depth += 1;
            expected_referrer_wallet_id = referrer_account_state.referrer_account;
        }
    }

    // Update and reward user

    user_account_state.claimed = true;
    user_account_state.token_account = user_token_account.key.clone();
    user_account_state.referrer_account = referrer.clone();
    user_account_state.pack_into_slice(&mut &mut user_account.data.borrow_mut()[..]);
    utils::transfer_to(program.clone(),
                       token_program.clone(),
                       token_mint.clone(),
                       pool_account.clone(),
                       pool_token_account.clone(),
                       user_token_account.clone(),
                       &pool_account_state,
                       pool_account_state.reward_per_account)
        .map_err(|_e| {
            // println!("GOT ERR {:?}", e);
            AirdropPoolError::TransferToUserFailed
        })?;

    Ok(())
}

