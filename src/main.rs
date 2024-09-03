use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use routes::{home::Home, page_not_found::PageNotFound};

pub mod platforms;
pub mod routes;

const _: &str = manganis::mg!(file("./public/styles/tailwind/tailwind.css"));

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/dot-dash/")]
    Home {},
    // PageNotFound is a catch all route that will match any route and placing the matched segments in the route field
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    #[cfg(not(target_arch = "wasm32"))]
    {
        use dioxus::desktop::{LogicalSize, WindowBuilder};

        let cfg = dioxus::desktop::Config::new().with_window(
            WindowBuilder::new()
                .with_title("Dot Dash")
                .with_always_on_top(false)
                .with_min_inner_size(LogicalSize::new(400, 600)),
        );
        LaunchBuilder::desktop().with_cfg(cfg).launch(App);
    }

    #[cfg(target_arch = "wasm32")]
    {
        launch(App);
    }
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}
