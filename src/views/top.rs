use dioxus::prelude::*;
use crate::components::{Parallax, Typography};

#[component]
pub fn TopView() -> Element {
    rsx! {
        Parallax {
            img: "url(https://segfau-yama.github.io/segfau-portfolio/assets/top_image-212d7568.webp)",
            height: "py-96",
            Typography {
                text: "Segfau-Lab",
                size: "text-5xl",
                color: "text-white",
                position: "text-center",
                class: "my-5",
            }
            Typography {
                text: "やまやまのホームページ",
                size: "text-xl",
                color: "text-white",
                position: "text-center",
            }
        }
    }
}