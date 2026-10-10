use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

pub struct SolanaInterface {
    pub keypair: Option<Keypair>,
    pub public_key: String,
    pub secret_key: String,
    pub secret_bytes: [u8; 64],
}

impl Default for SolanaInterface {
    fn default() -> Self {
        Self {
            keypair: None,
            public_key: String::new(),
            secret_key: String::new(),
            secret_bytes: [0; 64],
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
}
