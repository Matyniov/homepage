use dioxus::prelude::*;
use maty_homepage::router::Route;
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const ANIMS: Asset = asset!("/assets/styling/loading_anim.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: ANIMS }

        for link_str in [
            "https://fonts.cdnfonts.com/css/barcode-font",
            "https://fonts.cdnfonts.com/css/ds-terminal",
            "https://fonts.cdnfonts.com/css/terminal",
        ]
        {
            document::Link { rel: "stylesheet", href: link_str }
        }

        Router::<Route> {}
    }
}
