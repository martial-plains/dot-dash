use std::{
    error::Error,
    fs::{self, File},
    io::Write,
};

use serde::Serialize;

const DIOXUS_FILE_NAME: &str = "Dioxus.toml";

#[derive(Debug, Serialize)]
struct Application {
    name: &'static str,
    default_platform: String,
    out_dir: &'static str,
    asset_dir: &'static str,
}

#[derive(Debug, Serialize)]
struct Web {
    app: WebApp,
    watcher: WebWatcher,
}

#[derive(Debug, Serialize)]
struct WebApp {
    base_path: Option<&'static str>,
    title: &'static str,
}

#[derive(Debug, Serialize)]
struct WebWatcher {
    reload_html: bool,
    watch_path: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
struct Config {
    pub application: Application,
    pub web: Web,
}

fn main() -> Result<(), Box<dyn Error>> {
    let platform = std::env::var("DIOXUS_PLATFORM").unwrap_or("web".to_string());
    let config = Config {
        application: Application {
            name: "dot-dash",
            default_platform: platform.clone(),
            out_dir: "dist",
            asset_dir: "public",
        },
        web: Web {
            app: WebApp {
                base_path: if platform == "web" {
                    Some("/dot-dash/")
                } else {
                    None
                },
                title: "Dot Dash",
            },
            watcher: WebWatcher {
                reload_html: true,
                watch_path: vec!["src", "assets"],
            },
        },
    };

    if fs::exists(DIOXUS_FILE_NAME)? {
        fs::remove_file(DIOXUS_FILE_NAME)?;
    }

    let mut config_file = File::create(DIOXUS_FILE_NAME)?;

    config_file.write_all(toml::to_string(&config)?.as_bytes())?;

    Ok(())
}
