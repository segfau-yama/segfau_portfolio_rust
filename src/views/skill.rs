use dioxus::prelude::*;
use crate::components::{Card, CardHeader, CardBody, Col, Row, Typography, ProgressBar, Flexbox};
use crate::logics::IconArc;
use hmziq_dioxus_free_icons::icons::fa_solid_icons::{FaDna};
use hmziq_dioxus_free_icons::icons::si_icons::{
    SiReact, SiVuedotjs, SiFastapi, SiPlatformio, SiArduino, SiEspressif, SiKicad, SiFreecad, SiDassaultsystemes, SiCplusplus, SiRust, SiPython, SiJavascript,
};
use hmziq_dioxus_free_icons::Icon;
use std::sync::Arc;

#[derive(PartialEq, Clone)]
pub struct CardData {
    title: &'static str,
    contents: Vec<SkillData>,
}

#[derive(PartialEq, Clone)]
pub struct SkillData {
    name: &'static str,
    icon: IconArc,
    percentage: &'static str,
    text_color: &'static str,
    bg_color: &'static str,
}

#[component]
pub fn SkillView() -> Element {
    let language_skills = vec![
        SkillData {
            name: "C/C++",
            icon: IconArc(Arc::new(SiCplusplus)),
            percentage: "w-[50%]",
            text_color: "text-sky-500",
            bg_color: "bg-sky-500",
        },
        SkillData {
            name: "Rust",
            icon: IconArc(Arc::new(SiRust)),
            percentage: "w-[20%]",
            text_color: "text-black",
            bg_color: "bg-black",
        },
        SkillData {
            name: "Python",
            icon: IconArc(Arc::new(SiPython)),
            percentage: "w-[40%]",
            text_color: "text-yellow-300",
            bg_color: "bg-yellow-300",
        },
        SkillData {
            name: "JavaScript",
            icon: IconArc(Arc::new(SiJavascript)),
            percentage: "w-[20%]",
            text_color: "text-yellow-500",
            bg_color: "bg-yellow-500",
        },
    ];
    let web_skills = vec![
        SkillData {
            name: "React",
            icon: IconArc(Arc::new(SiReact)),
            percentage: "w-[15%]",
            text_color: "text-cyan-500",
            bg_color: "bg-cyan-500",
        },
        SkillData {
            name: "Vue",
            icon: IconArc(Arc::new(SiVuedotjs)),
            percentage: "w-[40%]",
            text_color: "text-teal-800",
            bg_color: "bg-teal-800",
        },
        SkillData {
            name: "Dioxus",
            icon: IconArc(Arc::new(FaDna)),
            percentage: "w-[10%]",
            text_color: "text-orange-500",
            bg_color: "bg-orange-500",
        },
        SkillData {
            name: "FastAPI",
            icon: IconArc(Arc::new(SiFastapi)),
            percentage: "w-[30%]",
            text_color: "text-emerald-500",
            bg_color: "bg-emerald-500",
        },
    ];
    let embedded_skills = vec![
        SkillData {
            name: "PlatformIO",
            icon: IconArc(Arc::new(SiPlatformio)),
            percentage: "w-[30%]",
            text_color: "text-yellow-300",
            bg_color: "bg-yellow-300",
        },
        SkillData {
            name: "Arduino",
            icon: IconArc(Arc::new(SiArduino)),
            percentage: "w-[50%]",
            text_color: "text-cyan-500",
            bg_color: "bg-cyan-500",
        },
        SkillData {
            name: "Espressif",
            icon: IconArc(Arc::new(SiEspressif)),
            percentage: "w-[20%]",
            text_color: "text-red-500",
            bg_color: "bg-red-500",
        },
        SkillData {
            name: "Kicad",
            icon: IconArc(Arc::new(SiKicad)),
            percentage: "w-[50%]",
            text_color: "text-indigo-500",
            bg_color: "bg-indigo-500",
        },
    ];
    let design_skills = vec![
        SkillData {
            name: "FreeCAD",
            icon: IconArc(Arc::new(SiFreecad)),
            percentage: "w-[50%]",
            text_color: "text-red-500",
            bg_color: "bg-red-500",
        },
        SkillData {
            name: "SolidWorks",
            icon: IconArc(Arc::new(SiDassaultsystemes)),
            percentage: "w-[30%]",
            text_color: "text-black",
            bg_color: "bg-black",
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
                class: "mb-6",
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
                                    text: card.title,
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
                                        Col {
                                            class: "flex justify-between items-center",
                                            Flexbox {
                                                items: "items-center",
                                                justify: "justify-center",
                                                class: "items-center gap-x-1",
                                                Icon {
                                                    width: 20,
                                                    height: 20,
                                                    icon: skill.icon.clone(),
                                                    class: skill.text_color,
                                                }
                                                Typography {
                                                    text: skill.name,
                                                    size: "text-lg",
                                                    color: "text-black",
                                                }
                                            }
                                            Typography {
                                                text: skill.percentage.replace("w-[", "").replace("]", ""),
                                                size: "text-lg",
                                                color: "text-black",
                                                class: "p-1",
                                            }
                                        }
                                        Col {
                                            class: "flex justify-center",
                                            ProgressBar {
                                                percentage: skill.percentage,
                                                rounded: "rounded-s-md",
                                                class: skill.bg_color,
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
