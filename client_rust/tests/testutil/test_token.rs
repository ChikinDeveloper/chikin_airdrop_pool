use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;
use spl_token;
use spl_token::state::Mint as SplTokenMint;

use client_rust::config::Config;

pub struct TestToken {
    pub mint_authority: Keypair,
    pub mint: Keypair,
}

impl TestToken {
    pub fn create(config: &Config) -> Self {
        let token_mint_authority = Keypair::new();
        let token_mint = Keypair::new();

        let minimum_balance_for_rent_exemption = config.rpc_client
            .get_minimum_balance_for_rent_exemption(SplTokenMint::LEN).unwrap();
        let freeze_authority_pubkey = None;

        let instructions = vec![
            solana_sdk::system_instruction::create_account(
                &config.fee_payer.pubkey(),
                &token_mint.pubkey(),
                minimum_balance_for_rent_exemption,
                SplTokenMint::LEN as u64,
                &spl_token::id()),
            spl_token::instruction::initialize_mint(
                &spl_token::id(),
                &token_mint.pubkey(),
                &token_mint_authority.pubkey(),
                freeze_authority_pubkey,
                6).unwrap(),
        ];

        let mut transaction = Transaction::new_with_payer(
            &instructions,
            Some(&config.fee_payer.pubkey()),
        );

        let mut signers = vec![
            config.fee_payer.as_ref(),
            &token_mint,
        ];
        let (recent_blockhash, _) = config.rpc_client.get_recent_blockhash().unwrap();
        config.check_fee_payer_balance(1).unwrap(); // TODO
        signers.sort_by_key(|e| e.pubkey());
        signers.dedup();
        transaction.sign(&signers, recent_blockhash);
        config.send_transaction(transaction).unwrap();

        TestToken {
            mint_authority: token_mint_authority,
            mint: token_mint,
        }
    }

    pub fn mint(&self, config: &Config, amount: u64, to_token_account: &Pubkey) {
        let mut transaction = Transaction::new_with_payer(
            &[
                spl_token::instruction::mint_to(&spl_token::id(),
                                                &self.mint.pubkey(),
                                                to_token_account,
                                                &self.mint_authority.pubkey(),
                                                &[],
                                                amount,
                ).unwrap(),
            ],
            Some(&config.fee_payer.pubkey()),
        );

        let mut signers: Vec<&dyn Signer> = vec![&self.mint_authority, &*config.fee_payer];
        let (recent_blockhash, _) = config.rpc_client.get_recent_blockhash().unwrap();
        config.check_fee_payer_balance(1).unwrap(); // TODO
        signers.sort_by_key(|e| e.pubkey());
        signers.dedup();
        transaction.sign(&signers, recent_blockhash);
        config.send_transaction(transaction).unwrap();
    }
}