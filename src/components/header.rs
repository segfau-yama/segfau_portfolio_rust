use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct HeaderProps {
    color: String,
    size: String,
    children: Element,
}

#[component]
pub fn Header(props: HeaderProps) -> Element {
    rsx! {
        nav { class: "{props.size} px-10 {props.color} sticky top-0 z-[9999]",
            div { class: "container flex flex-wrap sm:justify-between items-center sm:flex-row flex-col text-slate-50 max-w-screen-xl mx-auto",
                {props.children}
            }
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct HeaderTitleProps {
    children: Element,
}
#[component]
pub fn HeaderTitle(props: HeaderTitleProps) -> Element {
    rsx! {
        div {
            class: "mr-4 cursor-pointer py-1.5 text-slate-50 font-semibold sm:text-xl text-2xl",
            {props.children}
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct HeaderItemWrapperProps {
    children: Element,
}
#[component]
pub fn HeaderItemWrapper(props: HeaderItemWrapperProps) -> Element {
    rsx! {
        ul { class: "flex flex-col gap-2 mb-0 mt-0 flex-row items-center gap-6",
            {props.children}
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct HeaderItemProps {
    children: Element,
}
#[component]
pub fn HeaderItem(props: HeaderItemProps) -> Element {
    rsx! {
        li { class: "flex items-center p-1 text-md gap-x-2 hover:text-slate-200 focus:text-slate-400",
            {props.children}
        }
    }
}

