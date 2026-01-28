use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct RowProps {
    gap: String,
    class: String,
    children: Element,
}

#[component]
pub fn Row(props: RowProps) -> Element {
    let class = format!(
        "grid grid-cols-12 gap-{} {}",
        props.gap,
        props.class
    );

    rsx! {
        div { class: "{class}",
            {props.children}
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ColProps {
    cols: u8,
    class: String,
    children: Element,
}

#[component]
pub fn Col(props: ColProps) -> Element {
    // cols=1..12 を想定
    let span = props.cols.clamp(1, 12);

    let class = format!(
        "col-span-{} {}",
        span,
        props.class
    );

    rsx! {
        div { class: "{class}",
            {props.children}
        }
    }
}