use solana_program_test::*;
use solana_program::borsh::get_packed_len;
use chikin_airdrop_pool;
use chikin_airdrop_pool::instruction::AirdropPoolInstruction;
use chikin_airdrop_pool::state::AirdropClaimer;
use chikin_airdrop_pool::state::AirdropPool;
use solana_sdk::sysvar::rent::Rent;
use solana_sdk::native_token::Sol;

#[tokio::test]
async fn test_packed_size() {
    let rent = Rent::default();
    let airdrop_pool_len = get_packed_len::<AirdropPool>();
    let airdrop_claimer_len = get_packed_len::<AirdropClaimer>();
    let airdrop_pool_instruction_len = get_packed_len::<AirdropPoolInstruction>();
    println!("airdrop_pool_len={}", airdrop_pool_len);
    println!("airdrop_pool_min_balance_for_rent_exemption={}", Sol(rent.minimum_balance(airdrop_pool_len).max(1)));
    println!("airdrop_claimer_len={}", airdrop_claimer_len);
    println!("airdrop_claimer_min_balance_for_rent_exemption={}", Sol(rent.minimum_balance(airdrop_claimer_len).max(1)));
    println!("airdrop_pool_instruction_len={}", airdrop_pool_instruction_len);
    println!("airdrop_pool_instruction_min_balance_for_rent_exemption={}", Sol(rent.minimum_balance(airdrop_pool_instruction_len).max(1)));
}