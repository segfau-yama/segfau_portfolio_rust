use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ProgressBarProps {
    percentage: String,
    #[props(default="".to_string())]
    class: String,
    children: Element,
}

#[component]
pub fn ProgressBar(props: ProgressBarProps) -> Element {
    rsx! {
        div { class: "w-full",
            div { class: "flex items-center justify-between gap-4 my-2",
                {props.children}
            }
            div { class: "flex-start flex h-2.5 w-full overflow-hidden font-sans text-xs font-medium",
                div {
                    class: "flex items-center justify-center h-full overflow-hidden text-white break-all rounded-l-full {props.percentage} {props.class}",
                }
                div { class: "flex flex-auto bg-gray-300 rounded-r-full" }
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct ProgressBarContentProps {
    children: Element,
}

#[component]
pub fn ProgressBarContent(props: ProgressBarContentProps) -> Element {
    rsx! {
        h6 { class: "block font-sans text-base antialiased font-semibold leading-relaxed tracking-normal text-blue-gray-900",
            {props.children}
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct ProgressBarPercentageProps {
    percentage: String,
    #[props(default="".to_string())]
    class: String,
}

#[component]
pub fn ProgressBarPercentage(props: ProgressBarPercentageProps) -> Element {
    rsx! {
        h6 { class: "block font-sans text-base antialiased font-semibold leading-relaxed tracking-normal text-blue-gray-900",
            {props.percentage.clone()}
        }
        div { class: "flex-start flex h-2.5 w-full overflow-hidden font-sans text-xs font-medium",
            div {
                class: "flex items-center justify-center h-full overflow-hidden text-white break-all rounded-l-full w-[{props.percentage.clone()}] {props.class}",
            }
            div { class: "flex flex-auto bg-gray-300 rounded-r-full" }
        }
    }
}
