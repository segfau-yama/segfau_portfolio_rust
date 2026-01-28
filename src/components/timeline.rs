use dioxus::prelude::*;
use crate::components::grid::{Row, Col};

#[derive(PartialEq, Clone, Props)]
pub struct TimelineProps {
    children: Element,
}

#[component]
pub fn Timeline(props: TimelineProps) -> Element {
    rsx! {
        Row {
            gap: "0",
            class: "justify-center",
            {props.children}
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct TimelineItemProps {
    time: String,
    history: String,
    hide: Option<String>,
}


#[component]
pub fn TimelineItem(props: TimelineItemProps) -> Element {
    rsx! {
        Col {
            cols: 5,
            class: "col-start-1 col-end-6",
            div { 
                class: "h-24 font-semibold text-2xl text-gray-700 flex items-center justify-end",
                {props.time}
            }
        }
        Col {
            cols: 2,
            class: "col-start-6 col-end-8",
            div { 
                class: "flex flex-col items-center h-full",
                div {
                    class: format!("border-3 flex-auto border-gray-300 {}", if props.hide == Some("top".to_string()) { "invisible" } else { "" })
                }
                div { 
                    class: "w-8 h-8 border-4 border-gray-300 rounded-full bg-emerald-500" }
                div {
                    class: format!("border-3 flex-auto border-gray-300 {}", if props.hide == Some("bottom".to_string()) { "invisible" } else { "" })
                }
            }
        }
        Col {
            cols: 5,
            class: "col-start-8 col-end-12",
            div { class: "h-full text-left sm:text-lg text-md flex flex-col items-start justify-center",
                { props.history.split('\n')
                    .map(|line| rsx! {
                        span { "{line}" br {} }
                    }) }
            }
        }
    }
}

