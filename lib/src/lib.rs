#[cfg(not(target_arch = "wasm32"))]
pub mod desktop;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_arch = "wasm32")]
pub mod web;
