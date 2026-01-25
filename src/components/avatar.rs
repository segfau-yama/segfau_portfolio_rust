use dioxus::prelude::*;


#[component]
pub fn Avatar() -> Element {
    rsx! {
        div { class: "flex items-center gap-4",
            img {
                alt: "avatar",
                class: "inline-block relative object-cover object-center !rounded-full w-12 h-12",
                src: "https://docs.material-tailwind.com/img/face-2.jpg",
            }
            div {
                h6 { class: "text-slate-800 font-semibold", " Tania Andrew " }
                p { class: "text-slate-600 text-sm", " Web Developer " }
            }
        }
    }
}
