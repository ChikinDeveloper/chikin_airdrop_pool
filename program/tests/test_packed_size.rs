use solana_program_test::*;
use solana_program::borsh::get_packed_len;
use chikin_airdrop_pool;
use chikin_airdrop_pool::instruction::AirdropPoolInstruction;
use chikin_airdrop_pool::state::AirdropClaimer;
use chikin_airdrop_pool::state::AirdropPool;

#[tokio::test]
async fn test_packed_size() {
    println!("AirdropPool.len={}", get_packed_len::<AirdropPool>());
    println!("AirdropClaimer.len={}", get_packed_len::<AirdropClaimer>());
    println!("AirdropPoolInstruction.len={}", get_packed_len::<AirdropPoolInstruction>());
}