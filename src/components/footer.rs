use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::{BsDiscord, BsTwitter, BsGithub, BsEnvelopeFill};
use dioxus_free_icons::Icon;
use crate::components::Avatar;

// TODO: フッターコンポーネントの汎用性を上げる
#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "block px-4 py-2 mx-auto w-full lg:px-8 lg:py-3 bg-emerald-500 p-8",
            div { class: "container flex flex-wrap items-center justify-between text-slate-50 max-w-screen-xl mx-auto",
                Avatar { 
                    image: "https://segfau-yama.github.io/segfau-portfolio/assets/segfau_icon-b657bf7d.webp", rounded: "rounded-full", size: "size-16" 
                }
                div { class: "flex flex-wrap items-center gap-y-2 gap-x-8",
                    a {
                        href: "https://discord.com/users/501014325138292737",
                        Icon {
                            width: 30,
                            height: 30,
                            fill: "white",
                            icon: BsDiscord,
                        }
                    }
                    a {
                        href: "https://twitter.com/VyaVma",
                        Icon {
                            width: 30,
                            height: 30,
                            fill: "white",
                            icon: BsTwitter,
                        }
                    }
                    a {
                        href: "https://github.com/segfau-yama",
                        Icon {
                            width: 30,
                            height: 30,
                            fill: "white",
                            icon: BsGithub,
                        }
                    }
                    a {
                        href: "mailto:suiki547@gmail.com",
                        Icon {
                            width: 30,
                            height: 30,
                            fill: "white",
                            icon: BsEnvelopeFill,
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
