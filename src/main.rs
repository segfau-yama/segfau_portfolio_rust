use dioxus::prelude::*;

use views::{Blog, Home};
use components::{Header, HeaderItem, HeaderTitle, HeaderItemWrapper, Footer, ThemeProvider, ColorTheme, ScrollHandle, ScrollLink};
use std::collections::HashMap;
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
    anchor: String,
}

#[component]
pub fn Wrapper() -> Element {
    let _scroll = ScrollHandle::init(64.0);
    let mut links = use_signal(|| vec![
        HeaderLink { name: "Profile".to_string(), anchor: "profile".to_string() },
        HeaderLink { name: "History".to_string(), anchor: "history".to_string() },
        HeaderLink { name: "Work".to_string(), anchor: "work".to_string() },
    ]);
    let mut themes = HashMap::new();
    themes.insert(
        "light".to_string(),
        ColorTheme {
            primary: "#10b981".to_string(),
            secondary: "#0ea5e9".to_string(),
            background: "#f8fafc".to_string(),
            text: "#0f172a".to_string(),
            error: None,
            warning: None,
            success: None,
        },
    );
    themes.insert(
        "dark".to_string(),
        ColorTheme {
            primary: "#22c55e".to_string(),
            secondary: "#38bdf8".to_string(),
            background: "#0f172a".to_string(),
            text: "#e2e8f0".to_string(),
            error: None,
            warning: None,
            success: None,
        },
    );
    let theme = ThemeProvider::init(themes, "light".to_string());

    rsx! {
        div { class: "bg-theme", "data-theme": "{theme.current()}",
            Header { color: "bg-primary", size: "py-2 lg:py-3",
                HeaderTitle {
                    Link {
                        to: Route::Home {},
                        "Segfau-Lab"
                    }                
                }
                HeaderItemWrapper {
                    li { class: "flex items-center p-1 text-sm gap-x-2",
                        button {
                            class: "rounded px-3 py-1 text-xs font-semibold bg-white/20 hover:bg-white/30",
                            onclick: move |_| {
                                theme.toggle("light", "dark");
                            },
                            "Toggle Theme"
                        }
                    }
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
            div { class: "container bg-white pb-10 max-w-screen-xl mx-auto text-theme",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}
