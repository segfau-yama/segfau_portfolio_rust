use dioxus::prelude::*;
use crate::components::{Card, CardHeader, CardBody, CardFooter, Col, Row, Typography, ProgressBar};
use dioxus_free_icons::icons::fa_brands_icons::{FaReact, FaVuejs, FaRust, FaPython};
use dioxus_free_icons::icons::fa_solid_icons::{FaDna, FaC};
use dioxus_free_icons::Icon;

#[derive(PartialEq, Clone)]
pub struct CardData {
    title: &'static str,
    contents: Vec<SkillData>,
}

#[derive(PartialEq, Clone)]
pub struct SkillData {
    name: &'static str,
    icon: Element,
    percentage: &'static str,
    color: &'static str,
}

#[component]
pub fn SkillView() -> Element {
    let web_skills = vec![
        SkillData {
            name: "React",
            icon: rsx! {Icon { width: 20, height: 20, fill: "#61DBFB", icon: FaReact }},
            percentage: "w-[15%]",
            color: "bg-blue-500",
        },
        SkillData {
            name: "Vue",
            icon: rsx! {Icon { width: 20, height: 20, fill: "#42b883", icon: FaVuejs }},
            percentage: "w-[40%]",
            color: "bg-green-500",
        },
        SkillData {
            name: "Dioxus",
            icon: rsx! {Icon { width: 20, height: 20, fill: "#dea584", icon: FaDna }},
            percentage: "w-[10%]",
            color: "bg-orange-500",
        },
        SkillData {
            name: "FastAPI",
            icon: rsx! {Icon { width: 20, height: 20, fill: "#306998", icon: FaPython }},
            percentage: "w-[30%]",
            color: "bg-red-500",
        },
    ];
    let embedded_skills = vec![
        SkillData {
            name: "C",
            icon: rsx! {img { width: "20", height: "20", src: "https://raw.githubusercontent.com/simple-icons/simple-icons/6c937be50b303846e0a4b725948814c78c3da048/icons/c.svg" }},
            percentage: "w-[30%]",
            color: "bg-blue-500",
        },
        SkillData {
            name: "Arduino",
            icon: rsx! {img { width: "20", height: "20", src: "https://raw.githubusercontent.com/simple-icons/simple-icons/6c937be50b303846e0a4b725948814c78c3da048/icons/arduino.svg" }},
            percentage: "w-[50%]",
            color: "bg-cyan-500",
        },
        SkillData {
            name: "Espressif",
            icon: rsx! {img { width: "20", height: "20", src: "https://raw.githubusercontent.com/simple-icons/simple-icons/6c937be50b303846e0a4b725948814c78c3da048/icons/espressif.svg" }},
            percentage: "w-[50%]",
            color: "bg-red-500",
        },
        SkillData {
            name: "Kicad",
            icon: rsx! {img { width: "20", height: "20", src: "https://raw.githubusercontent.com/simple-icons/simple-icons/6c937be50b303846e0a4b725948814c78c3da048/icons/kicad.svg" }},
            percentage: "w-[50%]",
            color: "bg-yellow-500",
        },
    ];
    let design_skills = vec![
        SkillData {
            name: "FreeCAD",
            icon: rsx! {img { width: "20", height: "20", src: "https://raw.githubusercontent.com/simple-icons/simple-icons/6c937be50b303846e0a4b725948814c78c3da048/icons/freecad.svg" }},
            percentage: "w-[50%]",
            color: "bg-blue-500",
        },
        SkillData {
            name: "SolidWorks",
            icon: rsx! {},
            percentage: "w-[50%]",
            color: "bg-cyan-500",
        },
    ];
    let cards = use_signal(|| vec![
        CardData {
            title: "Web Development",
            contents: web_skills.clone(),
        },
        CardData {
            title: "Embedded Development",
            contents: embedded_skills.clone(),
        },
        CardData {
            title: "Design Development",
            contents: design_skills.clone(),
        },
    ]);
    rsx! {
        div { class: "mx-auto pt-20 px-4",
            Typography {
                text: "Skill",
                size: "text-5xl",
                color: "text-black",
                position: "text-center",
                class: "font-bold mb-6",
            }
            Row {
                cols: "grid-cols-1",
                gap: "gap-4",
                class: "justify-center lg:grid-cols-2",
                for card in cards.iter() {
                    Col {
                        Card {
                            color: "white",
                            shadow: "shadow-sm",
                            rounded: "rounded-lg",
                            CardHeader { color: "text-black", size: "h-auto",
                                Typography {
                                    text: {card.title},
                                    size: "text-3xl",
                                    color: "text-slate-800",
                                    class: "my-2 font-semibold text-center",
                                }
                            }
                            CardBody {
                                Row {
                                    cols: "grid-cols-1",
                                    gap: "gap-y-2",
                                    class: "justify-center items-center",
                                    for skill in card.contents.iter() {
                                        Col { class: "flex justify-between items-center",
                                            div { class: "flex justify-center items-center gap-x-1",
                                                div {
                                                    {skill.icon.clone()}
                                                }
                                                div {
                                                    Typography {
                                                        text: {skill.name},
                                                        size: "text-lg",
                                                        color: "text-black",
                                                    }
                                                }
                                            }
                                            Typography {
                                                text: {skill.percentage.replace("w-[", "").replace("]", "")},
                                                size: "text-lg",
                                                color: "text-black",
                                                class: "p-1",
                                            } 
                                        }
                                        Col {
                                            class: "flex justify-center",
                                            ProgressBar {
                                                percentage: {skill.percentage},
                                                rounded: "rounded-s-md",
                                                class: {skill.color},
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
