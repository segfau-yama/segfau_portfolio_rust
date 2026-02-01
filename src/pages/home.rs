use crate::components::{Card, Avatar, Timeline, TimelineItem, Parallax, Row, Col, ScrollAnchor, Typography};
use crate::views::{ProfileView, HistoryView, WorkView};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        ScrollAnchor {
            id: "home".to_string(),
            Parallax {
                img: "url(https://segfau-yama.github.io/segfau-portfolio/assets/top_image-212d7568.webp)", 
                height: "py-96".to_string(),
                Typography {
                     text: "Segfau-Lab".to_string(),
                     size: "5xl".to_string(),
                     color: "white".to_string(),
                     position: "center".to_string(),
                     class: Some("mt-5".to_string()),
                }
                Typography {
                     text: "やまやまのホームページ".to_string(),
                     size: "xl".to_string(),
                     color: "white".to_string(),
                     position: "center".to_string(),
                     class: Some("mt-5".to_string()),
                }
            }
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
