use dioxus::prelude::*;
use crate::components::{Card, CardHeader, CardBody, CardFooter, Col, Row, Typography, ProgressBar};
use hmziq_dioxus_free_icons::icons::fa_solid_icons::{FaDna};
use hmziq_dioxus_free_icons::icons::si_icons::{
    SiReact, SiVuedotjs, SiFastapi, SiPlatformio, SiArduino, SiEspressif, SiKicad, SiFreecad, SiDassaultsystemes, SiCplusplus, SiRust, SiPython, SiJavascript, 
};
use hmziq_dioxus_free_icons::Icon;

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
    let language_skills = vec![
        SkillData {
            name: "C/C++",
            icon: rsx! {Icon { width: 20, height: 20, icon: SiCplusplus, class: "text-sky-500" }},
            percentage: "w-[50%]",
            color: "bg-sky-500",
        },
        SkillData {
            name: "Rust",
            icon: rsx! {Icon { width: 20, height: 20, icon: SiRust, class: "text-black" }},
            percentage: "w-[20%]",
            color: "bg-black",
        },
        SkillData {
            name: "Python",
            icon: rsx! {Icon { width: 20, height: 20, icon: SiPython, class: "text-yellow-300" }},
            percentage: "w-[40%]",
            color: "bg-yellow-300",
        },
        SkillData {
            name: "JavaScript",
            icon: rsx! {Icon { width: 20, height: 20, icon: SiJavascript, class: "text-yellow-500" }},
            percentage: "w-[20%]",
            color: "bg-yellow-500",
        },
    ];
    let web_skills = vec![
        SkillData {
            name: "React",
            icon: rsx! {Icon { width: 20, height: 20, icon: SiReact, class: "text-blue-500" }},
            percentage: "w-[15%]",
            color: "bg-blue-500",
        },
        SkillData {
            name: "Vue",
            icon: rsx! {Icon { width: 20, height: 20, icon: SiVuedotjs, class: "text-teal-800" }},
            percentage: "w-[40%]",
            color: "bg-teal-800",
        },
        SkillData {
            name: "Dioxus",
            icon: rsx! {Icon { width: 20, height: 20, icon: FaDna, class: "text-orange-500" }},
            percentage: "w-[10%]",
            color: "bg-orange-500",
        },
        SkillData {
            name: "FastAPI",
            icon: rsx! {Icon { width: 20, height: 20, icon: SiFastapi, class: "text-emerald-500" }},
            percentage: "w-[30%]",
            color: "bg-emerald-500",
        },
    ];
    let embedded_skills = vec![
        SkillData {
            name: "PlatformIO",
            icon: rsx! {Icon { width: 20, height: 20, icon: SiPlatformio, class: "text-yellow-300" }},
            percentage: "w-[30%]",
            color: "bg-yellow-300",
        },
        SkillData {
            name: "Arduino",
            icon: rsx! {Icon { width: 20, height: 20, icon: SiArduino, class: "text-cyan-500" }},
            percentage: "w-[50%]",
            color: "bg-cyan-500",
        },
        SkillData {
            name: "Espressif",
            icon: rsx! {Icon { width: 20, height: 20, icon: SiEspressif, class: "text-red-500" }},
            percentage: "w-[20%]",
            color: "bg-red-500",
        },
        SkillData {
            name: "Kicad",
            icon: rsx! {Icon { width: 20, height: 20, icon: SiKicad, class: "text-indigo-500" }},
            percentage: "w-[50%]",
            color: "bg-indigo-500",
        },
    ];
    let design_skills = vec![
        SkillData {
            name: "FreeCAD",
            icon: rsx! {Icon { width: 20, height: 20, icon: SiFreecad, class: "text-red-500" }},
            percentage: "w-[50%]",
            color: "bg-red-500",
        },
        SkillData {
            name: "SolidWorks",
            icon: rsx! {Icon { width: 20, height: 20, icon: SiDassaultsystemes, class: "text-black" }},
            percentage: "w-[30%]",
            color: "bg-black",
        },
    ];

    let cards = use_signal(|| vec![
        CardData {
            title: "Programming Languages",
            contents: language_skills.clone(),
        },
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
                class: "justify-center md:grid-cols-2",
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
                                size: "h-auto",
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
