use dioxus::{logger::tracing, prelude::*};
use std::fmt::Display;

use crate::router::Route;

pub fn nav_element(
    route: Route,
    text: impl Display,
) -> Element {
    rsx! {
        Link {
            class: "bar_code_font text-6xl lg:text-8xl p-1 hover:bg-yellow-300 hover:text-black",
            active_class: "bar_code_font text-7xl lg:text-9xl bg-yellow-300 text-black",
            to: route,
            "{text}"
        }
    }
}

#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { class: "flex flex-col flex items-center lg:items-start justify-center lg:flex-row bg-blue-800 h-fit",

            p { class: "lg:hidden text-black text-sm border-2 border-black bg-yellow-300 border-dotted m-2 p-2",
                "This site is best experienced on desktop :3"
            }
            div { class: "lg:h-screen lg:min-h-220 flex items-center justify-center",
                nav {
                    id: "navbar",
                    class: "bg-cyan-200 text-blue-800 pl-1 pr-1 pb-1 black_dropshadow m-5",
                    fieldset { class: "flex lg:flex-col lg:h-full border-9 border-double border-blue-800 gap-1 items-center justify-center",
                        legend { class: "text-center text-base lg:text-xl pl-2 pr-2 text-cyan-200 bg-blue-800",
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

                        fieldset { class: "text-xs md:text:regular border-t-3 mt-auto ml-auto mr-auto mb-auto w-20 md:w-40 overflow-hidden lg:mt-15",
                            legend { class: "text-center pl-2 pr-2 text-cyan-200 bg-blue-800",
                                "Hello world::"
                            }
                            img {
                                class: "h-full w-full object-top",
                                image_rendering: "pixelated",
                                src: asset!("assets/gif/globe.gif"),
                            }
                        }
                    }
                }
            }

            div { class: "lg:w-full", Outlet::<Route> {} }

            div { class: "flex justify-between items-center mt-10 lg:mt-0 lg:[writing-mode:vertical-rl] text-cyan-200 p-1 lg:h-screen",
                p { class: "hidden lg:block text-9xl", "╳┼┼╳" }
                p { class: "text-9xl  bar_code_font", "MATYNIOV.HOMEPAGE" }
            }
        }
    }
}
