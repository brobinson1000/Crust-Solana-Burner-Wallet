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
            egui::CentralPanel::default()
                .frame(egui::Frame::none())
                .show(ctx, |ui| {
                    let rect = ui.max_rect();

                    // background GIF
                /*    ui.put(
                        rect,
                        egui::Image::new(egui::include_image!("../assets/solana-rain.gif"))
                            .fit_to_exact_size(rect.size()),
                    );
                */

                    // everything else goes on top of it
                    ui.allocate_ui_at_rect(rect.shrink(24.0), |ui| {
                        ui.heading("Wallet");
                        ui.label(format!("Welcome, {username}"));

                        let generate_keypair_btn = egui::Button::new("Generate Keypairs")
                            .min_size(egui::vec2(100.0, 40.0)) // Set minimum dimensions
                            .rounding(4.0);
                        if ui.add(generate_keypair_btn).clicked() {
                            println!("Button pressed");
                            solana_api.generate_keypairs();
                            if let Some(pubkey) = solana_api.public_key() { 
                                println!("{}", pubkey);
                            }

                        }
                    });
                });

            // closing the window with the X hides it again
            if ctx.input(|i| i.viewport().close_requested()) {
                *open = false;
            }
        },
    );
}
