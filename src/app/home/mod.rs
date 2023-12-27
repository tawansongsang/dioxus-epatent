mod login;
mod navbar;
use login::Login;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{app::home::navbar::NavBar, route::Route};

#[component]
pub fn Home(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    render! {
        NavBar {},
        Link {
            to: Route::Blog {
                id: *count.get()
            },
            "Go to blog"
        },
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }

        },
        Login {}

    }
}
