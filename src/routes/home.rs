use dioxus::prelude::*;
use morsify::{MorseCode, Options as MorseOptions};

#[component]
pub fn Home() -> Element {
    let mut text = use_signal(String::new);
    let mut morse = use_signal(String::new);
    let show_options = use_signal(|| false);
    let morse_code = use_signal(MorseCode::default);

    let morse_opts = use_signal(|| MorseOptions {
        dot: '.',
        dash: '-',
        space: '/',
        separator: ' ',
        ..Default::default()
    });

    let wpm = use_signal(|| 20);
    let frequency = use_signal(|| 700);

    rsx! {
        div { class: "w-full mx-auto h-4/5 p-5",
            div { class: "grid grid-cols-1 md:grid-cols-2 gap-6",
                TextFormControl {
                    text,
                    ontextinput: move |e: Event<FormData>| {
                        let value: String = e.value();
                        text.set(value.clone());
                        morse.set(morse_code().encode(&value));
                    }
                }

                MorseFormControl {
                    morse,
                    ontextinput: move |e: Event<FormData>| {
                        let value: String = e.value();
                        morse.set(value.clone());
                        text.set(morse_code().decode(&value));
                    },
                    morse_opts
                }
            }
            div { class: "grid grid-cols-1 gap-4 mt-6",
                if show_options() {
                    HideOptions { show_options }
                } else {
                    ShowOptions { show_options }
                }
            }
            if show_options() {
                Options {
                    morse_opts,
                    wpm,
                    frequency,
                    oninput: move |_| {
                        morse.set(morse_code().encode(text()));
                    }
                }
            }
        }
    }
}

#[component]
fn TextFormControl(text: Signal<String>, ontextinput: EventHandler<Event<FormData>>) -> Element {
    let mut is_playing = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col",
            label { r#for: "input", class: "label",
                span { class: "label-text font-semibold text-lg dark:text-white", "Text" }
                span { class: "label-text-alt flex gap-4",
                    span {
                        title: "Play the Audio",
                        class: format!("tooltip cursor-pointer {}", if is_playing() { "hidden" } else { "" }),
                        id: "play-input",
                        onclick: move |_| {
                            is_playing.set(true);
                            #[cfg(target_arch = "wasm32")]
                            {
                                use crate::platforms::web::play_text;
                                play_text(text().as_str(), move || { is_playing.set(false) });
                            }
                            #[cfg(target_os = "macos")]
                            {
                                use crate::platforms::macos::play_text;
                                play_text(text().as_str(), move || { is_playing.set(false) });
                            }
                        },
                        img {
                            class: "fill-current dark:filter dark:invert",
                            src: asset!("/public/images/play.svg"),
                            height: 20,
                            width: 20
                        }
                    }
                    span {
                        title: "Stop the Audio",
                        class: format!("tooltip cursor-pointer {}", if is_playing() { "" } else { "hidden" }),
                        id: "stop-input",
                        img {
                            class: "fill-none dark:filter dark:invert",
                            src: asset!("/public/images/stop.svg"),
                            height: 20,
                            width: 20
                        }
                    }

                    span {
                        "data-clipboard-target": "#input",
                        "aria-label": "Click to Copy",
                        "data-copied": "Copied!",
                        title: "Click to Copy",
                        class: "tooltip cursor-pointer clipboard",
                        onclick: move |_| {
                            #[cfg(target_arch = "wasm32")]
                            {
                                use crate::platforms::web::copy_to_clipboard;
                                copy_to_clipboard(text().as_str());
                            }
                            #[cfg(target_os = "macos")]
                            {
                                use crate::platforms::macos::copy_to_clipboard;
                                copy_to_clipboard(text().as_str());
                            }
                        },
                        img {
                            class: "fill-none dark:filter dark:invert",
                            src: asset!("/public/images/clipboard.svg"),
                            height: 20,
                            width: 20
                        }
                    }
                }
            }
            textarea {
                class: "textarea textarea-bordered w-full h-96 bg-base-100 border-solid border-2 dark:text-white dark:bg-[#24283B]",
                id: "input",
                value: text,
                oninput: move |e| ontextinput.call(e)
            }
        }
    }
}

