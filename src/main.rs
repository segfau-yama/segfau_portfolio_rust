use dioxus::prelude::*;

use views::{Blog, Home};
use components::{Header, HeaderItem, HeaderTitle, HeaderItemWrapper, Footer, ScrollHandle, ScrollLink};
mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Wrapper)]
        #[route("/")]
        Home {},
        #[route("/blog/")]
        Blog {},
}

enum Color {
    Primary,
    Secondary,
    Success,
    Danger,
    Warning,
    Info,
}

enum Theme {
    Light,
    Dark,
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[derive(PartialEq, Clone)]
struct HeaderLink {
    name: String,
    anchor: String,
}

#[component]
pub fn Wrapper() -> Element {
    let _scroll = ScrollHandle::init();
    let mut links = use_signal(|| vec![
        HeaderLink { name: "Home".to_string(), anchor: "home".to_string() },
        HeaderLink { name: "Profile".to_string(), anchor: "profile".to_string() },
        HeaderLink { name: "History".to_string(), anchor: "history".to_string() },
        HeaderLink { name: "Work".to_string(), anchor: "work".to_string() },
    ]);

    rsx! {
        div { class: "bg-gray-100",
            Header { color: "bg-emerald-500", size: "py-2 lg:py-3",
                HeaderTitle {
                    Link {
                        to: Route::Home {},
                        "Segfau-Lab"
                    }
                }
                HeaderItemWrapper {
                    for link in links.read().iter() {
                        HeaderItem {
                            ScrollLink {
                                to: link.anchor.clone(),
                                name: link.name.clone(),
                            }
                        }
                    }
                }
            }
            div { class: "container bg-white pb-10 max-w-screen-xl mx-auto",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}
