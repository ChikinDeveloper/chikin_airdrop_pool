use std::str::FromStr;

use solana_client::rpc_client::RpcClient;
use solana_sdk;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signer;
use solana_sdk::transaction::Transaction;
use spl_token;

use crate::error::AirdropPoolClientError;
use crate::error::AirdropPoolClientResult;

pub struct Config {
    pub rpc_client: RpcClient,
    pub fee_payer: Box<dyn Signer>,
    pub dry_run: bool,
    pub id_config: IdConfig,
}

impl Config {
    pub fn send_transaction(&self,
                            transaction: Transaction) -> solana_client::client_error::Result<()> {
        if self.dry_run {
            let result = self
                .rpc_client
                .simulate_transaction(&transaction)?;
            println!("Simulate result: {:?}", result);
        } else {
            let signature = self
                .rpc_client
                .send_and_confirm_transaction_with_spinner(&transaction)?;
            println!("Signature: {}", signature);
        }
        Ok(())
    }

    pub fn sign_and_send_transaction(&self,
                                     mut transaction: Transaction,
                                     mut signers: Vec<Box<dyn Signer>>) -> solana_client::client_error::Result<()> {
        let (recent_blockhash, _fee_calculator) = self.rpc_client.get_recent_blockhash()?;
        self.check_fee_payer_balance(1).unwrap(); // TODO
        signers.sort_by_key(|e| e.pubkey());
        signers.dedup();
        transaction.sign(&signers, recent_blockhash);
        self.send_transaction(transaction)
    }

    pub fn check_fee_payer_balance(&self,
                                   required_balance: u64) -> AirdropPoolClientResult<()> {
        let balance = self.rpc_client.get_balance(&self.fee_payer.pubkey())
            .map_err(|_| AirdropPoolClientError::RpcClientError)?;
        if balance < required_balance {
            Err(AirdropPoolClientError::InsufficientBalanceForFees {
                balance,
                required: required_balance,
            })
        } else {
            Ok(())
        }
    }

    pub fn get_fee_payer_balance(&self) -> u64 {
        self.rpc_client.get_account(&self.fee_payer.pubkey()).unwrap().lamports
    }
}

#[derive(Debug)]
pub struct IdConfig {
    pub program: Pubkey,
    pub rent_sysvar: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}

impl Default for IdConfig {
    fn default() -> Self {
        IdConfig {
            program: Pubkey::from_str("GC2MzVrqKfnE8RArGMWVNgVx64qzQF85QrFJFkR5XoaP").unwrap(),
            rent_sysvar: solana_sdk::sysvar::rent::id(),
            system_program: solana_sdk::system_program::id(),
            token_program: spl_token::id(),
        }
    }
}