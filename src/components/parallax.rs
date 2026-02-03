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
        div { class: "bg-[url(https://segfau-yama.github.io/segfau-portfolio/assets/top_image-212d7568.webp)] bg-fixed bg-cover {props.height} {props.class:?}",
            {props.children}
        }
    }
}
