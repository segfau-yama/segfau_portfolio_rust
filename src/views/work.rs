use dioxus::prelude::*;
use crate::components::{Card, CardHeader, CardBody, CardFooter, Col, Row, Typography};

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
        div { class: "mx-auto pt-20 px-4",
            Typography {
                text: "Work",
                size: "text-5xl",
                color: "text-black",
                position: "text-center",
                class: "font-bold mb-6",
            }
            Row {
                cols: "grid-cols-1",
                gap: "gap-4",
                class: "justify-center md:grid-cols-2 lg:grid-cols-3",
                for card in cards.iter() {
                    Col {
                        Card {
                            color: "white",
                            shadow: "shadow-sm",
                            rounded: "rounded-lg",
                            CardHeader { color: "h-white", size: "h-64",
                                img { alt: "card-image", src: {card.image} }
                            }
                            CardBody {
                                Typography {
                                    text: {card.title},
                                    size: "text-xl",
                                    color: "text-slate-800",
                                    position: "text-left",
                                    class: "my-2 font-semibold",
                                }
                                Typography {
                                    text: {card.text},
                                    size: "text-base",
                                    color: "text-slate-600",
                                    position: "text-left",
                                    class: "leading-normal",
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
