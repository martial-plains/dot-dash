use dioxus::prelude::*;
use routes::home::Home;

pub mod routes;

const STYLE: Asset = asset!("./public/styles/tailwind/tailwind.css");

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("failed to init logger");

    launch(app);
}

fn app() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: STYLE }
        main { class: "bg-transparent dark:bg-slate-800 min-h-screen w-screen", Router::<Route> {} }
    }
}
