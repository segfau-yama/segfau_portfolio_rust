use dioxus::prelude::*;


#[component]
pub fn Parallax() -> Element {
    rsx! {
        div { class: "relative h-screen overflow-hidden",
            div {
                class: "absolute inset-0 bg-no-repeat bg-cover bg-fixed parallax-bg brightness-75",
                style: "background-image: url('https://segfau-yama.github.io/segfau-portfolio/assets/top_image-212d7568.webp');",
            }
            div { class: "absolute inset-0 flex justify-center items-center",
                div { class: "text-center mt-4",
                    h1 { class: "text-5xl text-white mt-5", "Segfau-Lab" }
                    p { class: "text-xl text-white mt-5",
                        "やまやまのホームページ"
                    }
                }
            }
        }
    }
}
