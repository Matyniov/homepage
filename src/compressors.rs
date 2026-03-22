use dioxus::prelude::*;

pub const fn image_default() -> AssetOptions {
    AssetOptions::image()
        .with_avif()
        .with_size(ImageSize::Automatic)
        .into_asset_options()
}
