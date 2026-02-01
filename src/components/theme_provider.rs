use dioxus::prelude::*;
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct ColorTheme {
    pub primary: String,
    pub secondary: String,
    pub background: String,
    pub text: String,
    pub error: Option<String>,
    pub warning: Option<String>,
    pub success: Option<String>,
}

#[derive(Clone)]
pub struct ThemeProvider {
    themes: Signal<HashMap<String, ColorTheme>>,
    current: Signal<String>,
}

impl ThemeProvider {
    pub fn init(themes: HashMap<String, ColorTheme>, default: String) -> Self {
        let themes = use_signal(move || themes);
        let current = use_signal(move || default);

        let provider = Self { themes, current };
        use_context_provider(|| provider.clone());
        provider
    }

    pub fn use_theme() -> Self {
        use_context::<Self>()
    }

    pub fn current(&self) -> String {
        self.current()
    }

    pub fn set_current(&self, name: String) {
        self.current.set(name);
    }

    pub fn toggle(&self, a: &str, b: &str) {
        self.current.with_mut(|current| {
            *current = if current == a {
                b.to_string()
            } else {
                a.to_string()
            };
        });
    }

    pub fn theme(&self) -> Option<ColorTheme> {
        self.themes.read().get(&self.current()).cloned()
    }
}
