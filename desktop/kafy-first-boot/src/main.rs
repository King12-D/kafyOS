use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Step {
    Welcome,
    Keyboard,
    Privacy,
    Gaming,
    Finished,
}

struct FirstBootApp {
    step: Step,
    selected_lang: String,
    selected_kbd: String,
    telemetry: bool,
    steam_integration: bool,
    proton_ge: bool,
    completed: bool,
}

impl Default for FirstBootApp {
    fn default() -> Self {
        Self {
            step: Step::Welcome,
            selected_lang: "English (US)".to_string(),
            selected_kbd: "English (US)".to_string(),
            telemetry: false,
            steam_integration: true,
            proton_ge: true,
            completed: false,
        }
    }
}

impl FirstBootApp {
    fn apply_styling(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        
        // Dark theme customization
        style.visuals.dark_mode = true;
        style.visuals.override_text_color = Some(egui::Color32::from_rgb(240, 240, 245));
        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(18, 18, 24);
        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(30, 30, 42);
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(45, 45, 65);
        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(93, 63, 211); // Indigo active
        
        ctx.set_style(style);
    }

    fn draw_header(&self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(20.0);
            ui.heading(
                egui::RichText::new("KAFY OS")
                    .size(32.0)
                    .strong()
                    .color(egui::Color32::from_rgb(180, 150, 250))
            );
            ui.label(
                egui::RichText::new("Zero-setup. Gaming ready. Beautifully polished.")
                    .size(14.0)
                    .color(egui::Color32::from_rgb(150, 150, 170))
            );
            ui.add_space(10.0);

            // Progress Indicators
            ui.horizontal(|ui| {
                let steps = [Step::Welcome, Step::Keyboard, Step::Privacy, Step::Gaming, Step::Finished];
                ui.columns(5, |cols| {
                    for (i, &s) in steps.iter().enumerate() {
                        let active = self.step == s;
                        let text = match s {
                            Step::Welcome => "1. Welcome",
                            Step::Keyboard => "2. Keyboard",
                            Step::Privacy => "3. Privacy",
                            Step::Gaming => "4. Gaming",
                            Step::Finished => "5. Complete",
                        };
                        let label_color = if active {
                            egui::Color32::from_rgb(180, 150, 250)
                        } else {
                            egui::Color32::from_rgb(100, 100, 120)
                        };
                        cols[i].vertical_centered(|ui| {
                            ui.label(egui::RichText::new(text).size(11.0).strong().color(label_color));
                            // Draw indicator bar
                            let rect = ui.max_rect();
                            let height = 3.0;
                            let bar_color = if active {
                                egui::Color32::from_rgb(120, 80, 220)
                            } else {
                                egui::Color32::from_rgb(40, 40, 50)
                            };
                            ui.painter().rect_filled(
                                egui::Rect::from_min_max(
                                    egui::pos2(rect.min.x, rect.min.y + 16.0),
                                    egui::pos2(rect.max.x, rect.min.y + 16.0 + height)
                                ),
                                1.5,
                                bar_color
                            );
                        });
                    }
                });
            });
            ui.add_space(30.0);
        });
    }

    fn draw_footer(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.add_space(20.0);
        ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
            if self.step == Step::Finished {
                if ui.add_sized([100.0, 36.0], egui::Button::new("Finish")).clicked() {
                    self.completed = true;
                    // Write marker file to indicate first boot has completed
                    let _ = std::fs::create_dir_all(dirs::config_dir().unwrap_or_else(|| std::path::PathBuf::from("/tmp")));
                    if let Some(mut path) = dirs::config_dir() {
                        path.push("kafy-first-boot-done");
                        let _ = std::fs::write(path, "1");
                    }
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            } else {
                let next_label = match self.step {
                    Step::Gaming => "Configure & Next",
                    _ => "Next",
                };
                if ui.add_sized([100.0, 36.0], egui::Button::new(next_label)).clicked() {
                    self.step = match self.step {
                        Step::Welcome => Step::Keyboard,
                        Step::Keyboard => Step::Privacy,
                        Step::Privacy => Step::Gaming,
                        Step::Gaming => Step::Finished,
                        Step::Finished => Step::Finished,
                    };
                }
            }

            if self.step != Step::Welcome {
                if ui.add_sized([80.0, 36.0], egui::Button::new("Back")).clicked() {
                    self.step = match self.step {
                        Step::Welcome => Step::Welcome,
                        Step::Keyboard => Step::Welcome,
                        Step::Privacy => Step::Keyboard,
                        Step::Gaming => Step::Privacy,
                        Step::Finished => Step::Gaming,
                    };
                }
            }
        });
    }
}

