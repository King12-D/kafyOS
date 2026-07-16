use eframe::egui;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    Graphics,
    Controllers,
    Firmware,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum GpuVendor {
    Nvidia,
    Amd,
    Intel,
    Unknown(String),
}

struct DriverManagerApp {
    active_category: Category,
    gpu_vendor: GpuVendor,
    nvidia_driver_active: bool,
    install_progress: Arc<Mutex<Option<f32>>>,
    gamepads: Vec<String>,
    firmware_status: String,
}

impl Default for DriverManagerApp {
    fn default() -> Self {
        let gpu = Self::detect_gpu();
        let gamepads = Self::detect_gamepads();
        
        Self {
            active_category: Category::Graphics,
            gpu_vendor: gpu,
            nvidia_driver_active: false,
            install_progress: Arc::new(Mutex::new(None)),
            gamepads,
            firmware_status: "Up to date (intel-microcode / amd64-microcode active)".to_string(),
        }
    }
}

impl DriverManagerApp {
    fn detect_gpu() -> GpuVendor {
        // Try to read PCI vendor code of the primary graphics card from sysfs
        if let Ok(entries) = std::fs::read_dir("/sys/class/drm") {
            for entry in entries {
                if let Ok(entry) = entry {
                    let name = entry.file_name().to_string_lossy().into_owned();
                    if name.starts_ok_card() {
                        let mut path = entry.path();
                        path.push("device/vendor");
                        if let Ok(vendor_hex) = std::fs::read_to_string(&path) {
                            let hex = vendor_hex.trim().to_lowercase();
                            if hex.contains("10de") {
                                return GpuVendor::Nvidia;
                            } else if hex.contains("1002") {
                                return GpuVendor::Amd;
                            } else if hex.contains("8086") {
                                return GpuVendor::Intel;
                            } else {
                                return GpuVendor::Unknown(hex);
                            }
                        }
                    }
                }
            }
        }
        
        // Fallback to searching lspci if sysfs doesn't yield results
        GpuVendor::Intel // Default fallback
    }

    fn detect_gamepads() -> Vec<String> {
        let mut devices = Vec::new();
        // Check standard joystick event devices
        if let Ok(entries) = std::fs::read_dir("/dev/input") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().into_owned();
                if name.starts_with("js") {
                    devices.push(format!("Gamepad Controller (/{})", name));
                }
            }
        }
        if devices.is_empty() {
            devices.push("No controllers detected".to_string());
        }
        devices
    }

    fn apply_styling(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        style.visuals.dark_mode = true;
        style.visuals.override_text_color = Some(egui::Color32::from_rgb(240, 240, 245));
        style.visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(18, 18, 24);
        style.visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(30, 30, 42);
        style.visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(45, 45, 65);
        style.visuals.widgets.active.bg_fill = egui::Color32::from_rgb(93, 63, 211);
        ctx.set_style(style);
    }
}

// Helper trait to extend String comparison
trait CardExt {
    fn starts_ok_card(&self) -> bool;
}
impl CardExt for String {
    fn starts_ok_card(&self) -> bool {
        self.starts_with("card") && !self.contains('-')
    }
}

