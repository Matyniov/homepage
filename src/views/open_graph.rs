use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct OgProps {
    title: String,
    description: String,
    #[props(optional)]
    site_name: Option<String>,
    theme_color: String,
}

#[component]
pub fn OpenGraph(props: OgProps) -> Element {
    rsx! {
        document::Meta { property: "og:title", content: "{props.title}" }
        document::Meta { property: "og:description", content: "{props.description}" }
        document::Meta { property: "og:type", content: "website" }

        if let Some(name) = &props.site_name {
            document::Meta { property: "og:site_name", content: "{name}" }
        }
        document::Meta { name: "theme-color", content: "{props.theme_color}" }
        document::Meta { name: "twitter:title", content: "{props.title}" }
        document::Meta { name: "twitter:description", content: "{props.description}" }
    }
}
