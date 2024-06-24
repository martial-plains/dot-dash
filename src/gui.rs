use std::sync::mpsc::{self, Receiver, Sender};

use eframe::egui::{self, Context, ScrollArea};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    SetFrequency(f32),
    SetTimeUnit(u32),
    #[allow(clippy::enum_variant_names)]
    UpdateMessage(String),
    UpdateMorseCode(String),
    PlayMorse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Gui {
    menu_panel: MenuPanel,
    config_window: ConfigWindow,
    central_panel: CentralPanel,
    #[serde(skip, default = "mpsc::channel")]
    pub message_channel: (Sender<Message>, Receiver<Message>),
}

impl Default for Gui {
    fn default() -> Self {
        Gui::new()
    }
}

impl Gui {
    pub fn new() -> Self {
        Self {
            menu_panel: MenuPanel::default(),
            central_panel: CentralPanel::default(),
            config_window: ConfigWindow::default(),
            message_channel: mpsc::channel(),
        }
    }

    /// Renders the next frame, which includes any UI updates as well
    /// as the `Chip8` graphics state.
    pub fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        let menu_response = self.menu_panel.update(ctx, frame);
        if let MenuPanelResponse::ToggleConfigWindow = menu_response {
            self.config_window.toggle_visibility();
        }

        self.config_window.update(ctx, &mut self.message_channel.0);
        self.central_panel.update(ctx, &mut self.message_channel.0);
    }
}

#[derive(Default, Deserialize, Serialize)]
enum MenuPanelResponse {
    #[default]
    None,

    /// Indicates whether the config window should be toggled.
    ToggleConfigWindow,
}

/// A menu panel intended to be placed near the top of the window,
#[derive(Debug, Default, Deserialize, Serialize)]
struct MenuPanel {}

impl MenuPanel {
    /// Update the Ui of this `MenuPanel`. This will return a [`MenuPanelResponse`] indicating
    /// how other Ui components should be updated.
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) -> MenuPanelResponse {
        let mut response = MenuPanelResponse::default();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                ui.menu_button("Window", |ui| {
                    if ui.button("Config").clicked() {
                        response = MenuPanelResponse::ToggleConfigWindow;
                    }
                });
            });
        });

        response
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigWindow {
    frequency: f32,
    time_unit: u32,
    visible: bool,
}

impl Default for ConfigWindow {
    fn default() -> Self {
        Self {
            frequency: 700.0,
            time_unit: 60,
            visible: false,
        }
    }
}

impl ConfigWindow {
    /// Update and render the `ConfigWindow` to the given `Context`.
    /// This will append any GUI messages to `messages` if the `Chip8` state should be updated.
    fn update(&mut self, ctx: &egui::Context, messages: &mut mpsc::Sender<Message>) {
        egui::Window::new("Config")
            .open(&mut self.visible)
            .show(ctx, |ui| {
                egui::Grid::new("config_grid").show(ui, |ui| {
                    ui.label("Frequency");
                    if ui
                        .add(egui::widgets::DragValue::new(&mut self.frequency))
                        .changed()
                    {
                        let _ = messages.send(Message::SetFrequency(self.frequency));
                    }
                    ui.end_row();

                    ui.label("Time Unit");
                    if ui
                        .add(egui::widgets::DragValue::new(&mut self.time_unit))
                        .changed()
                    {
                        let _ = messages.send(Message::SetTimeUnit(self.time_unit));
                    }
                    ui.end_row();
                });
            });
    }

    /// Toggle the visibility of this `ConfigWindow`,
    fn toggle_visibility(&mut self) {
        self.visible = !self.visible;
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct CentralPanel {
    message: String,
    morse_code: String,
}

impl CentralPanel {
    fn update(&mut self, ctx: &egui::Context, messages: &mut mpsc::Sender<Message>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                ScrollArea::vertical()
                    .id_source("message")
                    .show(&mut columns[0], |ui| {
                        if self.message_ui(ui).changed() {
                            self.morse_code =
                                algorithms::ciphers::morse_code::encrypt(&self.message);
                            let _ = messages
                                .send(Message::UpdateMorseCode(self.morse_code.to_string()));
                        }
                    });

                ScrollArea::vertical()
                    .id_source("morse_code")
                    .show(&mut columns[1], |ui| {
                        if self.morse_code_ui(ui, messages).changed() {
                            self.message =
                                algorithms::ciphers::morse_code::decrypt(&self.morse_code);
                            let _ = messages.send(Message::UpdateMessage(self.message.to_string()));
                        };
                    });
            });
        });
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

    fn morse_code_ui(
        &mut self,
        ui: &mut egui::Ui,
        messages: &mut mpsc::Sender<Message>,
    ) -> egui::Response {
        let Self { morse_code, .. } = self;

        egui::Grid::new("morse_code_header")
            .num_columns(2)
            .show(ui, |ui| {
                let text = egui::RichText::new("Morse Code").heading();
                ui.add(egui::Label::new(text));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    if ui.button("Speak").clicked() {
                        messages.send(Message::PlayMorse).unwrap();
                    }
                });
            });

        let text_edit_response =
            ui.add(egui::TextEdit::multiline(morse_code).min_size(ui.available_size()));

        text_edit_response
    }
}
