use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct AvatarProps {
    image: String,
    size: String,
    rounded: String,
    #[props(default="".to_string())]
    class: String,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    rsx! {
        div { class: "flex items-center justify-center",
            img {
                alt: "avatar",
                class: "{props.rounded} {props.size} {props.class}",
                src: props.image,
            }
        }
    }
}
