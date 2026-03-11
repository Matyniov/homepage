use dioxus::{logger::tracing, prelude::*};

use crate::views::{
    CustomScrollBar, popup_widget::{BorderStyles, Popup}, scroll_bar::{ScrollInfo, on_mount_scrollable, on_scroll_scrollable}
};

#[component]
pub fn WhoAmI() -> Element {
    let scroll = use_signal(ScrollInfo::default);

    rsx! {
        div { id: "container", class: "lg:h-screen relative",
            // class: "ani-load-init",
            main {
                id: "scrollable",
                class: "lg:h-screen lg:overflow-y-auto custom-scroll-area flex flex-shrink-0 justify-center",

                onmount: move |evt| {
                    on_mount_scrollable(evt, scroll);
                },
                onscroll: move |evt| {
                    on_scroll_scrollable(evt, scroll);
                },
                div { class: "hidden lg:block",
                    CustomScrollBar { scroll }
                }

                article { class: "flex flex-col flex-shrink-0 items-center gap-10 max-w-2/3 text-black content_serif border-20 border-white bg-amber-100",
                    div { class: "pt-30 pb-30 bg-red-600 flex flex-col items-center text-white",

                        img {
                            class: "bg-yellow-400 w-100 rounded-full overflow-hidden p-2 border-10 border-white",
                            box_shadow: "",
                            src: asset!("/assets/imgs/maty_bust.png"),
                        }
                        h1 { class: "bold_serif text-5xl/15 text-center w-2/3 break-keep border-r-2 border-l-2",
                            "QUOTATIONS FROM CHAIRMAN "
                            span { class: "tracking-widest whitespace-nowrap", "MAO-TSYNIOV" }
                        }
                    }

                    h2 { class: " text-2xl", "Introduction" }

                    p { "Hello world!" }

                    h2 { class: "", "On communism and socialism" }

                    p { "Based. Poor execution." }

                    h2 { class: "content_serif", "On capitalism" }

                    p { "Hella cringe." }
                
                }
            }
        }
    }
}

// details { class: "group",
//     summary { class: "flex cursor-pointer list-none items-center justify-between font-medium text-gray-900",
//         "What is Dioxus?"
//         // A simple chevron that rotates when the parent is 'open'
//         span { class: "transition group-open:rotate-180",
//             "▼"
//         }
//     }
//     div { class: "mt-3 text-gray-600 leading-relaxed",
//         "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust."
//     }
// }
