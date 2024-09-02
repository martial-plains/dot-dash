use std::{
    thread::{sleep, spawn},
    time::Duration,
};

use morsify::Options;
use rodio::{OutputStream, Source};

/// Plays a given Morse code string using audio synthesis.
///
/// # Panics
///
/// This function may panic if:
/// - `OutputStream::try_default()` fails to initialize the default audio output stream. This could happen if there are issues with the audio subsystem or hardware.
/// - `create_sine_wave(frequency, duration_ms)` might panic if it fails to create a valid sine wave buffer. This is generally unlikely but can occur if there's an issue with the buffer size or sample rate calculations.
/// - `stream_handle.play_raw(sine_wave.convert_samples())` might panic if the audio playback fails. This can occur if there's an issue with the audio stream or if the `convert_samples` method encounters an error.
/// - `sleep(Duration::from_millis(u64::from(time_unit)))` might cause a panic if `u64::from(time_unit)` is out of bounds for `Duration::from_millis`. This is highly unlikely given that `u32` values are within the acceptable range.
pub fn play_morse(
    morse_code: &str,
    options: Options,
    frequency: f32,
    time_unit: u32,
    mut onend: impl FnMut() + 'static,
) {
    let morse_code = morse_code.to_string();

    spawn(move || {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let create_sine_wave = |frequency: f32, duration_ms: u32| {
            let sample_rate = 44100;
            let samples = (sample_rate as f32 * (duration_ms as f32 / 1000.0)) as usize;
            let mut buffer = vec![0.0; samples];
            for (i, item) in buffer.iter_mut().enumerate().take(samples) {
                let t = i as f32 / sample_rate as f32;
                *item = (2.0 * std::f32::consts::PI * frequency * t).sin();
            }
            rodio::buffer::SamplesBuffer::new(1, sample_rate, buffer)
        };

        for char in morse_code.chars() {
            match char {
                c if c == options.dot => {
                    let sine_wave = create_sine_wave(frequency, time_unit);
                    stream_handle.play_raw(sine_wave.convert_samples()).unwrap();
                    sleep(Duration::from_millis(u64::from(time_unit)));
                }
                c if c == options.dash => {
                    let sine_wave = create_sine_wave(frequency, time_unit * 3);
                    stream_handle.play_raw(sine_wave.convert_samples()).unwrap();
                    sleep(Duration::from_millis(u64::from(time_unit) * 3));
                }
                c if c == options.separator || c == options.space => {
                    sleep(Duration::from_millis(u64::from(time_unit) * 3));
                }
                _ => {
                    sleep(Duration::from_millis(u64::from(time_unit) * 7));
                }
            }

            sleep(Duration::from_millis(u64::from(time_unit)));
        }
    });

    onend();
}
