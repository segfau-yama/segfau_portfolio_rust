use dioxus::prelude::*;
use crate::components::{Timeline, TimelineItem, Typography};

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
        div { class: "md:mx-auto justify-center pt-20 px-4",
            Typography {
                text: "History".to_string(),
                size: "5xl".to_string(),
                color: "black".to_string(),
                position: "center".to_string(),
                class: Some("font-bold mb-10".to_string()),
            }
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