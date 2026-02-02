use crate::components::{Card, Avatar, Timeline, TimelineItem, Parallax, Row, Col, ScrollAnchor, Typography};
use crate::views::{TopView, ProfileView, HistoryView, WorkView};
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
            id: "history".to_string(),
            HistoryView {}
        }
        ScrollAnchor {
            id: "work".to_string(),
            WorkView {}
        }
    }
}
