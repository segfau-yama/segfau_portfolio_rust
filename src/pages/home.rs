use crate::components::{ScrollAnchor};
use crate::views::{TopView, ProfileView, SkillView, HistoryView, WorkView};
use dioxus::prelude::*;


#[component]
pub fn Home() -> Element {
    rsx! {
        ScrollAnchor {
            id: "home".to_string(),
            TopView {}
        }
        ScrollAnchor {
            id: "profile".to_string(),
            ProfileView {}
        }
        ScrollAnchor {
            id: "skill".to_string(),
            SkillView {}
        }
        ScrollAnchor {
            id: "history".to_string(),
            HistoryView {}
        }
        ScrollAnchor {
            id: "work".to_string(),
            WorkView {}
        }
    }
}
