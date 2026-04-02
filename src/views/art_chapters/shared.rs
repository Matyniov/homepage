use dioxus::prelude::*;

#[component]
pub fn Plaque(
    class: Option<String>,
    artist_name: String,
    artist_info: String,
    title: String,
    art_date: String,
    art_method: String,
    children: Element,
) -> Element {
    rsx! {
        div {
            id: "{title}",
            class: "p-10 xl:max-w-100 items-start text-justify fancy_font bg-white shadow-2xl",
            class: if let Some(class) = &class { "{class}" },
            h1 { class: "text-3xl", {artist_name} }
            p { class: "text-sm", {artist_info} }
            a {
                href: "#{title}",
                class: "mt-5 font-bold text-3xl fancy_title",
                {title.clone()}
            }
            p { class: "text-sm", {art_date} }
            p { class: "text-sm", {art_method} }
            p { class: "text-base mt-5", {children} }
        }
    }
}

#[component]
pub fn MatyPlaque(
    class: Option<String>,
    title: String,
    art_date: String,
    art_method: String,
    children: Element,
) -> Element {
    rsx! {
        Plaque {
            class,
            artist_name: "Matyniov",
            artist_info: "Poland, born 2002",
            title,
            art_date,
            art_method,
            children,
        }
    }
}

#[component]
pub fn ChapterPlaque(
    class: Option<String>,
    title: String,
    date: String,
    children: Element,
) -> Element {
    rsx! {
        Plaque {
            class: "xl:max-w-180",
            artist_name: "",
            artist_info: "",
            title,
            art_date: date,
            art_method: "",
            children,
        }
    }
}
pub const FIGURE_SHARED: &str =
    "flex flex-col items-center xl:p-10 bg-gray-200 xl:gap-10 shadow-xl";
pub const FIGURE_LEFT: &str = "xl:flex-row-reverse mr-auto";
pub const FIGURE_RIGHT: &str = "xl:flex-row ml-auto";
pub const PICTURE: &str = "xl:max-h-150 xl:max-w-200 shadow-2xl xl:border-40 border-white";
pub const LINK: &str = "text-blue-800 underline";
