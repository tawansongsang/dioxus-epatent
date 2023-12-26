mod login;

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::route::Route;
use login::Login;

#[component]
pub fn Blog(cx: Scope, id: i32) -> Element {
    render! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
pub fn Home(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    render! {
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
