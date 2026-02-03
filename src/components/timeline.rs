use dioxus::prelude::*;
use crate::components::grid::{Row, Col};

#[derive(PartialEq, Clone, Props)]
pub struct TimelineProps {
    children: Element,
}

#[component]
pub fn Timeline(props: TimelineProps) -> Element {
    rsx! {
        Row { cols: "grid-cols-[4fr_1fr_4fr]", gap: "gap-0", {props.children} }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct TimelineItemProps {
    size: String,
    time: String,
    history: String,
    hide: Option<String>,
}


#[component]
pub fn TimelineItem(props: TimelineItemProps) -> Element {
    rsx! {
        Col {
            div { class: "{props.size} font-semibold text-2xl text-gray-700 flex items-center justify-end",
                {props.time}
            }
        }
        Col {
            div { class: "h-full flex flex-col items-center justify-center",
                div {
                    class: format!(
                        "border-3 flex-auto border-gray-300 {}",
                        if props.hide == Some("top".to_string()) { "invisible" } else { "" },
                    ),
                }
                div { class: "w-8 h-8 border-4 border-gray-300 rounded-full bg-emerald-500" }
                div {
                    class: format!(
                        "border-3 flex-auto border-gray-300 {}",
                        if props.hide == Some("bottom".to_string()) { "invisible" } else { "" },
                    ),
                }
            }
        }
        Col {
            div { class: "h-full text-sm sm:text-md md:text-lg whitespace-pre-line flex items-center justify-start",
                {props.history}
            }
        }
    }
}
