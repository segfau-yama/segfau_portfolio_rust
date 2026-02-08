use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct FlexboxProps {
    #[props(default="".to_string())]
    color: String,
    #[props(default="".to_string())]
    items: String,
    #[props(default="".to_string())]
    justify: String,
    #[props(default="".to_string())]
    class: String,
    children: Element,
}

#[component]
pub fn Flexbox(props: FlexboxProps) -> Element {
    rsx! {
        div {
            class: "flex {props.items} {props.justify} {props.color} {props.class}",
            {props.children}
        }
    }
}
