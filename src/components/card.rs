use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct CardProps {
    color: String,
    shadow: String,
    rounded: String,
    class: Option<String>,
    children: Element,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    rsx! {
        div { class: "relative flex flex-col border border-slate-200 {props.color} {props.shadow} {props.rounded} {props.class:?}",
            {props.children}
        }
    }
}


#[derive(PartialEq, Clone, Props)]
pub struct CardHeaderProps {
    color: String,
    size: String,
    class: Option<String>,
    children: Element,
}

#[component]
pub fn CardHeader(props: CardHeaderProps) -> Element {
    rsx! {
        div { 
            class: "relative overflow-hidden flex-auto text-{props.color} h-{props.size} {props.class:?}",
            {props.children}
        }
    }
}


#[derive(PartialEq, Clone, Props)]
pub struct CardBodyProps {
    children: Element,
}

#[component]
pub fn CardBody(props: CardBodyProps) -> Element {
    rsx! {
        div {
            class: "p-4",
            {props.children}
        }
        
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct CardFooterProps {
    children: Element,
}

#[component]
pub fn CardFooter(props: CardFooterProps) -> Element {
    rsx! {
        div {
            class: "px-4 pb-4 pt-0 mt-2",
            {props.children}
        }
    }
}