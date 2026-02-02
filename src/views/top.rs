use dioxus::prelude::*;
use crate::components::{Parallax, Typography};

#[component]
pub fn TopView() -> Element {
    rsx! {
        Parallax {
            img: "url(https://segfau-yama.github.io/segfau-portfolio/assets/top_image-212d7568.webp)", 
            height: "py-96".to_string(),
            Typography {
                text: "Segfau-Lab".to_string(),
                size: "5xl".to_string(),
                color: "white".to_string(),
                position: "center".to_string(),
                class: "my-5".to_string(),
            }
            Typography {
                text: "やまやまのホームページ".to_string(),
                size: "xl".to_string(),
                color: "white".to_string(),
                position: "center".to_string(),
                class: "mt-5".to_string(),
            }
        }
    }
}