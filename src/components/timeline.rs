use dioxus::prelude::*;


#[component]
pub fn Timeline(children: Element) -> Element {
    rsx! {

        div { class: "w-[32rem]",
            ul { class: "flex flex-col w-full",
                {children}
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct TimelineItemProps {
    title: String,
    content: String,
}


#[component]
pub fn TimelineItem(props: TimelineItemProps) -> Element {
    rsx! {
        li { class: "relative flex flex-col gap-2",
            span { class: "absolute left-0 grid justify-center transition-opacity duration-200 bg-transparent",
                span { class: "h-full w-0.5 bg-blue-gray-100" }
            }
            div { class: "flex items-center h-3 gap-4",
                span { class: "relative z-[2] w-max flex-shrink-0 overflow-hidden rounded-full bg-gray-900 p-1.5 text-white" }
                h6 { class: "block font-sans text-base antialiased font-semibold leading-none tracking-normal text-blue-gray-900",
                    {props.title}
                }
            }
            div { class: "flex gap-4 pb-8",
                span { class: "flex-shrink-0 invisible h-full pointer-events-none" }
                div {
                    p { class: "block font-sans text-sm antialiased font-normal leading-normal text-gray-600",
                        {props.content}
                    }
                }
            }
        }
    }
}

#[component]
pub fn TimelineTest() -> Element {
    rsx! {

        div { class: "w-[32rem]",
            ul { class: "flex flex-col w-full",
                li { class: "relative flex flex-col gap-2",
                    span { class: "absolute left-0 grid justify-center transition-opacity duration-200 bg-transparent",
                        span { class: "h-full w-0.5 bg-blue-gray-100" }
                    }
                    div { class: "flex items-center h-3 gap-4",
                        span { class: "relative z-[2] w-max flex-shrink-0 overflow-hidden rounded-full bg-gray-900 p-1.5 text-white" }
                        h6 { class: "block font-sans text-base antialiased font-semibold leading-none tracking-normal text-blue-gray-900",
                            " Timeline Title Here. "
                        }
                    }
                    div { class: "flex gap-4 pb-8",
                        span { class: "flex-shrink-0 invisible h-full pointer-events-none" }
                        div {
                            p { class: "block font-sans text-sm antialiased font-normal leading-normal text-gray-600",
                                " The key to more success is to have a lot of pillows. Put it this way, it took me twenty five years to get these plants, twenty five years of blood sweat and tears, and I'm never giving up, I'm just getting started. I'm up to something. Fan luv. "
                            }
                        }
                    }
                }
                li { class: "relative flex flex-col gap-2",
                    span { class: "absolute left-0 grid justify-center transition-opacity duration-200 bg-transparent",
                        span { class: "h-full w-0.5 bg-blue-gray-100" }
                    }
                    div { class: "flex items-center h-3 gap-4",
                        span { class: "relative z-[2] w-max flex-shrink-0 overflow-hidden rounded-full bg-gray-900 p-1.5 text-white" }
                        h6 { class: "block font-sans text-base antialiased font-semibold leading-none tracking-normal text-blue-gray-900",
                            " Timeline Title Here. "
                        }
                    }
                    div { class: "flex gap-4 pb-8",
                        span { class: "flex-shrink-0 invisible h-full pointer-events-none" }
                        div {
                            p { class: "block font-sans text-sm antialiased font-normal leading-normal text-gray-600",
                                " The key to more success is to have a lot of pillows. Put it this way, it took me twenty five years to get these plants, twenty five years of blood sweat and tears, and I'm never giving up, I'm just getting started. I'm up to something. Fan luv. "
                            }
                        }
                    }
                }
                li { class: "relative flex flex-col gap-2",
                    div { class: "flex items-center h-3 gap-4",
                        span { class: "relative z-[2] w-max flex-shrink-0 overflow-hidden rounded-full bg-gray-900 p-1.5 text-white" }
                        h6 { class: "block font-sans text-base antialiased font-semibold leading-none tracking-normal text-blue-gray-900",
                            " Timeline Title Here. "
                        }
                    }
                    div { class: "flex gap-4",
                        span { class: "flex-shrink-0 invisible h-full pointer-events-none" }
                        div {
                            p { class: "block font-sans text-sm antialiased font-normal leading-normal text-gray-600",
                                " The key to more success is to have a lot of pillows. Put it this way, it took me twenty five years to get these plants, twenty five years of blood sweat and tears, and I'm never giving up, I'm just getting started. I'm up to something. Fan luv. "
                            }
                        }
                    }
                }
            }
        }
    }
}
