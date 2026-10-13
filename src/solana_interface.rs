use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

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

    pub fn send_sol(&mut self, recipient_pubkey &str, amount_in_sol i32) -> Result<bool, String> {
        
        let lamports = (amount_in_sol * LAMPORT_PER_SOL as f64) as u64;

        let transfer_instruction = system_program::transfer(
            self.public_key,
            recipient_pubkey,
            lamports,
        );


    }
        
}

