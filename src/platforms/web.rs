use std::cell::RefCell;

use dioxus::prelude::spawn;
use gloo::{timers::future::TimeoutFuture, utils::window};
use morsify::Options;
use wasm_bindgen_futures::JsFuture;
use web_sys::{AudioContext, OscillatorType, SpeechSynthesisUtterance};

/// Plays a given Morse code string using the Web Audio API.
///
/// # Panics
///
/// This function will panic if:
/// - The `AudioContext` or `OscillatorNode` cannot be created.
/// - The `OscillatorNode` fails to connect to the audio destination.
/// - The `OscillatorNode` fails to start or stop.
pub fn play_morse(
    morse_code: &str,
    options: Options,
    frequency: f32,
    time_unit: u32,
    mut onend: impl FnMut() + 'static,
) {
    use std::rc::Rc;

    let audio_ctx = Rc::new(AudioContext::new().unwrap());
    let o = Rc::new(RefCell::new({
        let o = audio_ctx.create_oscillator().unwrap();
        o.set_type(web_sys::OscillatorType::Sine);
        o.connect_with_audio_node(&audio_ctx.destination()).unwrap();
        o
    }));
    let morse_code_cloned = morse_code.to_owned();

    spawn(async move {
        let play_tone = || {
            *o.borrow_mut() = audio_ctx.create_oscillator().unwrap();
            o.borrow().set_type(OscillatorType::Sine);
            o.borrow().frequency().set_value(frequency);
            o.borrow()
                .connect_with_audio_node(&audio_ctx.destination())
                .unwrap();
            o.borrow().start().unwrap();
        };

        for char in morse_code_cloned.chars() {
            match char {
                c if c == options.dot => {
                    play_tone();
                    TimeoutFuture::new(time_unit).await;
                    o.borrow().stop().unwrap();
                }
                c if c == options.dash => {
                    play_tone();
                    TimeoutFuture::new(time_unit * 3).await;
                    o.borrow().stop().unwrap();
                }

                c if c == options.separator => {
                    TimeoutFuture::new(time_unit * 3).await;
                }

                c if c == options.space => {
                    TimeoutFuture::new(time_unit * 3).await;
                }

                _ => {
                    TimeoutFuture::new(time_unit * 7).await;
                }
            }

            TimeoutFuture::new(time_unit).await;
        }

        onend();
    });
}

pub fn play_text(text: &str, mut onend: impl FnMut() + 'static) {
    let synth = window().speech_synthesis().unwrap();
    let utterance = SpeechSynthesisUtterance::new().unwrap();
    utterance.set_text(text);
    synth.speak(&utterance);

    onend();
}

pub fn copy_to_clipboard(text: &str) {
    let navigator = window().navigator();
    let clipboard = navigator.clipboard();

    let text = text.to_string();

    spawn(async move {
        JsFuture::from(clipboard.write_text(&text)).await.unwrap();
    });
}
