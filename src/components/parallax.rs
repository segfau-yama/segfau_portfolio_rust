use dioxus::prelude::*;


#[component]
pub fn Parallax() -> Element {
    rsx! {
        div { class: "relative h-screen overflow-hidden",
            div {
                class: "absolute inset-0 bg-no-repeat bg-cover bg-fixed parallax-bg",
                style: "background-image: url('https://media.geeksforgeeks.org/wp-content/uploads/20240322101847/Default_An_illustration_depictin-(2)-660.jpg');",
            }
            div { class: "absolute inset-0 flex justify-center items-center",
                div { class: "text-center mt-4",
                    h1 { class: "text-3xl text-white mt-5", "Welcome to Tailwind CSS Parallax Effect" }
                    p { class: "text-lg text-white mt-5 ml-4",
                        "Elevate your web design with stunning parallax effects using Tailwind CSS."
                    }
                }
            }
        }
    }
}
