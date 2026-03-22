use dioxus::prelude::*;

pub fn under_construction_widget() -> Element {
    rsx! {
        div { class: "items-center justify-center bg-black black_dropshadow",
            img { class: "w-200 h-auto", src: asset!("/assets/gif/con4.gif") }
            div { class: "flex items-center justify-center",
                img { src: asset!("/assets/gif/con2.gif") }
                img { class: "h-40 w-auto", src: asset!("/assets/gif/con1.gif") }
                img { src: asset!("/assets/gif/con3.gif") }

            }
            img { class: "w-200 h-auto", src: asset!("/assets/gif/con4.gif") }
        }
    }
}
