use dioxus::{logger::tracing, prelude::*};

use crate::views::{
    popup_widget::{BorderStyles, Popup},
    scroll_bar::ScrollInfo,
    CustomScrollBar,
};

#[component]
pub fn WhoAmI() -> Element {
    let mut scroll = use_signal(|| ScrollInfo::default());

    rsx! {
        div { id: "container", class: "lg:h-screen relative",

            div {
                id: "scrollable",
                class: "lg:h-screen lg:overflow-y-auto custom-scroll-area flex flex-col items-center pt-15",
                onmount: move |element| {
                    spawn(async move {
                        let d: &MountedData = &element.data();

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
                },
                onscroll: move |evt| {
                    let top = evt.data().scroll_top() as i32;
                    let height = evt.data().scroll_height() as i32;
                    let screen = evt.data().client_height() as i32;
                    *scroll.write() = ScrollInfo { top, height, screen };
                },
                div { class: "hidden lg:block",
                    CustomScrollBar { scroll }
                }
                div { class: "w-3/4 ",
                    Popup {
                        title: "Vessel.",
                        main_col: "white",
                        bg_col: "gray-500",
                        border_style: BorderStyles::Double,
                        h1 { class: "lines_font text-5xl", "Who is the man behind the digital veil?" }
                    }
                
                }
            }
        
        }
    }
}
