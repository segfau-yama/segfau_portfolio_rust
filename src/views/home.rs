use crate::components::{Card, Avatar, Timeline, TimelineItem, Parallax};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        Parallax {}
        Avatar { image: "https://docs.material-tailwind.com/img/face-2.jpg", rounded: "200", size: "100px" }
        Timeline {
            TimelineItem {
                title: "Timeline Title Here",
                content: "The key to more success is to have a lot of pillows. Put it this way, it took me twenty five years to get these plants, twenty five years of blood sweat and tears, and I'm never giving up, I'm just getting started. I'm up to something. Fan luv."
            }
            TimelineItem {
                title: "Timeline Title Here",
                content: "The key to more success is to have a lot of pillows. Put it this way, it took me twenty five years to get these plants, twenty five years of blood sweat and tears, and I'm never giving up, I'm just getting started. I'm up to something. Fan luv."
            }
        }
        Card {}
    }
}
