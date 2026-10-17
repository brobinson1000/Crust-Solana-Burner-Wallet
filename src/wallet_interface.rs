use eframe::egui;
use crate::solana_interface::SolanaInterface;
use std::sync::{LazyLock, Mutex};
use std::collections::HashMap;

static TRANSACTION_HISTORY_STORE: LazyLock<Mutex<HashMap<String, f64>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Default)]
pub struct wallet_interface_ {
    pub solana_api: SolanaInterface,
    pub public_key: String,
    pub has_kp: bool,
    pub total_sol: f64,
    pub destination_addr: String,
    pub amount_input_str: String,
}



impl wallet_interface_ {
pub fn show(&mut self, ctx: &egui::Context, username: &str, open: &mut bool) {
    if !*open {
        return;
    }

    let mut solana_api = SolanaInterface::default();
    let mut sender_addr = String::new();
    let mut solana_send_amnt_str = String::new();

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
                        ui.label("Username: ");
                        let username_input = egui::TextEdit::singleline(&mut self.destination_addr)
                            .hint_text("Satoshi");
                        ui.add(username_input);

                        ui.add_space(8.0);

                        let solana_amount_input = egui::TextEdit::singleline(&mut self.amount_input_str)
                            .hint_text("0.0 SOL");
                        ui.add(solana_amount_input);

                        ui.add_space(16.0);
                 });

                ui.horizontal(|ui| {
                    let generate_btn = egui::Button::new("Generate Key Pairs")
                        .min_size(egui::vec2(120.0, 40.0))
                        .rounding(4.0);

                    if ui.add(generate_btn).clicked() {
                        self.solana_api.generate_keypairs();
                        if let Some(pubkey) = self.solana_api.public_key() {
                            self.public_key = pubkey.to_string();
                            self.has_kp = true;
                        }
                    }

                    let send_sol_btn = egui::Button::new("Send Solana")
                        .min_size(egui::vec2(120.0, 40.0))
                        .rounding(4.0);

                    if ui.add(send_sol_btn).clicked() {
                        let amount = self.amount_input_str.trim().parse().unwrap_or(0.0);
                        println!("Sending {} sol", amount);
                    }
                });

                if self.has_kp {
                    ui.add_space(20.0);
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Public Key");
                        ui.monospace(&self.public_key);
                    });
                }
            });

            if ctx.input(|i| i.viewport().close_requested()) {
                *open = false;
            }
        },
    );

}
}
