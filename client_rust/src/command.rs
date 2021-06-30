use chikin_airdrop::config as program_config;
use chikin_airdrop::instruction::ChikinAirdropInstruction;
use chikin_airdrop::state::{ChikinAirdropPool, ChikinAirdropUser};
use solana_client::rpc_client::RpcClient;
use solana_sdk::native_token::Sol;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::signer::unique_signers;
use solana_sdk::transaction::Transaction;

use crate::config::Config;

type Error = Box<dyn std::error::Error>;
type CommandResult = Result<(), Error>;

pub fn create(config: &Config, token_mint: Pubkey) -> CommandResult {
    let mut transaction = Transaction::new_with_payer(
        &[
            ChikinAirdropInstruction::initialize(
                config.fee_payer.pubkey(),
                config.id_config.program,
                config.id_config.rent_sysvar,
                config.id_config.system_program,
                config.id_config.token_program,
                token_mint,
            ),
        ],
        Some(&config.fee_payer.pubkey()),
    );

    let (recent_blockhash, fee_calculator) = config.rpc_client.get_recent_blockhash()?;

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

pub fn claim(config: &Config, claimer: Pubkey, referrer: Option<Pubkey>) -> CommandResult {
    let mut transaction = Transaction::new_with_payer(
        &[
            ChikinAirdropInstruction::claim(
                config.fee_payer.pubkey(),
                config.id_config.program,
                config.id_config.token_program,
                token_mint,
                claimer,
                referrer,
            ),
        ],
        Some(&config.fee_payer.pubkey()),
    );

    let (recent_blockhash, fee_calculator) = config.rpc_client.get_recent_blockhash()?;

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

