use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BorderStyles {
    Solid,
    Double,
}

impl BorderStyles {
    fn get_style_and_width(&self) -> (String, u32) {
        match self {
            Self::Solid => ("border-solid".into(), 3),
            Self::Double => ("border-double".into(), 9),
        }
    }
}

#[component]
pub fn Popup(
    bg_col: String,
    main_col: String,
    border_style: Option<BorderStyles>,
    title: Option<String>,
    children: Element,
) -> Element {
    let (style, width) = border_style
        .unwrap_or(BorderStyles::Solid)
        .get_style_and_width();
    rsx! {
        div {
            class: "bg-{bg_col} pl-2 pr-2 pb-2 black_dropshadow text-{main_col}",
            class: if title.is_none() { "pt-2" },
            fieldset { class: "xl:relative border-{width} p-5 border-{main_col} {style}",
                if let Some(title) = title.clone() {
                    legend { class: "text-l pl-2 pr-2 bg-{main_col} text-{bg_col}", {title} }
                }
                {children}
            }
        }
    }
}
