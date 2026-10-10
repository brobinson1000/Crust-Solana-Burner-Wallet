use eframe::egui;
use crate::solana_interface::SolanaInterface;

pub fn show(ctx: &egui::Context, username: &str, open: &mut bool) {
    if !*open {
        return;
    }

    let mut solana_api = SolanaInterface::default();


    ctx.show_viewport_immediate(
        egui::ViewportId::from_hash_of("wallet_window"),
        egui::ViewportBuilder::default()
            .with_title("Crust Wallet")
            .with_inner_size([640.0, 420.0]),
        |ctx, _class| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading("Wallet");
                ui.label(format!("Welcome, {username}"));

                let generate_keypair_btn = egui::Button::new("Generate Keypairs")
                       .min_size(egui::vec2(100.0, 40.0)) // Set minimum dimensions
                       .rounding(4.0);
                if ui.add(generate_keypair_btn).clicked() {
                    println!("Button pressed");
                    solana_api.generate_keypairs();
                    println!("{}", solana_api.public_key);
                }
            });

            // closing the window with the X hides it again
            if ctx.input(|i| i.viewport().close_requested()) {
                *open = false;
            }
        },
    );
}
