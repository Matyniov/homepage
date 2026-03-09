use dioxus::prelude::*;

#[derive(Debug, Default, Clone, Copy)]
pub struct ScrollInfo {
    pub top: i32,
    pub height: i32,
    pub screen: i32,
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