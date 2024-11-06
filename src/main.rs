use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use routes::{home::Home, page_not_found::PageNotFound};

pub mod platforms;
pub mod routes;

const STYLE: Asset = asset!("./public/styles/tailwind/tailwind.css");

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    #[cfg(feature = "web")]
    {
        dioxus_logger::init(Level::INFO).expect("failed to init logger");

        launch(app);
    }

    #[cfg(feature = "desktop")]
    {
        use dioxus::desktop::{LogicalSize, WindowBuilder};

        // Init logger
        dioxus_logger::init(Level::INFO).expect("failed to init logger");

        let cfg = dioxus::desktop::Config::new().with_window(
            WindowBuilder::new()
                .with_title("Dot Dash")
                .with_always_on_top(false)
                .with_min_inner_size(LogicalSize::new(400, 600)),
        );
        LaunchBuilder::desktop().with_cfg(cfg).launch(app);
    }
}

fn app() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: STYLE }
        main { class: "bg-transparent dark:bg-slate-800 min-h-screen w-screen", Router::<Route> {} }
    }
}
