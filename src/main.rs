use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::{BsDiscord, BsTwitter, BsGithub, BsEnvelopeFill};
use dioxus_free_icons::Icon;
use dioxus_free_icons::IconShape;

use pages::{Home, Blog};
use components::{Header, HeaderItem, HeaderTitle, HeaderItemWrapper, Footer, ScrollHandle, ScrollLink, Avatar};
mod components;
mod views;
mod pages;

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
    to: String,
}

#[derive(PartialEq, Clone)]
struct FooterLink<T> {
    icon: Box<dyn IconShape>,
    to: String,
}

#[component]
pub fn Wrapper() -> Element {
    let _scroll = ScrollHandle::init();
    let mut header_links = use_signal(|| vec![
        HeaderLink { name: "Home".to_string(), to: "home".to_string() },
        HeaderLink { name: "Profile".to_string(), to: "profile".to_string() },
        HeaderLink { name: "Skill".to_string(), to: "skill".to_string() },
        HeaderLink { name: "History".to_string(), to: "history".to_string() },
        HeaderLink { name: "Work".to_string(), to: "work".to_string() },
    ]);

    let mut footer_links = use_signal(|| vec![
        FooterLink { icon: Box::new(BsDiscord), to: "https://discord.com/users/501014325138292737".to_string() },
        FooterLink { icon: Box::new(BsTwitter), to: "https://twitter.com/VyaVma".to_string() },
        FooterLink { icon: Box::new(BsGithub), to: "https://github.com/segfau-yama".to_string() },
        FooterLink { icon: Box::new(BsEnvelopeFill), to: "mailto:suiki547@gmail.com".to_string() },
    ]);

    rsx! {
        div { class: "bg-gray-100",
            Header { color: "bg-emerald-500", size: "py-2 lg:py-3 px-10",
                HeaderTitle {
                    Link { to: Route::Home {}, "Segfau-Lab" }
                }
                HeaderItemWrapper {
                    for link in header_links.read().iter() {
                        HeaderItem {
                            ScrollLink { to: link.to.clone(), name: link.name.clone() }
                        }
                    }
                }
            }
            div { class: "container bg-white pb-10 max-w-screen-xl mx-auto", Outlet::<Route> {} }
            Footer {
                color: "bg-emerald-500",
                size: "px-4 py-2 lg:px-8 lg:py-3 p-8",
                div { class: "container flex flex-wrap items-center justify-between text-slate-50 max-w-screen-xl mx-auto",
                    Avatar {
                        image: "https://segfau-yama.github.io/segfau-portfolio/assets/segfau_icon-b657bf7d.webp",
                        rounded: "rounded-full",
                        size: "size-16",
                    }
                    div { class: "flex flex-wrap items-center gap-y-2 gap-x-8",
                        for link in footer_links.read().iter() {
                            a { href: link.to,
                                Icon {
                                    width: 30,
                                    height: 30,
                                    fill: "white",
                                    icon: link.icon,
                                }
                            }
                        }
                    }
                }
                p { class: "block mb-4 text-sm text-center text-slate-100 md:mb-0 border-t border-slate-200 mt-4 pt-4",
                    " Copyright © 2026 YamaYama. "
                }
            }
        }
    }
}
