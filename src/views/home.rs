use crate::components::{Card, Avatar, Timeline, TimelineItem, Parallax, Row, Col};
use dioxus::prelude::*;

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    rsx! {
        Parallax {}
        ProfileView {}
        HistoryView {}
        WorkView {}
    }
}

#[component]
pub fn ProfileView() -> Element {
    rsx! {
        div {
            class: "mx-auto my-10 px-4 justify-center",

            h1 {
                class: "text-center text-5xl font-bold mb-6",
                "Profile"
            }
            div {
                class: "flex flex-wrap gap-6 justify-center",
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
    }
}


#[derive(PartialEq, Clone)]
pub struct TimelineData {
    time: &'static str,
    text: &'static str,
    hide: Option<&'static str>,
}

#[component]
pub fn HistoryView() -> Element {
    let histories = use_signal(|| vec![
        TimelineData { time: "2002/3", text: "青森県にて生まれる", hide: Some("top") },
        TimelineData { time: "2017/3", text: "仙台高等専門学校総合工学科Ⅰ類 入学", hide: None },
        TimelineData { time: "2019/1", text: "Web×Iotハッカソンメイカーズチャレンジ 参加", hide: None },
        TimelineData { time: "2019/12", text: "Thailand-Japan Student ICT Fair 参加", hide: None },
        TimelineData { time: "2022/3", text: "仙台高等専門学校総合工学科Ⅰ類 卒業", hide: None },
        TimelineData {
            time: "2022/4",
            text: "仙台高等専門学校総合工学科専攻科\n情報電子システム工学専攻 入学",
            hide: None,
        },
        TimelineData { time: "2022/7", text: "基本情報技術者 取得", hide: None },
        TimelineData { time: "2022/8", text: "第二種電気工事士 取得", hide: None },
        TimelineData { time: "2023/8", text: "日本高専学会学生優秀発表賞 受賞", hide: None },
        TimelineData { time: "2024/4", text: "製造業系の地元企業に就職", hide: None },
        TimelineData { time: "2026/1", text: "製造業系の地元企業を退職", hide: None },
        TimelineData { time: "2026/1", text: "SES系の企業に就職", hide: Some("bottom") },
    ]);

    rsx! {
        div { class: "container md:mx-auto my-10 justify-center px-4",
            h1 { class: "text-center text-5xl font-bold mb-10", "History" }
            Timeline {
                for history in histories.iter() {
                    TimelineItem {
                        time: history.time.to_string(),
                        history: history.text.to_string(),
                        hide: history.hide.map(|s| s.to_string()),
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
pub fn WorkView() -> Element {
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
            Row {
                gap: "4",
                class: "justify-center",
                for card in cards.iter() {
                    Col {
                        cols: 12,
                        class: "col-span-12 md:col-span-6 lg:col-span-4",
                        Card {
                            title: card.title.clone(),
                            text: card.text.clone(),
                            width: "w-full".to_string(),
                            image: card.image.clone(),
                        }
                    }
                }
            }
        }
    }
}
