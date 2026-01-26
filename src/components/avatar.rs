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
        div { class: "flex items-center gap-4",
            img {
                alt: "avatar",
                class: "inline-block relative object-cover object-center !rounded-[{props.rounded}] w-[{props.size}] h-[{props.size}]",
                src: props.image,
            }
        }
    }
}
