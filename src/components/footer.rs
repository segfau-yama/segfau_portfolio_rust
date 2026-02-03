use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::{BsDiscord, BsTwitter, BsGithub, BsEnvelopeFill};
use dioxus_free_icons::Icon;
use crate::components::Avatar;

#[derive(PartialEq, Clone, Props)]
pub struct FooterProps {
    color: String,
    size: String,
    children: Element,
}

// TODO: フッターコンポーネントの汎用性を上げる
#[component]
pub fn Footer(props: FooterProps) -> Element {
    rsx! {
        footer { class: "block mx-auto w-full {props.size} {props.color}",
            {props.children}
        }
    }
}