#[component]
fn MorseFormControl(
    morse: Signal<String>,
    ontextinput: EventHandler<Event<FormData>>,
    morse_opts: Signal<MorseOptions>,
) -> Element {
    let mut is_playing = use_signal(|| false);

    rsx! {
        div { class: "flex flex-col",
            label { r#for: "output", class: "label",
                span { class: "label-text font-semibold text-lg dark:text-white", "Morse Code" }
                span { class: "label-text-alt flex gap-4",
                    if !is_playing() {
                        span {
                            "aria-label": "Play the Audio",
                            title: "Play the Audio",
                            class: "cursor-pointer",
                            id: "play-output",
                            onclick: move |_| {
                                #[cfg(target_arch = "wasm32")]
                                {
                                    use crate::platforms::web::play_morse;
                                    is_playing.set(true);
                                    play_morse(
                                        &morse(),
                                        morse_opts(),
                                        700.0,
                                        60,
                                        move || { is_playing.set(false) },
                                    );
                                }
                                #[cfg(not(target_arch = "wasm32"))]
                                {
                                    use crate::platforms::desktop::play_morse;
                                    is_playing.set(true);
                                    play_morse(
                                        &morse(),
                                        morse_opts(),
                                        700.0,
                                        60,
                                        move || { is_playing.set(false) },
                                    );
                                }
                            },
                            img {
                                class: "fill-none dark:filter dark:invert",
                                src: asset!("public/images/play.svg"),
                                height: 20,
                                width: 20
                            }
                        }
                    }

                    if is_playing() {
                        span {
                            "aria-label": "Stop the Audio",
                            title: "Stop the Audio",
                            class: "cursor-pointer",
                            id: "stop-output",
                            img {
                                class: "fill-none dark:filter dark:invert",
                                src: asset!("/public/images/stop.svg"),
                                height: 20,
                                width: 20
                            }
                        }
                    }
                    span {
                        "aria-label": "Click to Copy",
                        "data-copied": "Copied!",
                        title: "Click to Copy",
                        "data-clipboard-target": "#output",
                        class: "tooltip cursor-pointer clipboard",
                        onclick: move |_| {
                            #[cfg(target_arch = "wasm32")]
                            {
                                use crate::platforms::web::copy_to_clipboard;
                                copy_to_clipboard(morse().as_str());
                            }
                            #[cfg(target_os = "macos")]
                            {
                                use crate::platforms::macos::copy_to_clipboard;
                                copy_to_clipboard(morse().as_str());
                            }
                        },

                        img {
                            class: "fill-none dark:filter dark:invert",
                            src: asset!("/public/images/clipboard.svg"),
                            height: 20,
                            width: 20
                        }
                    }
                }
            }
            textarea {
                class: "textarea textarea-bordered w-full h-96 lg:h-full bg-base-100 border-solid border-2 dark:text-white dark:bg-[#24283B]",
                id: "output",
                value: morse,
                oninput: move |e| {
                    ontextinput.call(e);
                }
            }
        }
    }
}

#[component]
fn ShowOptions(show_options: Signal<bool>) -> Element {
    rsx! {
        span {
            class: "flex gap-2 items-center justify-center cursor-pointer text-info",
            id: "show-options",
            onclick: move |_| {
                show_options.set(true);
            },

            span { class: "inline-flex",
                img {
                    class: "dark:filter dark:invert",
                    src: asset!("/public/images/options.svg"),
                    height: 24,
                    width: 24
                }
            }
            span { class: "hidden sm:inline-flex", "Options" }
        }
    }
}

