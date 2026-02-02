use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct RowProps {
    gap: String,
    cols: String,
    #[props(default="".to_string())]
    class: String,
    children: Element,
}

#[component]
pub fn Row(props: RowProps) -> Element {

    rsx! {
        div { class: "grid gap-{props.gap} grid-cols-{props.cols} {props.class}",
            {props.children}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ColProps {
    #[props(default="".to_string())]
    class: String,
    children: Element,
}

#[component]
pub fn Col(props: ColProps) -> Element {
    rsx! {
        div { class: "{props.class}",
            {props.children}
        }
    }
}