use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TypographyProps {
    text: String,
    size: String,
    color: String,
    position: String,
    class : Option<String>,
}

#[component]
pub fn Typography(props: TypographyProps) -> Element {
    rsx! {
        div {
            class: "text-{props.size} text-{props.color} text-{props.position} {props.class.clone().unwrap_or_default()}",
            "{props.text}"
        }
    }
}