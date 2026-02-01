use dioxus::prelude::*;
use crate::components::{Avatar, Typography};

#[component]
pub fn ProfileView() -> Element {
    rsx! {
        div {
            class: "mx-auto pt-20 px-4 justify-center",
            Typography {
                text: "Profile".to_string(),
                size: "5xl".to_string(),
                color: "black".to_string(),
                position: "center".to_string(),
                class: Some("font-bold mb-6".to_string()),
            }
            div {
                class: "flex flex-wrap gap-6 justify-center",
                div {
                    class: "
                        flex
                        items-center
                        justify-center
                    ",
                    Avatar { image: "https://segfau-yama.github.io/segfau-portfolio/assets/segfau_icon-b657bf7d.webp", rounded: "full", size: "50px" }
                }
                div {
                    class: "
                        flex
                        items-center
                        justify-center
                    ",
                    Typography {
                        text: 
                        "青森県青森市生まれ宮城県仙台市育ちのポンコツです．
                        広く浅くをモットーにして生きているため全てが中途半端
                        プログラミング：Python, JavaScript, C, C++
                        回路設計：KiCad
                        機械設計：SolidWorks, Fusion360, FreeCAD
                        ".to_string(),
                        size: "xl".to_string(),
                        color: "black".to_string(),
                        position: "left".to_string(),
                        class: Some("whitespace-pre-line leading-relaxed space-y-1".to_string()),
                    }
                }
            }
        }
    }
}