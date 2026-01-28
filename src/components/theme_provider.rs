use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct ThemeProviderProps {
    theme: ReadSignal<String>,
}

#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    rsx! {
    }
}
