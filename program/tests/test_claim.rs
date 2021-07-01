mod testutils;

use {
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
    chikin_airdrop_pool::{
        self,
        config,
        processor::process_instruction,
        state::AirdropClaimer,
    },
};

use testutils::UserInfo;
use testutils::ProgramInfo;
use std::str::FromStr;

#[tokio::test]
async fn test_claim() {
    let program_id = Pubkey::new_unique();
    let token_program_id = spl_token::id();
    let token_mint_id = Pubkey::from_str("3K1Td3DmxWt2rxT1H4furqWJyZu3nuc7QQs6W5rtHY3P").unwrap();

    println!("test_chikin_airdrop_pool: program_id={}", program_id);
    println!("test_chikin_airdrop_pool: token_program_id={}", token_program_id);
    println!("test_chikin_airdrop_pool: token_mint_id={}", token_mint_id);

    let mut program_test = ProgramTest::new(
        "ChikinProgram", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction),
    );

    program_test.add_program(
        "TokenProgram",
        token_program_id,
        processor!(spl_token::processor::Processor::process),
    );

    let program_info = ProgramInfo::create(&mut program_test,
                                             &program_id,
                                             token_program_id,
                                             token_mint_id);

    let user1_info = UserInfo::create(&mut program_test, program_id, token_mint_id, program_info.pool_account_id);
    let user2_info = UserInfo::create(&mut program_test, program_id, token_mint_id, program_info.pool_account_id);
    let user3_info = UserInfo::create(&mut program_test, program_id, token_mint_id, program_info.pool_account_id);

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    test_user(&mut banks_client,
              &payer,
              recent_blockhash,
              program_id,
              token_mint_id,
              &program_info,
              &user1_info,
              &[]).await;

    user1_info.debug("user1", &mut banks_client).await;

    test_user(&mut banks_client,
              &payer,
              recent_blockhash,
              program_id,
              token_mint_id,
              &program_info,
              &user2_info,
              &[user1_info.clone()]).await;

    test_user(&mut banks_client,
              &payer,
              recent_blockhash,
              program_id,
              token_mint_id,
              &program_info,
              &user3_info,
              &[user2_info.clone(), user1_info.clone()]).await;

    user1_info.debug("user1", &mut banks_client).await;
    user2_info.debug("user2", &mut banks_client).await;
}


async fn test_user(banks_client: &mut BanksClient,
                   payer: &Keypair,
                   recent_blockhash: Hash,
                   program_id: Pubkey,
                   token_mint_id: Pubkey,
                   program_info: &ProgramInfo,
                   user_info: &UserInfo,
                   referrers: &[UserInfo]) {

    // Verify account initialization
    let user_token_account = banks_client
        .get_account(user_info.token_account)
        .await
        .expect("get_account")
        .expect("user_token_account not found");
    let user_token_account_state = SplTokenAccount::unpack_unchecked(&user_token_account.data).unwrap();
    assert_eq!(user_token_account_state.amount, 0);

    // Claim reward
    claim_reward(banks_client,
                 payer,
                 recent_blockhash,
                 program_id,
                 token_mint_id,
                 program_info,
                 user_info,
                 referrers).await;

    // Verify account update
    let user_account = banks_client.get_account(user_info.account)
        .await
        .expect("get_account")
        .expect("user_account not found");
    let user_account_state = AirdropClaimer::unpack_unchecked(&user_account.data).unwrap();
    assert_eq!(user_account_state.claimed, true);
    assert_eq!(user_account_state.referrer_account, referrers.first().map(|e| e.account).unwrap_or(Pubkey::default()));

    let user_token_account = banks_client
        .get_account(user_info.token_account)
        .await
        .expect("get_account")
        .expect("user_token_account not found");
    let user_token_account_state = SplTokenAccount::unpack_unchecked(&user_token_account.data).unwrap();
    assert_eq!(user_token_account_state.amount, config::REWARD_PER_ACCOUNT);
}

async fn claim_reward(banks_client: &mut BanksClient,
                      payer: &Keypair,
                      recent_blockhash: Hash,
                      program_id: Pubkey,
                      token_mint_id: Pubkey,
                      program_info: &ProgramInfo,
                      user_info: &UserInfo,
                      referrers: &[UserInfo]) {
    let accounts = {
        let mut result = vec![
            AccountMeta::new(program_id, false),
            AccountMeta::new(spl_token::id(), false),
            AccountMeta::new(token_mint_id, false),
            AccountMeta::new(program_info.pool_account_id, false),
            AccountMeta::new(program_info.pool_token_account_id, false),
            AccountMeta::new(user_info.account, false),
            AccountMeta::new(user_info.token_account, false),
        ];
        for referrer in referrers {
            result.push(AccountMeta::new(referrer.account, false));
            result.push(AccountMeta::new(referrer.token_account, false));
        }
        result
    };

    let instruction = chikin_airdrop_pool::instruction::AirdropPoolInstruction::Claim {
        referrer: referrers.first().map(|e| e.account).unwrap_or(Pubkey::default()),
    };
    let mut data = [0; chikin_airdrop_pool::instruction::AirdropPoolInstruction::SIZE];
    instruction.pack_into(&mut data);

    let mut transaction = Transaction::new_with_payer(
        &[
            Instruction::new_with_bytes(program_id,
                                        &data,
                                        accounts),
        ],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}