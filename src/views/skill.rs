use dioxus::prelude::*;
use crate::components::{Card, CardHeader, CardBody, CardFooter, Col, Row, Typography, ProgressBar, ProgressBarContent};
use dioxus_free_icons::icons::fa_brands_icons::{FaReact, FaVuejs};
use dioxus_free_icons::icons::fa_solid_icons::FaDna;
use dioxus_free_icons::Icon;

#[derive(PartialEq, Clone)]
pub struct CardData {
    title: &'static str,
    text: &'static str,
}

#[component]
pub fn SkillView() -> Element {
    let cards = use_signal(|| vec![
        CardData {
            title: "Frontend",
            text: "a",
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
                class: "justify-center md:grid-cols-2 lg:grid-cols-3",
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
                                    position: "text-center",
                                    class: "my-2 font-semibold",
                                }
                            }
                            CardBody {
                                ProgressBar {
                                    percentage: "w-[90%]",
                                    class: "bg-blue-500",
                                    ProgressBarContent {
                                        div { class: "flex items-center gap-2",
                                            Icon {
                                                width: 20,
                                                height: 20,
                                                fill: "black",
                                                icon: FaReact,
                                            }
                                            div { "React" }
                                        }
                                    }
                                    ProgressBarContent { "90%" }
                                }
                                ProgressBar {
                                    percentage: "w-[30%]",
                                    class: "bg-green-500",
                                    ProgressBarContent {
                                        div { class: "flex items-center gap-2",
                                            Icon {
                                                width: 20,
                                                height: 20,
                                                fill: "black",
                                                icon: FaVuejs,
                                            }
                                            div { "Vue" }
                                        }
                                    }
                                    ProgressBarContent { "30%" }
                                }
                                ProgressBar {
                                    percentage: "w-[75%]",
                                    class: "bg-slate-500",
                                    ProgressBarContent {
                                        div { class: "flex items-center gap-2",
                                            Icon {
                                                width: 20,
                                                height: 20,
                                                fill: "black",
                                                icon: FaDna,
                                            }
                                            div { "Dioxus" }
                                        }
                                    }
                                    ProgressBarContent { "75%" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
