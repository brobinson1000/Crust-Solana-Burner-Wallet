use eframe::egui;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex};
use std::thread;

use crate::solana_interface::SolanaInterface;

#[allow(dead_code)]
static TRANSACTION_HISTORY_STORE: LazyLock<Mutex<HashMap<String, f64>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Default)]
pub struct wallet_interface_ {
    pub solana_api: Arc<SolanaInterface>,
    pub public_key: String,
    pub has_kp: bool,
    pub total_sol: f64,
    pub total_lamports: u64,
    pub destination_addr: String,
    pub amount_input_str: String,
}

impl wallet_interface_ {
    pub fn show(&mut self, ctx: &egui::Context, username: &str, open: &mut bool) {
        if !*open {
            return;
        }

        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("wallet_window"),
            egui::ViewportBuilder::default()
                .with_title("Crust Wallet")
                .with_inner_size([640.0, 420.0]),
            |ctx, _class| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.add_space(24.0);

                    ui.heading("Wallet");
                    ui.label(format!("Welcome, {username}"));

                    ui.vertical(|ui| {
                        ui.label("Destination address:");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.destination_addr)
                                .hint_text("Recipient public key"),
                        );

                        ui.add_space(8.0);

                        ui.label("Amount:");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.amount_input_str)
                                .hint_text("0.0 SOL"),
                        );

                        ui.add_space(16.0);
                    });

                    ui.horizontal(|ui| {
                        let generate_btn = egui::Button::new("Generate Key Pairs")
                            .min_size(egui::vec2(120.0, 40.0))
                            .rounding(4.0);

                        if ui.add(generate_btn).clicked() {
                            let mut api = SolanaInterface::default();
                            api.generate_keypairs();
                            if let Some(pubkey) = api.public_key() {
                                self.public_key = pubkey.to_string();
                                self.has_kp = true;
                            }
                            self.solana_api = Arc::new(api);
                        }

                        if self.has_kp {

                            let mut pubkey = self.public_key;
                            let api = Arc::clone(&self.solana_api);

                            thread::spawn(move || match api.client.get_balance(&pubkey) {
                                Ok(_) => {
                                    println!("Successfully fetched balance")
                                }
                                Err(e) => {
                                    println!("Error has occured {e}");
                                }
                            });
                        }

                        let send_sol_btn = egui::Button::new("Send Solana")
                            .min_size(egui::vec2(120.0, 40.0))
                            .rounding(4.0);

                        if ui.add(send_sol_btn).clicked() && self.has_kp {
                            let destination = self.destination_addr.trim().to_string();
                            let amount: f64 = self.amount_input_str.trim().parse().unwrap_or(0.0);
                            let api = Arc::clone(&self.solana_api);

                            thread::spawn(move || match api.send_sol(&destination, amount) {
                                Ok(_) => println!("Successfully sent {} SOL", amount),
                                Err(e) => println!("Error sending SOL: {:#}", e),
                            });
                        }
                    });




                });

                if ctx.input(|i| i.viewport().close_requested()) {
                    *open = false;
                }
            },
        );
    }
}
