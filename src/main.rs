#![allow(non_snake_case)]
mod app;
mod component;
mod route;

use dioxus_router::prelude::*;

use dioxus::prelude::*;
use log::LevelFilter;

use crate::route::Route;

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    log::info!("starting app");
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    render! {
        Router::<Route> {}
    }
}
