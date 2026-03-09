use dioxus::prelude::*;

use crate::views::{Art, Contact, InfoWidget, Navbar, Projects, Socials, WhoAmI};

#[component]
fn Test() -> Element {
    rsx! {
        InfoWidget {}
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
        #[route("/")]
        Socials {},
        #[route("/contact")]
        Contact {},

        #[route("/art")]
        Art {},

        #[route("/whoami")]
        WhoAmI {},

        #[route("/projects")]
        Projects {},

        #[cfg(debug_assertions)]
        #[route("/test")]
        Test {}
}
