use borsh::BorshDeserialize;
use borsh::BorshSchema;
use borsh::BorshSerialize;
use solana_program::pubkey::Pubkey;
use crate::packable::Packable;

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub struct AirdropPool {
    pub token_program_id: Pubkey,
    pub token_mint_id: Pubkey,
    pub account_nonce: [u8; 4],
    pub reward_per_account: u64,
    pub reward_per_referral: u64,
    pub max_referral_depth: u8,
}

implement_packable!(AirdropPool, 85);

#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq, BorshSerialize, BorshDeserialize, BorshSchema)]
pub struct AirdropClaimer {
    pub referrer_wallet: Option<Pubkey>,
    pub claimed: u8,
}

implement_packable!(AirdropClaimer, 34);