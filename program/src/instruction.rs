use {
    solana_program::{
        program_error::ProgramError,
        program_pack::{Sealed},
        pubkey::Pubkey,
    },
};
use crate::config;
use solana_program::instruction::{Instruction, AccountMeta};

#[derive(Debug)]
pub enum AirdropPoolInstruction {
    Initialize,
    Claim { referrer: Pubkey },
}

impl Sealed for AirdropPoolInstruction {}

impl AirdropPoolInstruction {
    pub const SIZE: usize = 33;

    pub fn initialize(
        payer: Pubkey,
        program: Pubkey,
        rent_sysvar: Pubkey,
        system_program: Pubkey,
        token_program: Pubkey,
        token_mint: Pubkey,
    ) -> Instruction {
        let (pool_account, _) = config::get_pool_account(&program, &token_mint);
        let (pool_token_account, _) = config::get_pool_token_account(&program, &pool_account);

        let object = AirdropPoolInstruction::Initialize;
        let mut data = [0; AirdropPoolInstruction::SIZE];
        object.pack_into(&mut data);
        
        let accounts = vec![
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(program, false),
            AccountMeta::new_readonly(rent_sysvar, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(token_mint, false),
            AccountMeta::new(pool_account, false),
            AccountMeta::new(pool_token_account, false),
        ];
        
        Instruction::new_with_bytes(program, &data, accounts)
    }

    pub fn claim(
        payer: Pubkey,
        program: Pubkey,
        token_program: Pubkey,
        token_mint: Pubkey,
        claimer: Pubkey,
        referrer: Option<Pubkey>,
    ) -> Instruction {
        let (pool_account, _) = config::get_pool_account(&program, &token_mint);
        let (pool_token_account, _) = config::get_pool_token_account(&program, &pool_account);

        let object = AirdropPoolInstruction::Claim {
            referrer: referrer.unwrap_or_else(|| Pubkey::default()),
        };
        let mut data = [0; AirdropPoolInstruction::SIZE];
        object.pack_into(&mut data);

        let accounts = vec![
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(program, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new(pool_account, false),
            AccountMeta::new(pool_token_account, false),
            AccountMeta::new(claimer, false),
        ];

        Instruction::new_with_bytes(program, &data, accounts)
    }

    pub fn pack_into(&self, dst: &mut [u8]) {
        match self {
            AirdropPoolInstruction::Initialize => {
                dst[0] = 0;
            },
            AirdropPoolInstruction::Claim { referrer } => {
                dst[0] = 1;
                dst[1..33].copy_from_slice(&referrer.to_bytes());
            },
        }
    }

    pub fn unpack(src: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = src.split_first().ok_or(ProgramError::InvalidArgument)?;
        let result = match tag {
            0 => { AirdropPoolInstruction::Initialize },
            1 => {
                AirdropPoolInstruction::Claim { referrer: Pubkey::new(rest) }
            },
            _ => return Err(ProgramError::InvalidArgument),
        };
        Ok(result)
    }
}