impl eframe::App for FirstBootApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_styling(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            // Background aesthetics (simulated dark glass)
            ui.painter().rect_filled(
                ui.max_rect(),
                8.0,
                egui::Color32::from_rgba_unmultiplied(20, 20, 28, 240)
            );

            self.draw_header(ui);

            // Step Content Area
            egui::ScrollArea::vertical().max_height(280.0).show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    match self.step {
                        Step::Welcome => {
                            ui.label(
                                egui::RichText::new("Welcome to your new desktop.")
                                    .size(20.0)
                                    .strong()
                            );
                            ui.add_space(15.0);
                            ui.label(
                                "Kafy OS brings a premium, lag-free user experience, \npowerful window management, and out-of-the-box hardware optimization."
                            );
                            ui.add_space(25.0);

                            ui.horizontal(|ui| {
                                ui.label("System Language: ");
                                egui::ComboBox::from_id_source("lang_box")
                                    .selected_text(&self.selected_lang)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.selected_lang, "English (US)".to_string(), "English (US)");
                                        ui.selectable_value(&mut self.selected_lang, "English (UK)".to_string(), "English (UK)");
                                        ui.selectable_value(&mut self.selected_lang, "Spanish".to_string(), "Español");
                                        ui.selectable_value(&mut self.selected_lang, "French".to_string(), "Français");
                                        ui.selectable_value(&mut self.selected_lang, "German".to_string(), "Deutsch");
                                    });
                            });
                        }
                        Step::Keyboard => {
                            ui.label(
                                egui::RichText::new("Choose Keyboard Layout")
                                    .size(20.0)
                                    .strong()
                            );
                            ui.add_space(15.0);
                            ui.label("Select the layout matching your hardware keyboard.");
                            ui.add_space(25.0);

                            ui.horizontal(|ui| {
                                ui.label("Keyboard Layout: ");
                                egui::ComboBox::from_id_source("kbd_box")
                                    .selected_text(&self.selected_kbd)
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.selected_kbd, "English (US)".to_string(), "English (US)");
                                        ui.selectable_value(&mut self.selected_kbd, "English (US, international)".to_string(), "English (US, international)");
                                        ui.selectable_value(&mut self.selected_kbd, "United Kingdom".to_string(), "United Kingdom");
                                        ui.selectable_value(&mut self.selected_kbd, "Spanish".to_string(), "Spanish");
                                        ui.selectable_value(&mut self.selected_kbd, "French".to_string(), "French");
                                        ui.selectable_value(&mut self.selected_kbd, "German".to_string(), "German");
                                    });
                            });
                        }
                        Step::Privacy => {
                            ui.label(
                                egui::RichText::new("Privacy Preferences")
                                    .size(20.0)
                                    .strong()
                            );
                            ui.add_space(15.0);
                            ui.label(
                                "We respect your data. Choose what you share with the Kafy Project."
                            );
                            ui.add_space(20.0);

                            ui.checkbox(&mut self.telemetry, "Send anonymous crash logs and system statistics");
                            ui.label(
                                egui::RichText::new("Helping us identify hardware incompatibilities. No personal data is ever collected.")
                                    .size(11.0)
                                    .color(egui::Color32::from_rgb(120, 120, 140))
                            );
                        }
                        Step::Gaming => {
                            ui.label(
                                egui::RichText::new("Gaming Baseline Configuration")
                                    .size(20.0)
                                    .strong()
                            );
                            ui.add_space(15.0);
                            ui.label(
                                "Kafy optimizes gaming settings by default. Choose extra utilities to enable now."
                            );
                            ui.add_space(20.0);

                            ui.checkbox(&mut self.steam_integration, "Enable Steam installer & client shortcuts");
                            ui.checkbox(&mut self.proton_ge, "Pre-configure Proton GE compatibility layer for Steam & Lutris");
                        }
                        Step::Finished => {
                            ui.label(
                                egui::RichText::new("You are all set!")
                                    .size(22.0)
                                    .strong()
                                    .color(egui::Color32::from_rgb(180, 150, 250))
                            );
                            ui.add_space(15.0);
                            ui.label(
                                "Kafy OS is configured and ready to go. Click Finish to launch your desktop!"
                            );
                            ui.add_space(25.0);
                            ui.label(
                                egui::RichText::new("Enjoy your streamlined desktop flow.")
                                    .italics()
                                    .color(egui::Color32::from_rgb(150, 150, 170))
                            );
                        }
                    }
                });
            });

            self.draw_footer(ui, ctx);
        });
    }
}

// Simple fallback helper module for dirs crate if it isn't resolved
mod dirs {
    use std::path::PathBuf;
    pub fn config_dir() -> Option<PathBuf> {
        std::env::var("HOME").ok().map(|h| {
            let mut p = PathBuf::from(h);
            p.push(".config");
            p
        })
    }
}

fn main() -> eframe::Result<()> {
    if let Some(mut path) = dirs::config_dir() {
        path.push("kafy-first-boot-done");
        if path.exists() {
            return Ok(());
        }
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([700.0, 500.0])
            .with_resizable(false)
            .with_title("Kafy OS Setup"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Kafy OS Setup",
        options,
        Box::new(|_cc| Box::new(FirstBootApp::default())),
    )
}
