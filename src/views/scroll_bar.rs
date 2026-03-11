use dioxus::{logger::tracing, prelude::*};

#[derive(Debug, Default, Clone, Copy)]
pub struct ScrollInfo {
    pub top: i32,
    pub height: i32,
    pub screen: i32,
}

pub fn on_mount_scrollable(evt: Event<MountedData>, mut scroll: Signal<ScrollInfo>) {
    spawn(async move {
        let d: &MountedData = &evt.data();

        let scrollable_size = d.get_scroll_size().await.unwrap();
        let height = scrollable_size.height as i32;
        let scrollable_rect = d.get_client_rect().await.unwrap();
        let screen = scrollable_rect.height() as i32;
        tracing::debug!("scroll info: {}, {}", height, screen);
        *scroll.write() = ScrollInfo {
            top: 0,
            height,
            screen,
        };
    });
}

pub fn on_scroll_scrollable(evt: Event<ScrollData>, mut scroll: Signal<ScrollInfo>) {
    let top = evt.data().scroll_top() as i32;
    let height = evt.data().scroll_height();
    let screen = evt.data().client_height();
    *scroll.write() = ScrollInfo {
        top,
        height,
        screen,
    };
}

#[component]
pub fn CustomScrollBar(scroll: Signal<ScrollInfo>) -> Element {
    let scroll_top = use_memo(move || {
        let ScrollInfo {
            top,
            height,
            screen,
        } = *scroll.read();
        if height == 0 {
            return 0;
        }
        top * screen / height
    });
    let scroll_height = use_memo(move || {
        let ScrollInfo {
            top: _,
            height,
            screen,
        } = *scroll.read();
        if height == 0 {
            return 0;
        }
        screen * screen / height
    });
    rsx! {
        div {
            id: "scroll_bar_custom",
            top: "{scroll_top}px",
            height: "{scroll_height}px",
            pointer_events: "none",
            z_index: 10,
            class: "flex flex-col absolute right-0 min-h-20 w-5  justify-between text-black items-center border-3 border-white border-b-black border-r-black bg-gray-300 hover:bg-gray-500 text-sm",

            p { class: "bg-gray-200 w-full text-center", "▲" }
            p { class: "bg-gray-200 w-full text-center", "▼" }
        }
    }
}
