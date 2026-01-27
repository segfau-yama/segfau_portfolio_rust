use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct CardProps {
    title: String,
    text: String,
    width: String,
    image: String,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    rsx! {
        div { class: "relative flex flex-col bg-white shadow-sm border border-slate-200 rounded-lg h-96 md:h-80 lg:h-64",
            div { class: "relative overflow-hidden text-white flex-auto",
                img {
                    alt: "card-image",
                    src: {props.image},
                }
            }
            div { class: "p-4",
                h6 { class: "mb-2 text-slate-800 text-xl font-semibold", {props.title} }
                p { class: "text-slate-600 leading-normal font-light",
                    {props.text}
                }
            }
        }
    }
}
