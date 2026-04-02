pub mod popup_widget;
mod socials;
pub use socials::Socials;

mod navbar;
pub use navbar::RoutedSPA;

mod art;
pub use art::Art;
mod contact;
pub use contact::encode;
pub use contact::Contact;
mod projects;
pub use projects::Projects;
mod who_am_i;
pub use who_am_i::WhoAmI;
mod under_construction;

pub mod gif_links;
mod info_widget;
pub use info_widget::InfoWidget;

mod art_chapters;

mod open_graph;
pub use open_graph::{OgProps, OpenGraph};
