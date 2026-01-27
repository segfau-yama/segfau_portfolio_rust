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
        nav { class: "block w-full px-4 lg:px-8 {props.size} mx-auto {props.color} bg-opacity-10 sticky top-0 shadow backdrop-blur-lg backdrop-saturate-150 z-[9999]",
            div { class: "container flex flex-wrap items-center justify-between mx-auto text-slate-50",
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
            class: "mr-4 block cursor-pointer py-1.5 text-base text-slate-50 font-semibold",
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
        div { class: "block",
            ul { class: "flex flex-col gap-2 mb-0 mt-0 flex-row items-center gap-6",
                {props.children}
            }
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
        li { class: "flex items-center p-1 text-sm gap-x-2 hover:text-slate-200 focus:text-slate-400",
            {props.children}
        }
    }
}

