use dioxus::prelude::*;

use views::{Blog, Home};
use components::{Header, HeaderItem, HeaderTitle, HeaderItemWrapper, Footer};
mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Wrapper)]
        #[route("/")]
        Home {},
        #[route("/blog/:id")]
        Blog { id: i32 },
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
    route: Route,
}

#[component]
pub fn Wrapper() -> Element {
    let mut links = use_signal(|| vec![
        HeaderLink { name: "Home".to_string(), route: Route::Home {} },
        HeaderLink { name: "Blog".to_string(), route: Route::Blog { id: 1 } },
        HeaderLink { name: "Another Blog".to_string(), route: Route::Blog { id: 2 } },
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
                            Link {
                                to: link.route.clone(),
                                "{link.name}"
                            }
                        }
                    }
                }
            }
            div { class: "container bg-white pb-10 max-w-screen-lg mx-auto",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}