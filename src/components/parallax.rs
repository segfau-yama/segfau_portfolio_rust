use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ParallaxProps {
    img: String,
}

#[component]
pub fn Parallax(props: ParallaxProps) -> Element {
    rsx! {
            div {
                class: "bg-[url('{props.img}')] bg-fixed bg-cover",
                div { class: "text-center py-96",
                    h1 { class: "text-5xl text-white mt-5", "Segfau-Lab" }
                    p { class: "text-xl text-white mt-5",
                        "やまやまのホームページ"
                    }
                }
            }
    }
}
