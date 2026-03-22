use crate::views::{popup_widget::Popup, InfoWidget};
use crate::{
    router::Route,
    views::{gif_links, popup_widget::BorderStyles},
};
use chrono::{Datelike, Local, NaiveDate};
use dioxus::prelude::*;

const BDAY: NaiveDate = NaiveDate::from_ymd_opt(2002, 5, 30).unwrap();

const TWITTER_URL: &str = "https://x.com/matyniov_arts";
const BLUESKY_URL: &str = "https://bsky.app/profile/matyniov.bsky.social";
const FLICKR_URL: &str = "https://www.flickr.com/people/204101766@N05/";
const GITHUB_URL: &str = "https://github.com/Matyniov";
const YOUTUBE_URL: &str = "https://www.youtube.com/@Matyniov";

#[component]
fn SocialLink(title: String, accent: String, link: String, icon: Asset) -> Element {
    rsx! {
        a {
            class: "flex items-center black_dropshadow bg-gray-400 text-white mb-8 hover:bg-yellow-300 hover:text-black",
            href: "{link}",
            target: "_blank",
            img { class: "size-10 {accent} mr-2 p-2", src: "{icon}" }
            p { "{title} ↗" }
        }
    }
}
// }
/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Socials() -> Element {
    let now = Local::now().date_naive();
    let age: String = now
        .years_since(BDAY)
        .and_then(|inner| inner.to_string().into())
        .unwrap_or("ERR".into());

    let birthday_today: bool = BDAY
        .with_year(now.year())
        .map(|date| date == now)
        .unwrap_or(false);

    let cake = if birthday_today { "🎂" } else { "" };

    rsx! {
        // Canvas
        main {
            class: "relative flex items-center justify-center lg:h-screen lg:min-h-220 lg:w-full mt-5 lg:mt-0 p-5",

            class: "ani-load-init",
            div { class: "z-7 lg:pr-10",
                Popup {
                    bg_col: "cyan-200",
                    main_col: "blue-800",
                    border_style: BorderStyles::Double,
                    title: "Profile",
                    div {
                        class: "lg:absolute lg:right-0 lg:top-1/2 lg:-translate-y-1/3 lg:translate-x-1/4 black_dropshadow z-10",
                        class: "ani-load-trans-first",
                        img {
                            image_rendering: "pixelated",
                            src: asset!("/assets/imgs/maty1.png"),
                        }
                    }
                    div {
                        class: "black_dropshadow",
                        class: "ani-load-trans-first",
                        div { class: "bg-blue-800 text-cyan-200 p-2",
                            h1 { class: "lines_font text-5xl md:text-7xl lg:text-9xl italic lg:mr-8",
                                "Matyniov"
                            }
                            p { class: "text-right text-xs md:text-xl", "a.k.a. Maty" }
                        }
                        div { class: "flex flex-col lg:flex-row items-center justify-between gap-2 border-3 p-2 mb-10",
                            p { "■ ■ ■ ■ ■" }
                            p { "HE/HIM" }
                            p { "lvl.{age}{cake}" }
                            p { "EU//PL//WAW" }
                            p { "■ ■ ■ ■ ■" }
                        }
                    }
                    ul {
                        class: "w-full lg:w-1/2",
                        class: "ani-load-trans-first",
                        li {
                            SocialLink {
                                title: "Bluesky",
                                icon: asset!("/assets/svgs/bluesky.svg"),
                                link: BLUESKY_URL,
                                accent: "bg-blue-500",
                            }
                        }
                        li {
                            SocialLink {
                                title: "Twitter",
                                icon: asset!("/assets/svgs/twitter.svg"),
                                link: TWITTER_URL,
                                accent: "bg-sky-400",
                            }
                        }
                        li {
                            SocialLink {
                                title: "Youtube",
                                icon: asset!("/assets/svgs/youtube.svg"),
                                link: YOUTUBE_URL,
                                accent: "bg-red-500",
                            }
                        }
                        li {
                            SocialLink {
                                title: "Flickr",
                                icon: asset!("/assets/svgs/flickr.svg"),
                                link: FLICKR_URL,
                                accent: "bg-pink-500",
                            }
                        }
                        li {
                            SocialLink {
                                title: "Github",
                                icon: asset!("/assets/svgs/github.svg"),
                                link: GITHUB_URL,
                                accent: "bg-gray-100",
                            }
                        }
                        li { class: "text-center text-s mb-2 md:text-base",
                            "If you would like to get in touch,"
                            br {}
                            "check out the "
                            Link {
                                class: "font-bold italic",
                                to: Route::Contact {},
                                "contact page"
                            }
                            "!"
                        }
                    }

                    div { class: "flex gap-1 justify-between items-end",
                        p { class: "text-base md:text-2xl", ">>>EOF" }
                        p { class: "bar_code_font text-5xl md:text-7xl", "PROFILE.PAGE" }
                    }
                }
            }

            div {
                class: "hidden lg:block absolute right-1/30 bottom-2/15  z-5 p-5 bg-gray-900 black_dropshadow",
                class: "ani-load-trans-second",
                fieldset { class: "border-b-3 text-cyan-300 flex flex-col items-center justify-center",
                    legend { class: "text-center lines_font text-2xl", "⇶ The digital man" }
                    img {
                        class: "h-48 mb-2",
                        src: asset!("/assets/gif/digital man 1.gif"),
                    }
                    p { "Would you like to" }
                    p { "live forever?" }
                }
            }

            div {
                class: "hidden lg:block absolute left-1/20 bottom-3/20",
                class: "ani-load-trans-second",
                Popup {
                    // bg-pink-700
                    // text-pink-700
                    // bg-white
                    // text-white
                    bg_col: "pink-700",
                    main_col: "white",
                    border_style: BorderStyles::Solid,
                    title: "Check out my flickr!",
                    div { class: "items-center text-center",
                        p { "I take photos!" }
                        p { "(even some on film)" }
                        img { src: gif_links::KODAK }

                    }
                }
            }

            div {
                class: "hidden lg:block absolute left-1/40 top-3/20",
                class: "ani-load-trans-second",

                Popup {
                    // bg-gray-500
                    // text-gray-500
                    // bg-white
                    // text-white
                    bg_col: "gray-500",
                    main_col: "white",
                    border_style: BorderStyles::Solid,
                    title: "Nickname",
                    div { class: "flex items-center text-center",
                        p { class: "w-30 text-center",
                            "Dont bother trying to pronounce my name, just call me Maty"
                        }
                        img { src: gif_links::CLIPPY_NOTE }

                    }
                }
            }

            div {
                class: "hidden lg:block absolute top-1/20 right-0",
                class: "ani-load-trans-second",
                InfoWidget {}
            }
        }
    }
}
