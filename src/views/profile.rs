use dioxus::prelude::*;
use crate::components::{Avatar, Typography, Flexbox};

#[component]
pub fn ProfileView() -> Element {
    rsx! {
        div { class: "mx-auto pt-20 px-4",
            Typography {
                text: "Profile",
                size: "text-5xl",
                color: "text-black",
                position: "text-center",
                class: "mb-6",
            }
            Flexbox {
                justify: "justify-center",
                class: "flex-wrap gap-6",
                Flexbox {
                    items: "items-center",
                    justify: "justify-center",
                    Avatar {
                        image: "https://segfau-yama.github.io/segfau-portfolio/assets/segfau_icon-b657bf7d.webp",
                        rounded: "rounded-full",
                        size: "size-[300px]",
                        class: "shadow shadow-sm border-1 border-slate-200",
                    }
                }
                Flexbox {
                    items: "items-center",
                    justify: "justify-center",
                    Typography {
                        text: "青森県青森市生まれ宮城県仙台市育ちのポンコツです．
                        広く浅くをモットーにして生きているため全てが中途半端
                        プログラミング：Python, JavaScript, C, C++
                        回路設計：KiCad
                        機械設計：SolidWorks, Fusion360, FreeCAD",
                        size: "text-xl",
                        color: "black",
                        position: "left",
                        class: "whitespace-pre-line leading-relaxed space-y-1",
                    }
                }
            }
        }
    }
}
