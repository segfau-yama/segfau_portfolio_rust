use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ProgressBarProps {
    percentage: String,
    #[props(default="".to_string())]
    rounded: String,
    #[props(default="".to_string())]
    class: String,
    children: Element,
}

// TODO: progress barにtailwindでアニメーションを追加する。アニメーションはコンポーネントが表示された際にpercentageに応じて変化するようにする。
#[component]
pub fn ProgressBar(props: ProgressBarProps) -> Element {
    rsx! {
        div { class: "w-full",
            div { class: "flex-start flex h-5 w-full overflow-hidden font-sans text-xs font-medium",
                div { class: "flex items-center justify-center h-full overflow-hidden text-white break-all {props.rounded} {props.percentage} {props.class}", dir: "ltr",
                    {props.children}
                }
                div { class: "flex flex-auto bg-gray-300 {props.rounded}", dir: "rtl"}
            }
        }
    }
}

