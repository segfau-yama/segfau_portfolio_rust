use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ParallaxProps {
    img: String,
    height: String,
    children: Element,
    class: Option<String>,
}

#[component]
pub fn Parallax(props: ParallaxProps) -> Element {
    rsx! {
        div {
            class: "bg-[{props.img}] bg-fixed bg-cover {props.height} {props.class.clone().unwrap_or_default()}",
            {props.children}
        }
    }
}
