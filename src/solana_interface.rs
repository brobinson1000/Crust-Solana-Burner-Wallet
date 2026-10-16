use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL, pubkey::Pubkey, signer::keypair::Keypair, signer::Signer,
    system_instruction, transaction::Transaction,
};
use std::str::FromStr;

use anyhow::{anyhow, Context, Result};

pub struct SolanaInterface {
    pub keypair: Option<Keypair>,
    pub client: RpcClient,
}

impl Default for SolanaInterface {
    fn default() -> Self {
        Self {
            keypair: None,
            client: RpcClient::new("https://api.devnet.solana.com"),
        }
    }
}

impl SolanaInterface {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            keypair: None,
            client: RpcClient::new(rpc_url),
        }
    }

    pub fn generate_keypairs(&mut self) {
        self.keypair = Some(Keypair::new());
    }

    pub fn public_key(&self) -> Option<Pubkey> {
        self.keypair.as_ref().map(|kp| kp.pubkey())
    }

    pub fn secret_key_base58(&self) -> Option<String> {
        self.keypair.as_ref().map(|kp| kp.to_base58_string())
    }

    pub fn send_sol(&self, recipient_pubkey: &str, amount_in_sol: f64) -> Result<()> {
        let sender_kp = self
            .keypair
            .as_ref()
            .ok_or_else(|| anyhow!("No keypair generated or loaded"))?;

        let sender_pubkey = sender_kp.pubkey();

        let recipient_pubkey = Pubkey::from_str(recipient_pubkey)
            .with_context(|| format!("Invalid recipient public key {}", recipient_pubkey))?;

        let lamports = (amount_in_sol * LAMPORTS_PER_SOL as f64).round() as u64;

        let transfer_instruction =
            system_instruction::transfer(&sender_pubkey, &recipient_pubkey, lamports);

        let recent_blockhash = self
            .client
            .get_latest_blockhash()
            .context("Failed to get latest blockhash")?;

        let transaction = Transaction::new_signed_with_payer(
            &[transfer_instruction],
            Some(&sender_pubkey),
            &[sender_kp],
            recent_blockhash,
        );

        println!("Sending {} sol to {}", amount_in_sol, recipient_pubkey);
        let _signature = self
            .client
            .send_and_confirm_transaction(&transaction)
            .context("Transaction execution or confirmation failed")?;

        Ok(())
    }
}