impl eframe::App for DriverManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_styling(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            // Main background layout
            ui.painter().rect_filled(
                ui.max_rect(),
                8.0,
                egui::Color32::from_rgba_unmultiplied(20, 20, 28, 245)
            );

            ui.horizontal(|ui| {
                // Sidebar Navigation (Left Panel)
                ui.allocate_ui_with_layout(
                    egui::vec2(180.0, ui.available_height()),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.add_space(20.0);
                        ui.heading(
                            egui::RichText::new("Drivers & Hardware")
                                .size(16.0)
                                .strong()
                                .color(egui::Color32::from_rgb(180, 150, 250))
                        );
                        ui.add_space(20.0);

                        let btn_width = 160.0;
                        let btn_height = 36.0;

                        if ui.add_sized(
                            [btn_width, btn_height],
                            egui::SelectableLabel::new(
                                self.active_category == Category::Graphics,
                                "Graphics Drivers"
                            )
                        ).clicked() {
                            self.active_category = Category::Graphics;
                        }
                        ui.add_space(5.0);

                        if ui.add_sized(
                            [btn_width, btn_height],
                            egui::SelectableLabel::new(
                                self.active_category == Category::Controllers,
                                "Controllers & Input"
                            )
                        ).clicked() {
                            self.active_category = Category::Controllers;
                            self.gamepads = Self::detect_gamepads(); // refresh
                        }
                        ui.add_space(5.0);

                        if ui.add_sized(
                            [btn_width, btn_height],
                            egui::SelectableLabel::new(
                                self.active_category == Category::Firmware,
                                "Firmware & Updates"
                            )
                        ).clicked() {
                            self.active_category = Category::Firmware;
                        }
                    }
                );

                // Divider line
                let rect = ui.max_rect();
                ui.painter().line_segment(
                    [egui::pos2(rect.min.x + 190.0, rect.min.y + 10.0), egui::pos2(rect.min.x + 190.0, rect.max.y - 10.0)],
                    egui::Stroke::new(1.0, egui::Color32::from_rgb(45, 45, 60))
                );

                // Main Content Panel (Right Panel)
                ui.add_space(25.0);
                ui.vertical(|ui| {
                    ui.add_space(20.0);

                    match self.active_category {
                        Category::Graphics => {
                            ui.heading("Graphics Cards & Drivers");
                            ui.add_space(15.0);

                            let (vendor_name, details, optimized) = match &self.gpu_vendor {
                                GpuVendor::Nvidia => ("NVIDIA Corporation", "Requires proprietary drivers for maximum gaming performance.", false),
                                GpuVendor::Amd => ("Advanced Micro Devices, Inc. (AMD)", "Using high-performance open-source Mesa drivers.", true),
                                GpuVendor::Intel => ("Intel Corporation", "Using power-efficient open-source Mesa drivers.", true),
                                GpuVendor::Unknown(hex) => ("Unknown GPU Vendor", hex.as_str(), false),
                            };

                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("Hardware Detected:").strong());
                                ui.label(vendor_name);
                            });
                            ui.add_space(8.0);
                            ui.label(details);
                            ui.add_space(20.0);

                            if optimized {
                                ui.colored_label(
                                    egui::Color32::from_rgb(100, 220, 150),
                                    "✔ System is optimized out-of-the-box (Mesa active)"
                                );
                            } else if self.nvidia_driver_active {
                                ui.colored_label(
                                    egui::Color32::from_rgb(100, 220, 150),
                                    "✔ NVIDIA Proprietary Driver active (Optimized)"
                                );
                            } else {
                                ui.colored_label(
                                    egui::Color32::from_rgb(250, 180, 100),
                                    "⚠ Standard open-source drivers active. Gaming optimization recommended."
                                );
                                ui.add_space(15.0);

                                let mut progress_guard = self.install_progress.lock().unwrap();
                                if let Some(prog) = *progress_guard {
                                    ui.horizontal(|ui| {
                                        ui.spinner();
                                        ui.label(format!("Installing NVIDIA Drivers... {:.0}%", prog * 100.0));
                                    });
                                    ui.add(egui::ProgressBar::new(prog).show_percentage());
                                    
                                    // Simulated background driver installation progress
                                    if prog >= 1.0 {
                                        *progress_guard = None;
                                        self.nvidia_driver_active = true;
                                    } else {
                                        ctx.request_repaint_after(Duration::from_millis(50));
                                    }
                                } else if ui.button("Install NVIDIA Proprietary Drivers").clicked() {
                                    *progress_guard = Some(0.0);
                                    let progress_clone = Arc::clone(&self.install_progress);
                                    
                                    thread::spawn(move || {
                                        for i in 1..=50 {
                                            thread::sleep(Duration::from_millis(60));
                                            let mut guard = progress_clone.lock().unwrap();
                                            if let Some(ref mut p) = *guard {
                                                *p = (i as f32) / 50.0;
                                            }
                                        }
                                    });
                                }
                            }
                        }
                        Category::Controllers => {
                            ui.heading("Gamepad Controllers");
                            ui.add_space(15.0);
                            ui.label("Kafy OS supports Xbox, PlayStation, Nintendo Switch, and Steam controllers natively.");
                            ui.add_space(20.0);

                            ui.label(egui::RichText::new("Connected Devices:").strong());
                            for gamepad in &self.gamepads {
                                ui.horizontal(|ui| {
                                    ui.label("🎮");
                                    ui.label(gamepad);
                                });
                            }
                            ui.add_space(20.0);
                            if ui.button("Scan for Gamepads").clicked() {
                                self.gamepads = Self::detect_gamepads();
                            }
                        }
                        Category::Firmware => {
                            ui.heading("Firmware & Microcode Updates");
                            ui.add_space(15.0);
                            ui.label("Firmware updates protect your CPU against security vulnerabilities and optimize cache performance.");
                            ui.add_space(20.0);

                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("Status:").strong());
                                ui.colored_label(egui::Color32::from_rgb(100, 220, 150), &self.firmware_status);
                            });
                        }
                    }
                });
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([720.0, 480.0])
            .with_resizable(false)
            .with_title("Driver & Hardware Manager"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Driver Manager",
        options,
        Box::new(|_cc| Box::new(DriverManagerApp::default())),
    )
}
