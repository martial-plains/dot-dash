use gloo::timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use web_sys::{AudioContext, OscillatorNode, OscillatorType};

pub fn play_morse(
    morse_code: &str,
    frequency: f32,
    time_unit: u32,
    o: &mut OscillatorNode,
    audio_ctx: &AudioContext,
) {
    use std::{cell::RefCell, ptr, rc::Rc};

    let o = Rc::new(RefCell::new(o.clone()));
    let ctx = unsafe { Rc::from_raw(ptr::from_ref(audio_ctx)) };
    let morse_code_cloned = morse_code.to_owned();

    spawn_local(async move {
        let play_tone = || {
            *o.borrow_mut() = ctx.create_oscillator().unwrap();
            o.borrow().set_type(OscillatorType::Sine);
            o.borrow().frequency().set_value(frequency);
            o.borrow()
                .connect_with_audio_node(&ctx.destination())
                .unwrap();
            o.borrow().start().unwrap();
        };

        let chars = morse_code_cloned.chars();
        for char in chars {
            match char {
                '.' => {
                    play_tone();
                    TimeoutFuture::new(time_unit).await;
                    o.borrow().stop().unwrap();
                }
                '-' => {
                    play_tone();
                    TimeoutFuture::new(time_unit * 3).await;
                    o.borrow().stop().unwrap();
                }

                ' ' => {
                    TimeoutFuture::new(time_unit * 3).await;
                }

                _ => {
                    TimeoutFuture::new(time_unit * 7).await;
                }
            }

            TimeoutFuture::new(time_unit).await;
        }
    });
}
