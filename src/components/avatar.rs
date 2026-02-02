use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct AvatarProps {
    image: String,
    size: String,
    rounded: String,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    rsx! {
        div { class: "flex items-center justify-center gap-4",
            img {
                alt: "avatar",
                class: "rounded-{props.rounded} size-[{props.size}]",
                src: props.image,
            }
        }
    }
}
