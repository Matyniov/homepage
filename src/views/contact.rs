use dioxus::prelude::*;

use crate::views::under_construction::under_construction_widget;

#[component]
pub fn Contact() -> Element {
    rsx! {
        {under_construction_widget()}
    }
}
