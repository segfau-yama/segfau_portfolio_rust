use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ContainerProps {
    color: String,
    #[props(default="".to_string())]
    class: String,
    children: Element,
}

#[component]
pub fn Container(props: ContainerProps) -> Element {
    rsx! {
        div { class: "container {props.color} {props.class}",
            {props.children}
        }
    }
}
