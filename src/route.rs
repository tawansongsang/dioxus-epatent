use crate::component::{Blog, Home};
use dioxus::prelude::*;
use dioxus_router::prelude::Routable;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}
