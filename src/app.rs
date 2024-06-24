use eframe::egui::{self, Context};
use serde::{Deserialize, Serialize};
use web_sys::{AudioContext, OscillatorNode};

use crate::gui::Gui;

#[cfg(target_arch = "wasm32")]
mod web;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    #[cfg(target_arch = "wasm32")]
    #[serde(skip)]
    audio_ctx: AudioContext,
    #[cfg(target_arch = "wasm32")]
    #[serde(skip)]
    o: OscillatorNode,

    params: Params,
    gui: Gui,
}

impl Default for App {
    fn default() -> Self {
        let context = AudioContext::new().unwrap();
        let o = context.create_oscillator().unwrap();
        o.set_type(web_sys::OscillatorType::Sine);
        o.connect_with_audio_node(&context.destination()).unwrap();

        Self {
            audio_ctx: context,
            o,
            params: Params::default(),
            gui: Gui::default(),
        }
    }
}

impl App {
    /// Called once before the first frame.
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        App::default()
    }

    pub fn update_gui(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        self.gui.update(ctx, frame);

        if let Ok(message) = self.gui.message_channel.1.try_recv() {
            match message {
                crate::gui::Message::SetFrequency(value) => self.params.frequency = value,
                crate::gui::Message::SetTimeUnit(value) => self.params.time_unit = value,
                crate::gui::Message::PlayMorse => {
                    #[cfg(target_arch = "wasm32")]
                    {
                        web::play_morse(
                            &self.params.morse_code,
                            self.params.frequency,
                            self.params.time_unit,
                            &mut self.o,
                            &self.audio_ctx,
                        );
                    }
                }
                crate::gui::Message::UpdateMessage(value) => self.params.message = value,
                crate::gui::Message::UpdateMorseCode(value) => self.params.morse_code = value,
            }
        }
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.update_gui(ctx, frame);
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Params {
    frequency: f32,
    time_unit: u32,
    message: String,
    morse_code: String,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            frequency: 700.0,
            time_unit: 60,
            message: String::default(),
            morse_code: String::default(),
        }
    }
}