#[component]
fn HideOptions(show_options: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "flex gap-2 items-center justify-center cursor-pointer text-info",
            id: "hide-options",
            onclick: move |_| {
                show_options.set(false);
            },
            span { class: "inline-flex",
                img {
                    class: "dark:filter dark:invert",
                    class: "fill-none",
                    src: asset!("/public/images/hide.svg"),
                    height: 24,
                    width: 24
                }
            }
            span { class: "hidden sm:inline-flex dark:text-white", "Hide Options" }
        }
    }
}

#[component]
fn Options(
    morse_opts: Signal<MorseOptions>,
    wpm: Signal<i64>,
    frequency: Signal<i64>,
    oninput: EventHandler<Event<FormData>>,
) -> Element {
    rsx! {
        div { class: "my-12", id: "options",
            h2 { class: "divider divider-neutral text-center text-lg sm:text-2xl font-bold my-6 dark:text-white",
                "Options"
            }
            div { class: "grid grid-cols-2 md:grid-cols-4 gap-6 my-4",
                div { class: "form-control w-full",
                    label { r#for: "dot", class: "label",
                        span { class: "label-text font-semibold text-base dark:text-white", "Dot" }
                    }
                    input {
                        value: morse_opts().dot.to_string(),
                        r#type: "text",
                        prevent_default: "oninput",
                        maxlength: 1,
                        class: "input input-bordered w-full",
                        id: "dot",
                        oninput: move |e| {
                            let Ok(dot) = e.value().parse() else {
                                return;
                            };
                            morse_opts.write().dot = dot;
                            oninput(e);
                        }
                    }
                }
                div { class: "form-control w-full",
                    label { r#for: "dash", class: "label",
                        span { class: "label-text font-semibold text-base dark:text-white", "Dash" }
                    }
                    input {
                        value: morse_opts().dash.to_string(),
                        prevent_default: "oninput",
                        r#type: "text",
                        maxlength: 1,
                        class: "input input-bordered w-full",
                        id: "dash",
                        oninput: move |e| {
                            let Ok(dash) = e.value().parse() else {
                                return;
                            };
                            morse_opts.write().dash = dash;
                            oninput(e);
                        }
                    }
                }
                div { class: "form-control w-full",
                    label { r#for: "space", class: "label",
                        span { class: "label-text font-semibold text-base dark:text-white", "Space" }
                    }
                    input {
                        r#type: "text",
                        maxlength: 1,
                        value: morse_opts().space.to_string(),
                        class: "input input-bordered w-full",
                        id: "space",
                        oninput: move |e| {
                            let Ok(space) = e.value().parse() else {
                                return;
                            };
                            morse_opts.write().space = space;
                            oninput(e);
                        }
                    }
                }
                div { class: "form-control w-full",
                    label { r#for: "separator", class: "label",
                        span { class: "label-text font-semibold text-base dark:text-white", "Separator" }
                    }
                    input {
                        value: morse_opts().separator.to_string(),
                        r#type: "text",
                        maxlength: 1,
                        class: "input input-bordered w-full",
                        id: "separator",
                        oninput: move |e| {
                            let Ok(separator) = e.value().parse() else {
                                return;
                            };
                            morse_opts.write().separator = separator;
                            oninput(e);
                        }
                    }
                }
                div { class: "form-control w-full",
                    label { r#for: "wpm", class: "label",
                        span { class: "label-text font-semibold text-base dark:text-white", "WPM" }
                    }
                    input {
                        r#type: "number",
                        value: wpm,
                        class: "input input-bordered w-full",
                        id: "wpm",
                        oninput: move |e| {
                            wpm.set(e.value().parse::<i64>().unwrap_or_default());
                            oninput(e);
                        }
                    }
                }
                div { class: "form-control w-full",
                    label { r#for: "frequency", class: "label",
                        span { class: "label-text font-semibold text-base dark:text-white", "Frequency (Hz)" }
                    }
                    input {
                        r#type: "number",
                        value: frequency,
                        class: "input input-bordered w-full",
                        id: "frequency",
                        onchange: move |e| {
                            frequency.set(e.value().parse::<i64>().unwrap_or_default());
                            oninput(e);
                        }
                    }
                }
            }
        }
    }
}
