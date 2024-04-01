use eframe::egui;
use egui::ScrollArea;

#[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    message: String,
    morse_code: String,
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    fn message_ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let Self { message, .. } = self;

        let _heading_response = {
            let text = egui::RichText::new("Message").heading();
            ui.add(egui::Label::new(text))
        };

        let text_edit_response =
            ui.add(egui::TextEdit::multiline(message).min_size(ui.available_size()));

        text_edit_response
    }

    fn morse_code_ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let Self { morse_code, .. } = self;

        let _heading_response = {
            let text = egui::RichText::new("Morse Code").heading();
            ui.add(egui::Label::new(text))
        };
        let text_edit_response =
            ui.add(egui::TextEdit::multiline(morse_code).min_size(ui.available_size()));

        text_edit_response
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                ui.menu_button("View", |ui| {
                    egui::widgets::global_dark_light_mode_buttons(ui);
                })
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                ScrollArea::vertical()
                    .id_source("message")
                    .show(&mut columns[0], |ui| {
                        if self.message_ui(ui).changed() {
                            self.morse_code =
                                algorithms::ciphers::morse_code::encrypt(&self.message);
                        }
                    });

                ScrollArea::vertical()
                    .id_source("morse_code")
                    .show(&mut columns[1], |ui| {
                        if self.morse_code_ui(ui).changed() {
                            self.message =
                                algorithms::ciphers::morse_code::decrypt(&self.morse_code);
                        };
                    });
            });
        });
    }
}
