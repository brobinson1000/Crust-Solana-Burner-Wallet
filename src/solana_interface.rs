use std::str::FromStr;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signer::keypair::Keypair,
    signer::Signer,
    transaction::Transaction,
    system_instruction,
};


pub struct SolanaInterface {
    pub keypair: Option<Keypair>,
    pub public_key: String,
    pub secret_key: String,
    pub secret_bytes: [u8; 64],
    pub solana_rpc: String,
}

impl Default for SolanaInterface {
    fn default() -> Self {
        Self {
            keypair: None,
            public_key: String::new(),
            secret_key: String::new(),
            secret_bytes: [0; 64],
            solana_rpc: String::from("https://api.devnet.solana.com"),
        }
    }
}

impl SolanaInterface {
    pub fn generate_keypairs(&mut self) {
        let new_kp = Keypair::new();

        self.public_key = new_kp.pubkey().to_string();

        self.secret_key = new_kp.to_base58_string();

        self.secret_bytes = new_kp.to_bytes();

        self.keypair = Some(new_kp); 

    }

    pub fn send_sol(&mut self, recipient_pubkey: &str, amount_in_sol: f64) -> Result<bool, String> {
       
        let client = RpcClient::new(&self.solana_rpc);

        let sender_kp = self.keypair.as_ref().ok_or("No keypair generated or loaded")?;

        let sender_pubkey = sender_kp.pubkey();

        let recipient_pubkey = Pubkey::from_str(recipient_pubkey)
            .map_err(|e| format!("Invalid recipient public key {}", e))?;


        let lamports = (amount_in_sol * LAMPORTS_PER_SOL as f64) as u64;

        let transfer_instruction = system_instruction::transfer(
            &sender_pubkey,
            &recipient_pubkey,
            lamports,
        );

        let recent_blockhash = client.get_latest_blockhash()
            .map_err(|e| format!("Failed to fetch valid blockhash {}", e))?;



        let transaction = Transaction::new_signed_with_payer(
            &[transfer_instruction],
            Some(&sender_pubkey),
            &[sender_kp],
            recent_blockhash,
        );

        println!("Sending {} sol to {}", amount_in_sol, recipient_pubkey);
        let _signature = client.send_and_confirm_transaction(&transaction)
            .map_err(|e| format!("Transaction execution failed: {}", e))?;

        Ok(true)
    }
        
}

