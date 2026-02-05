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
        nav { class: "{props.size} {props.color} sticky top-0 z-[9999]",
            div { class: "container flex flex-wrap sm:justify-between sm:flex-row flex-col text-slate-50 max-w-screen-xl mx-auto",
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
        div { class: "flex justify-center sm:mr-4 cursor-pointer py-1 text-slate-50 font-semibold sm:text-xl text-2xl",
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
        ul { class: "flex justify-between items-center sm:gap-1 gap-4 px-4", {props.children} }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct HeaderItemProps {
    children: Element,
}
#[component]
pub fn HeaderItem(props: HeaderItemProps) -> Element {
    rsx! {
        li { class: "flex items-center p-1 sm:text-md text-sm gap-x-2 hover:text-slate-200 focus:text-slate-400",
            {props.children}
        }
    }
}

