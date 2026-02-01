use dioxus::html::geometry::PixelsVector2D;
use dioxus::prelude::*;
use std::{collections::HashMap, rc::Rc};

#[derive(Clone)]
pub struct ScrollHandle {
    pub anchors: Signal<HashMap<String, Rc<MountedData>>>,
}

impl ScrollHandle {
    /// ScrollHandle を Context から取得する（Provider必須）
    pub fn use_scroll() -> Self {
        use_context::<Self>()
    }

    /// App側（共通祖先）で1回だけ呼んで Provider を確立する
    pub fn init() -> Self {
        let anchors: Signal<HashMap<String, Rc<MountedData>>> = use_signal(HashMap::new);

        let handle = Self {
            anchors,
        };

        use_context_provider(|| handle.clone());
        handle
    }

    /// アンカー登録（onmounted で呼ぶ）
    pub fn register_anchor(&mut self, id: String, event: MountedEvent) {
        self.anchors.write().insert(id, event.data());
    }

    /// header補正付きでスクロール（scroll方式）
    pub async fn to_anchor(&self, id: &str) {
        let Some(target) = self.anchors.read().get(id).cloned() else {
            return;
        };
        target.scroll_to(ScrollBehavior::Smooth).await.unwrap();
    }

    /// 後方互換用の名前
    pub async fn scroll_to_anchor(&self, id: &str) {
        self.to_anchor(id).await;
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct ScrollAnchorProps {
    id: String,
    children: Element,
}

#[component]
pub fn ScrollAnchor(props: ScrollAnchorProps) -> Element {
    let mut scroll = ScrollHandle::use_scroll();

    rsx! {
        div {
            onmounted: move |event: MountedEvent| scroll.register_anchor(props.id.clone(), event),
            {props.children}
        }
    }
}

#[derive(PartialEq, Clone, Props)]
pub struct ScrollLinkProps {
    to: String,
    name: String,
}

#[component]
pub fn ScrollLink(props: ScrollLinkProps) -> Element {
    let mut scroll = ScrollHandle::use_scroll();

    rsx! {
        button {
            onclick: move |_| {
                let scroll = scroll.clone();
                let to = props.to.clone();
                spawn(async move {
                    scroll.to_anchor(&to).await;
                });
            },
            "{props.name}"
        }
    }
}
