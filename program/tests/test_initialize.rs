mod testutils;

use {
    chikin_airdrop_pool::{
        self,
        config,
        processor::process_instruction,
        state::AirdropClaimer,
    },
    solana_program_test::*,
    solana_sdk::{
        hash::Hash,
        instruction::{AccountMeta, Instruction},
        program_pack::Pack,
        pubkey::Pubkey,
        signature::{Keypair, Signer},
        transaction::Transaction,
    },
    spl_token::{self, state::Account as SplTokenAccount},
};
use chikin_airdrop_pool::instruction::AirdropPoolInstruction;
use testutils::ProgramInfo;
use testutils::UserInfo;
use solana_sdk::account::Account;
use chikin_airdrop_pool::state::AirdropPool;
use std::str::FromStr;

#[tokio::test]
async fn test_initialize() {
    let program_id = Pubkey::new_unique();
    let rent_sysvar_id = solana_program::sysvar::rent::id();
    let system_program_id = solana_program::system_program::id();
    let token_program_id = spl_token::id();
    let token_mint_id = Pubkey::from_str("8s9FCz99Wcr3dHpiauFRi6bLXzshXfcGTfgQE7UEopVx").unwrap();
    let (program_account_id, _) = config::get_pool_account(&program_id, &token_mint_id);
    let program_token_account_id = Pubkey::new_unique();

    println!("test_initialize: program_id={}", program_id);
    println!("test_initialize: token_program_id={}", token_program_id);
    println!("test_initialize: token_mint_id={}", token_mint_id);
    println!("test_initialize: program_account_id={}", program_account_id);
    println!("test_initialize: program_token_account_id={}", program_token_account_id);

    let mut program_test = ProgramTest::new(
        "ChikinProgram", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction),
    );

    // program_test.add_program(
    //     "TokenProgram",
    //     token_program_id,
    //     processor!(spl_token::processor::Processor::process),
    // );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    send_initialize(&mut banks_client,
                    &payer,
                    recent_blockhash,
                    program_id,
                    rent_sysvar_id,
                    system_program_id,
                    token_program_id,
                    token_mint_id,
                    program_account_id,
                    program_token_account_id).await;
}

async fn send_initialize(banks_client: &mut BanksClient,
                         payer: &Keypair,
                         recent_blockhash: Hash,
                         program_id: Pubkey,
                         rent_sysvar_id: Pubkey,
                         system_program_id: Pubkey,
                         token_program_id: Pubkey,
                         token_mint_id: Pubkey,
                         program_account_id: Pubkey,
                         program_token_account_id: Pubkey) {
    let instruction = AirdropPoolInstruction::Initialize;
    let mut instruction_data = [0; chikin_airdrop_pool::instruction::AirdropPoolInstruction::SIZE];
    instruction.pack_into(&mut instruction_data);

    let accounts = vec![
        AccountMeta::new(payer.pubkey(), false),
        AccountMeta::new(program_id, false),
        AccountMeta::new(rent_sysvar_id, false),
        AccountMeta::new(system_program_id, false),
        AccountMeta::new(token_program_id, false),
        AccountMeta::new(token_mint_id, false),
        AccountMeta::new(program_account_id, false),
        AccountMeta::new(program_token_account_id, false),
    ];

    let mut transaction = Transaction::new_with_payer(
        &[
            Instruction::new_with_bytes(program_id,
                                        &instruction_data,
                                        accounts),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}