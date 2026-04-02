use dioxus::prelude::*;
use std::fmt::Display;

use crate::router::Route;

pub fn nav_element(route: Route, text: impl Display) -> Element {
    rsx! {
        Link {
            class: "bar_code_font text-8xl p-1 hover:bg-yellow-300 hover:text-black hover:cursor-pointer",
            active_class: "bar_code_font text-9xl bg-yellow-300 text-black",
            to: route,
            "{text}"
        }
    }
}

#[component]
pub fn RoutedSPA() -> Element {
    rsx! {
        div { class: "flex flex-col flex items-center xl:items-start justify-start xl:flex-row bg-blue-800 overflow-x-hidden",

            p { class: "xl:hidden text-black text-sm border-2 border-black bg-yellow-300 border-dotted m-2 p-2",
                "This site is best experienced on desktop :3"
            }
            div { class: "xl:h-screen xl:min-h-220 flex items-center justify-center",
                nav {
                    id: "navbar",
                    class: "bg-cyan-200 text-blue-800 pl-1 pr-1 pb-1 black_dropshadow m-5",
                    fieldset { class: "flex xl:flex-col xl:h-full border-9 border-double border-blue-800 gap-1 items-center justify-center",
                        legend { class: "text-center text-base xl:text-xl pl-2 pr-2 text-cyan-200 bg-blue-800",
                            "NAVIGATION::"
                        }
                        div {
                            id: "navbar_inner",
                            class: "flex flex-col items-start",
                            {nav_element(Route::Socials {}, "000.SOCIALS.")}
                            {nav_element(Route::WhoAmI {}, "001.WHOAMI..")}
                            {nav_element(Route::Art {}, "002.ART.....")}
                            {nav_element(Route::Projects {}, "003.PROJECTS")}
                            {nav_element(Route::Contact {}, "004.CONTACT.")}
                        }

                        fieldset { class: "text-base border-t-3 my-10 ml-auto mr-auto size-40 overflow-hidden hidden xl:block",
                            legend { class: "text-center pl-2 pr-2 text-cyan-200 bg-blue-800",
                                "Hello world::"
                            }
                            img {
                                class: "p-2 h-auto w-full object-top",
                                image_rendering: "pixelated",
                                src: asset!("assets/gif/globe.gif"),
                            }
                        }
                    }
                }
            }

            div { class: "xl:w-full", Outlet::<Route> {} }

            div { class: "flex justify-between items-center mt-10 xl:mt-0 xl:[writing-mode:vertical-rl] text-cyan-200 p-1 xl:h-screen  bg-blue-800",
                p { class: "hidden xl:block text-9xl", "╳┼┼╳" }
                p { class: "text-7xl xl:text-9xl  bar_code_font", "MATYNIOV.HOMEPAGE" }
            }
        }
    }
}
