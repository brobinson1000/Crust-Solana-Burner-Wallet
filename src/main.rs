use eframe::egui;
use egui::Color32;

mod wallet_interface;
mod solana_interface;

struct UserProperties {
   username: String,
   password: String,
}

impl Default for UserProperties {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
        }
    }
}



struct MyApp {
    userp: UserProperties,
    show_wallet: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            userp: UserProperties::default(),
            show_wallet: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                let rect = ui.max_rect();

                // background GIF
                ui.put(
                    rect,
                    egui::Image::new(egui::include_image!("../assets/ascii-crab.gif"))
                        .fit_to_exact_size(rect.size()),
                );

                ui.allocate_ui_at_rect(rect.shrink(24.0), |ui| {
                    ui.heading("Crust");

                    ui.vertical(|ui| {
                        ui.label("Username: ");
                        let username_input = egui::TextEdit::singleline(&mut self.userp.username)
                            .hint_text("Satoshi");
                        ui.add(username_input);
                    });

                    ui.vertical(|ui| {
                        ui.label("Password: ");
                        ui.visuals_mut().extreme_bg_color = Color32::from_rgb(255, 112, 64);
                        let password_input = egui::TextEdit::singleline(&mut self.userp.password)
                            .text_color(Color32::WHITE)
                            .password(true)
                            .hint_text("Passphrase")
                            .desired_width(200.0);
                        ui.add(password_input);
                    });


                    let orange = Color32::from_rgb(255, 79, 23);        // same as the crab's eyes
                    let orange_hover = Color32::from_rgb(255, 112, 64);  // lighter on hover
                    let orange_press = Color32::from_rgb(200, 60, 15);   // darker while pressed
                    let dark = Color32::from_rgb(8, 7, 6);               // the GIF's background color

                    let w = &mut ui.visuals_mut().widgets;
                    w.inactive.weak_bg_fill = orange;
                    w.hovered.weak_bg_fill = orange_hover;
                    w.active.weak_bg_fill = orange_press;
                    w.inactive.fg_stroke.color = dark;
                    w.hovered.fg_stroke.color = dark;
                    w.active.fg_stroke.color = dark;

                    if ui.button("Login").clicked() {
                        self.show_wallet = true;
                    }

                });
            });
            wallet_interface::show(ctx, &self.userp.username, &mut self.show_wallet);
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Crust Wallet v.01",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    )
}
