use dioxus::prelude::*;


#[component]
pub fn Timeline(children: Element) -> Element {
    rsx! {

        div { class: "w-[32rem]",
            ul { class: "flex flex-col space-y-6",
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
            div { class: "flex gap-8 pb-8",
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

