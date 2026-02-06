use dioxus::prelude::*;
use crate::components::{Card, CardHeader, CardBody, CardFooter, Col, Row, Typography, ProgressBar};
use hmziq_dioxus_free_icons::icons::fa_solid_icons::{FaDna};
use hmziq_dioxus_free_icons::icons::si_icons::{
    SiReact, SiVuedotjs, SiFastapi, SiPlatformio, SiArduino, SiEspressif, SiKicad, SiFreecad, SiDassaultsystemes, SiCplusplus, SiRust, SiPython, SiJavascript, 
};
use hmziq_dioxus_free_icons::Icon;
use hmziq_dioxus_free_icons::IconShape;
use std::sync::Arc;

struct IconBox(Arc<dyn IconShape>);


impl IconShape for IconBox {
    fn view_box(&self) -> &str { 
        self.0.view_box() 
    }
    fn xmlns(&self) -> &str { 
        self.0.xmlns() 
    }
    fn child_elements(&self) -> Element {
        self.0.child_elements()
    }
    fn fill_and_stroke<'a>(&self, user_color: &'a str) -> (&'a str, &'a str, &'a str) {
        self.0.fill_and_stroke(user_color)
    }
    fn stroke_linecap(&self) -> &str { 
        self.0.stroke_linecap() 
    }
    fn stroke_linejoin(&self) -> &str { 
        self.0.stroke_linejoin() 
    }
}

impl Clone for IconBox {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl PartialEq for IconBox {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

#[derive(PartialEq, Clone)]
pub struct CardData {
    title: &'static str,
    contents: Vec<SkillData>,
}

#[derive(PartialEq, Clone)]
pub struct SkillData {
    name: &'static str,
    icon: IconBox,
    percentage: &'static str,
    color: &'static str,
}

#[component]
pub fn SkillView() -> Element {
    let language_skills = vec![
        SkillData {
            name: "C/C++",
            icon: IconBox(Arc::new(SiCplusplus)),
            percentage: "w-[50%]",
            color: "bg-sky-500",
        },
        SkillData {
            name: "Rust",
            icon: IconBox(Arc::new(SiRust)),
            percentage: "w-[20%]",
            color: "bg-black",
        },
        SkillData {
            name: "Python",
            icon: IconBox(Arc::new(SiPython)),
            percentage: "w-[40%]",
            color: "bg-yellow-300",
        },
        SkillData {
            name: "JavaScript",
            icon: IconBox(Arc::new(SiJavascript)),
            percentage: "w-[20%]",
            color: "bg-yellow-500",
        },
    ];
    let web_skills = vec![
        SkillData {
            name: "React",
            icon: IconBox(Arc::new(SiReact)),
            percentage: "w-[15%]",
            color: "bg-blue-500",
        },
        SkillData {
            name: "Vue",
            icon: IconBox(Arc::new(SiVuedotjs)),
            percentage: "w-[40%]",
            color: "bg-teal-800",
        },
        SkillData {
            name: "Dioxus",
            icon: IconBox(Arc::new(FaDna)),
            percentage: "w-[10%]",
            color: "bg-orange-500",
        },
        SkillData {
            name: "FastAPI",
            icon: IconBox(Arc::new(SiFastapi)),
            percentage: "w-[30%]",
            color: "bg-emerald-500",
        },
    ];
    let embedded_skills = vec![
        SkillData {
            name: "PlatformIO",
            icon: IconBox(Arc::new(SiPlatformio)),
            percentage: "w-[30%]",
            color: "bg-yellow-300",
        },
        SkillData {
            name: "Arduino",
            icon: IconBox(Arc::new(SiArduino)),
            percentage: "w-[50%]",
            color: "bg-cyan-500",
        },
        SkillData {
            name: "Espressif",
            icon: IconBox(Arc::new(SiEspressif)),
            percentage: "w-[20%]",
            color: "bg-red-500",
        },
        SkillData {
            name: "Kicad",
            icon: IconBox(Arc::new(SiKicad)),
            percentage: "w-[50%]",
            color: "bg-indigo-500",
        },
    ];
    let design_skills = vec![
        SkillData {
            name: "FreeCAD",
            icon: IconBox(Arc::new(SiFreecad)),
            percentage: "w-[50%]",
            color: "bg-red-500",
        },
        SkillData {
            name: "SolidWorks",
            icon: IconBox(Arc::new(SiDassaultsystemes)),
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
                                                    Icon {
                                                        width: 20,
                                                        height: 20,
                                                        icon: skill.icon.clone(),
                                                        class: "text-black",
                                                    }
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
