use chikin_airdrop_pool::instruction::AirdropPoolInstruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;
use crate::client;

use crate::config::Config;
use crate::error::AirdropPoolClientError;

type Error = Box<dyn std::error::Error>;
type CommandResult = Result<(), Error>;

pub fn initialize(
    config: &Config,
    token_mint: Pubkey,
    pool_account_nonce: [u8; 4],
    reward_per_account: u64,
    reward_per_referral: u64,
    max_referral_depth: u32,
) -> CommandResult {
    let mut transaction = Transaction::new_with_payer(
        &[
            AirdropPoolInstruction::initialize(
                config.fee_payer.pubkey(),
                config.id_config.program,
                config.id_config.rent_sysvar,
                config.id_config.system_program,
                config.id_config.token_program,
                token_mint,
                pool_account_nonce,
                reward_per_account,
                reward_per_referral,
                max_referral_depth,
            ),
        ],
        Some(&config.fee_payer.pubkey()),
    );

    let (recent_blockhash, _fee_calculator) = config.rpc_client.get_recent_blockhash()?;

    config.check_fee_payer_balance(1)?; // TODO

    let mut signers = vec![
        config.fee_payer.as_ref()
    ];
    signers.sort_by_key(|e| e.pubkey());
    signers.dedup();

    transaction.sign(&signers, recent_blockhash);

    config.send_transaction(transaction)?;

    Ok(())
}

pub fn claim(config: &Config, token_mint: Pubkey, pool_account: Pubkey, claimer_wallet: &Keypair, referrer_wallet: Option<Pubkey>) -> CommandResult {
    let pool_account_state = client::get_airdrop_pool(&config.rpc_client, &pool_account)?;

    // Pack referrers
    let mut referrer_wallet_list = vec![];
    let mut tmp_referrer_wallet_option = referrer_wallet;
    let mut tmp_referrer_depth = 1;
    while let Some(tmp_referrer_wallet) = tmp_referrer_wallet_option {
        if tmp_referrer_depth > pool_account_state.max_referral_depth {
            break;
        }
        let referrer_account_state = client::get_airdrop_user(&config.rpc_client, &tmp_referrer_wallet)?;
        if referrer_account_state.claimed == 0 {
            return Err(AirdropPoolClientError::ReferrerDidNotClaim.into());
        }
        referrer_wallet_list.push(tmp_referrer_wallet);
        tmp_referrer_wallet_option = referrer_account_state.referrer_wallet;

        tmp_referrer_depth += 1;
    }

    // Build transaction
    let mut transaction = Transaction::new_with_payer(
        &[
            AirdropPoolInstruction::claim(
                config.id_config.program,
                config.id_config.rent_sysvar,
                config.id_config.system_program,
                config.id_config.token_program,
                token_mint,
                pool_account,
                claimer_wallet.pubkey(),
                &referrer_wallet_list,
            ),
        ],
        Some(&config.fee_payer.pubkey()),
    );

    let (recent_blockhash, _fee_calculator) = config.rpc_client.get_recent_blockhash()?;

    config.check_fee_payer_balance(1)?; // TODO

    let mut signers = vec![
        config.fee_payer.as_ref(),
        claimer_wallet
    ];
    signers.sort_by_key(|e| e.pubkey());
    signers.dedup();

    transaction.sign(&signers, recent_blockhash);

    config.send_transaction(transaction)?;

    Ok(())
}

