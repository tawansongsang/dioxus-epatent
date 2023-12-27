use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::route::Route;

#[component]
pub fn Blog(cx: Scope, id: i32) -> Element {
    render! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}
