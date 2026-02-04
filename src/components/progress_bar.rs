use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ProgressBarProps {
    percentage: String,
    #[props(default="".to_string())]
    rounded: String,
    #[props(default="".to_string())]
    class: String,
    children: Element,
}

#[component]
pub fn ProgressBar(props: ProgressBarProps) -> Element {
    // State to control animation - starts at 0 width
    let mut is_visible = use_signal(|| false);
    
    // Trigger animation when component mounts
    use_effect(move || {
        // Schedule the animation to start after initial render
        #[cfg(target_arch = "wasm32")]
        {
            spawn(async move {
                // Small delay to allow initial render before animation
                use wasm_bindgen::prelude::*;
                let window = web_sys::window().unwrap();
                let promise = js_sys::Promise::new(&mut |resolve, _| {
                    window.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, 50).unwrap();
                });
                wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
                is_visible.set(true);
            });
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // For server-side rendering, set it immediately
            is_visible.set(true);
        }
    });
    
    // Determine the width class based on animation state
    let width_class = if is_visible() {
        props.percentage.clone()
    } else {
        "w-[0%]".to_string()
    };
    
    rsx! {
        div { class: "w-full",
            div { class: "flex-start flex h-5 w-full overflow-hidden font-sans text-xs font-medium",
                div { 
                    class: "flex items-center justify-center h-full overflow-hidden text-white break-all transition-all duration-1000 ease-out {props.rounded} {width_class} {props.class}", 
                    dir: "ltr",
                    {props.children}
                }
                div { class: "flex flex-auto bg-gray-300 {props.rounded}", dir: "rtl"}
            }
        }
    }
}

