use dioxus::prelude::*;

use crate::views::under_construction::under_construction_widget;

#[component]
pub fn Projects() -> Element {
    rsx! {
        div { class: "flex flex-col items-center justify-center h-screen",
            div { class: "fancy_title text-6xl w-100 mb-20 text-center -skew-10 bg-white p-10 black_dropshadow",
                "Something beautiful is going to happen"
            }
            {under_construction_widget()}
        
        }
    }
}
