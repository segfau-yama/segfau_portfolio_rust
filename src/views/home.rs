use crate::components::{Card, Avatar, Timeline, TimelineItem, Parallax};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        Parallax {}
        div {
            class: "mx-auto my-10 px-4 justify-center",

            h1 {
                class: "text-center text-5xl font-bold mb-6",
                "Profile"
            }
            div {
                class: "flex justify-center flex-wrap space-6",
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
        HistoryPage {}
        WorkPage {}
    }
}


#[derive(PartialEq, Clone)]
pub struct History {
    time: &'static str,
    text: &'static str,
    hide: Option<&'static str>,
}

#[component]
pub fn HistoryPage() -> Element {
    let histories = use_signal(|| vec![
        History { time: "2002/3", text: "青森県にて生まれる", hide: Some("top") },
        History { time: "2017/3", text: "仙台高等専門学校総合工学科Ⅰ類 入学", hide: None },
        History { time: "2019/1", text: "Web×Iotハッカソンメイカーズチャレンジ 参加", hide: None },
        History { time: "2019/12", text: "Thailand-Japan Student ICT Fair 参加", hide: None },
        History { time: "2022/3", text: "仙台高等専門学校総合工学科Ⅰ類 卒業", hide: None },
        History {
            time: "2022/4",
            text: "仙台高等専門学校総合工学科専攻科\n情報電子システム工学専攻 入学",
            hide: None,
        },
        History { time: "2022/7", text: "基本情報技術者 取得", hide: None },
        History { time: "2022/8", text: "第二種電気工事士 取得", hide: None },
        History { time: "2023/8", text: "日本高専学会学生優秀発表賞 受賞", hide: None },
        History { time: "2024/4", text: "製造業系の地元企業に就職", hide: None },
        History { time: "2026/1", text: "製造業系の地元企業を退職", hide: None },
        History { time: "2026/1", text: "SES系の企業に就職", hide: Some("bottom") },
    ]);

    rsx! {
        div { class: "container md:mx-auto my-10 justify-between px-4",
            h1 { class: "text-center text-5xl font-bold mb-10", "History" }

            // gap を親で作ると“隙間”に線が出ないので、item 側の py で間隔を作る
            div { class: "flex flex-col",

                // Signal<Vec<T>> は docs の通り .iter() で回せる
                for history in histories.iter() {
                    // key は安定したユニーク値を推奨（例では time を使用）
                    div { class: "flex flex-row md:h-24 h-32",

                        // time
                        div { class: "block flex-3 text-right font-semibold text-2xl text-gray-700 content-center",
                            {history.time.clone()}
                        }

                        // divider: 上線 / dot / 下線（Vuetify の VTimelineDivider 的）
                        div { class: "block flex flex-col flex-1 items-center content-center",
                            div {
                                class: format!("border-3 flex-auto border-gray-300 {}", if history.hide == Some("top") { "invisible" } else { "" })
                            }
                            div { class: "w-8 h-8 border-4 border-gray-300 rounded-full bg-emerald-500" }
                            div {
                                class: format!("border-3 flex-auto border-gray-300 {}", if history.hide == Some("bottom") { "invisible" } else { "" })
                            }
                        }

                        // content
                        div { class: "block text-left text-lg flex-3 content-center",
                            {
                                history.text.clone()
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

#[derive(PartialEq, Clone)]
pub struct CardData {
    title: &'static str,
    text: &'static str,
    image: &'static str,
}

#[component]
pub fn WorkPage() -> Element {
    let cards = use_signal(|| vec![
        CardData {
            title: "im920s_arduino",
            text: 
            "
                Arduino UNOとIM920sでシリアル通信を行うスケッチ。
                PS3コントローラとの接続、シリアルモニタからのコマンド打ち込みが可能。
            ",
            image: "https://segfau-yama.github.io/segfau-portfolio/assets/im920s-eddeb179.webp",
        },
        CardData {
            title: "YmYm Omuni",
            text: 
            "
                DCモータで動く三輪オムニラジコン。
                機体は3dプリンタパーツとテクセルで作成。
            ",
            image: "https://segfau-yama.github.io/segfau-portfolio/assets/ymym_omuni-0e4e8139.webp",
        },
        CardData {
            title: "Motor Control Board",
            text: 
            "
                4つのDCモーターを制御できるボード。
                ESP32を利用しているためbluetoothコントローラと通信が可能。
            ",
            image: "https://segfau-yama.github.io/segfau-portfolio/assets/mcb-909c3c56.webp",
        },
        CardData {
            title: "MagDet",
            text: 
            "
                温泉旅館の空き情報をWeb上に表示するIoT機器
                M5Stackで施錠検知を行っている
            ",
            image: "https://segfau-yama.github.io/segfau-portfolio/assets/magdet-e429c2f5.webp",
        },
        CardData {
            title: "NPCB(National Power Calc Bot)",
            text: 
            "
                架空国家での国力計算を自動化するbot。
                ニコニコの音楽再生機能部分のコード分離予定。
            ",
            image: "https://segfau-yama.github.io/segfau-portfolio/assets/npcb-4cf22eca.webp",
        },
        CardData {
            title: "NW Osero",
            text: 
            "
                同時に複数人対戦可能な通信型オセロゲーム。
                LAN内での対戦のみ対応。
            ",
            image: "data:image/webp;base64,UklGRn4AAABXRUJQVlA4THEAAAAvf8JPAA8woBHzHwa2kSQ1wsMkBEIlZUwNptYvNqL/itw2UjKnWWboE8gL0GJXBjQR7X/baWJsXQH85wFjfroj/Of+wF0p3NWGqVKYqg2VpVDZH93rjhNy+svW37rv7xyC/1wQutcd9ysJ/Edca92RAAA=",
        },
    ]);
    rsx! {
        div { class: "mx-auto my-10 px-4",
            h1 {
                class: "text-center text-5xl font-bold mb-6",
                "Work"
            }
            div {
                class: "flex flex-wrap justify-center",
                for card in cards.iter() {
                    Card {
                        title: card.title.clone(),
                        text: card.text.clone(),
                        width: "w-full md:w-1/2 lg:w-1/3".to_string(),
                        image: card.image.clone(),
                    }
                }
            }
            
        }
    }
}
