use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TypographyProps {
    text: String,
    size: String,
    color: String,
    position: String,
    #[props(default="".to_string())]
    class : String,
}

#[component]
pub fn Typography(props: TypographyProps) -> Element {
    rsx! {
        div {
            class: "{props.size} {props.color} {props.position} {props.class}",
            {props.text}
        }
    }
}