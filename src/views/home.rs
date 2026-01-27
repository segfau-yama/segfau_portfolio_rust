use crate::components::{Card, Avatar, Timeline, TimelineItem, Parallax};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        Parallax {}
        div {
            class: "mx-auto my-10 justify-center",

            h1 {
                class: "text-center text-5xl font-bold mb-6",
                "Profile"
            }
            div {
                class: "flex justify-center sm:flex-row flex-col space-6",
                div {
                    class: "
                        flex
                        items-center
                        justify-center
                    ",
                    Avatar { image: "https://segfau-yama.github.io/segfau-portfolio/assets/segfau_icon-b657bf7d.webp", rounded: "full", size: "50px" }
                }
                div {
                    class: "
                        flex
                        items-center
                        justify-center
                    ",
                    div {
                        class: "text-left text-xl leading-relaxed space-y-1",
                        "青森県青森市生まれ宮城県仙台市育ちのポンコツです．"
                        br {}
                        "広く浅くをモットーにして生きているため全てが中途半端"
                        br {}
                        "プログラミング：Python, JavaScript, C, C++"
                        br {}
                        "回路設計：KiCad"
                        br {}
                        "機械設計：SolidWorks, Fusion360"
                    }
                }
            }
        }
        div {
            HistoryPage {}
            // class: "container mx-auto my-10 px-4 justify-center",
            // Timeline {
            //     TimelineItem {
            //         title: "Timeline Title Here",
            //         content: "The key to more success is to have a lot of pillows. Put it this way, it took me twenty five years to get these plants, twenty five years of blood sweat and tears, and I'm never giving up, I'm just getting started. I'm up to something. Fan luv."
            //     }
            //     TimelineItem {
            //         title: "Timeline Title Here",
            //         content: "The key to more success is to have a lot of pillows. Put it this way, it took me twenty five years to get these plants, twenty five years of blood sweat and tears, and I'm never giving up, I'm just getting started. I'm up to something. Fan luv."
            //     }
            // }
        }
        Card {}
    }
}


#[derive(Clone)]
struct History {
    time: &'static str,
    text: &'static str,
    top: bool,
    bottom: bool,
}

#[component]
pub fn HistoryPage() -> Element {
    let histories = use_signal(|| vec![
        History { time: "2002/3", text: "青森県にて生まれる", top: true, bottom: false },
        History { time: "2017/3", text: "仙台高等専門学校総合工学科Ⅰ類 入学", top: false, bottom: false },
        History { time: "2019/1", text: "Web×Iotハッカソンメイカーズチャレンジ 参加", top: false, bottom: false },
        History { time: "2019/12", text: "Thailand-Japan Student ICT Fair 参加", top: false, bottom: false },
        History { time: "2022/3", text: "仙台高等専門学校総合工学科Ⅰ類 卒業", top: false, bottom: false },
        History {
            time: "2022/4",
            text: "仙台高等専門学校総合工学科専攻科\n情報電子システム工学専攻 入学",
            top: false,
            bottom: false,
        },
        History { time: "2022/7", text: "基本情報技術者 取得", top: false, bottom: false },
        History { time: "2022/8", text: "第二種電気工事士 取得", top: false, bottom: false },
        History { time: "2023/8", text: "日本高専学会学生優秀発表賞 受賞", top: false, bottom: false },
        History { time: "2024/4", text: "製造業系の地元企業に就職", top: false, bottom: false },
        History { time: "2026/1", text: "製造業系の地元企業を退職", top: false, bottom: false },
        History { time: "2026/1", text: "SES系の企業に就職", top: false, bottom: true },
    ]);

    rsx! {
        div { class: "container mx-auto my-10 place-items-center",
            h1 { class: "text-center text-5xl font-bold mb-10", "History" }

            // gap を親で作ると“隙間”に線が出ないので、item 側の py で間隔を作る
            div { class: "flex flex-col space-6",

                // Signal<Vec<T>> は docs の通り .iter() で回せる
                for history in histories.iter() {
                    // key は安定したユニーク値を推奨（例では time を使用）
                    div { class: "grid grid-cols-[6rem_2rem_1fr] gap-6",

                        // time
                        div { class: "text-right font-semibold text-gray-700 pt-1 content-center",
                            {history.time}
                        }

                        // divider: 上線 / dot / 下線（Vuetify の VTimelineDivider 的）
                        div { class: "flex flex-col items-center self-stretch",
                            div {
                                class: format!("border-3 flex-1 border-gray-300 {}", if history.top { "invisible" } else { "" })
                            }
                            div { class: "w-8 h-8 border-4 border-gray-300 rounded-full bg-emerald-500" }
                            div {
                                class: format!("border-3 flex-1 border-gray-300 {}", if history.bottom { "invisible" } else { "" })
                            }
                        }

                        // content
                        div { class: "p-4 text-left",
                            {
                                history.text
                                    .split('\n')
                                    .map(|line| rsx! {
                                        span { "{line}" br {} }
                                    })
                            }
                        }
                    }
                }
            }
        }
    }
}
