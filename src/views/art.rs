use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlAudioElement;

use crate::views::art_chapters::{ch1::Ch1, ch2::Ch2, ch3::Ch3, shared::ChapterPlaque};

fn get_icon(playing: bool) -> String {
    if !playing {
        "►".into()
    } else {
        "⏸".into()
    }
}

const DEFAULT_VOLUME: f64 = 0.25;

#[component]
fn FadeIn(children: Element) -> Element {
    let mut is_visible = use_signal(|| false);

    rsx! {
        div {
            // 1. Initial Hidden State + Transition Setup
            class: "transition-all duration-1000 ease-out",
            class: if is_visible() { "opacity-100 translate-y-0" } else { "opacity-0 translate-y-10" },

            // 2. The Trigger
            onvisible: move |_| is_visible.set(true),

            {children}
        }
    }
}

#[component]
pub fn AudioPlayer(class: Option<String>, source: Asset, title_text: Element) -> Element {
    let mut is_playing = use_signal(|| false);
    let mut audio_handle = use_signal(|| None::<web_sys::HtmlAudioElement>);
    let mut volume = use_signal(|| DEFAULT_VOLUME);

    use_effect(move || {
        if let Some(audio) = audio_handle.read().as_ref() {
            audio.set_volume(DEFAULT_VOLUME);
        }
    });

    use_effect(move || {
        if let Some(audio) = audio_handle.read().as_ref() {
            audio.set_volume(*volume.read());
        }
    });

    let toggle_play = move |_| {
        if let Some(audio) = audio_handle.read().as_ref() {
            if *is_playing.read() {
                let _ = audio.pause();
            } else {
                let _ = audio.play();
            }
            is_playing.toggle();
        }
    };

    let seek_start = move |_| {
        if let Some(audio) = audio_handle.read().as_ref() {
            audio.set_current_time(0.0);
        }
    };

    let cast_as_audio_element = move |evt: Event<MountedData>| {
        use dioxus::web::WebEventExt;
        audio_handle.set(
            evt.try_as_web_event()
                .and_then(|inner| inner.dyn_into::<HtmlAudioElement>().ok()),
        );
    };

    let change_volume = move |evt: Event<FormData>| {
        if let Ok(vol) = evt.value().parse::<f64>() {
            volume.set(vol);
        }
    };

    rsx! {
        div {
            id: "player",
            class: "z-20 fixed bottom-0 mb-5 left-1/2 -translate-x-1/2 rounded-full bg-white shadow p-5 ",
            class: "text-xl",
            class: if let Some(inner) = &class { "{inner}" },
            audio {
                src: source,
                class: "hidden",
                r#loop: true,
                onmount: cast_as_audio_element,
            }
            div { class: "flex flex-col xl:flex-row gap-5 items-center justify-center",
                {title_text}
                div { class: "flex gap-5 items-center justify-center",
                    button { onclick: toggle_play, {get_icon(*is_playing.read())} }
                    button { onclick: seek_start, "⏮" }
                    input {
                        r#type: "range",
                        class: "w-24 accent-black",
                        min: "0",
                        max: "1",
                        step: "0.01",
                        value: "{volume}",
                        oninput: change_volume,
                    }
                }
            }
        
        }
    }
}

#[component]
pub fn Art() -> Element {
    rsx! {
        AudioPlayer {
            source: asset!("/assets/audio/thebussy.mp3"),
            class: "fancy_title",
            title_text: rsx! {
                a {
                    href: "https://pixabay.com/music/classical-piano-clair-de-lune-suite-bergamasque-l-75-3rd-movement-claude-debussy-448s-11942/",
                    target: "_blank",
                    "♫ The bussay ♫"
                }
            },
        }
        div { class: "xl:p-8 flex flex-col items-center gap-10 ani-entrance",
            div { class: "fancy_title text-4xl xl:text-8xl text-white tracking-widest skew-y-6 skew-x-20 p-4 bg-black mt-10 mb-10 shadow-xl",
                "-galeria sztuki-"
            }
            Ch1 {}
            Ch2 {}
            Ch3 {}

            ChapterPlaque { title: "...Hopefully many more to come", date: "Now",
                "Thank you for taking your time to browse my humble gallery! Every person is a story, and im glad you
                decided to discover some of mine through this art."
            }

            div { class: "xl:mt-20" }
        }
    }
}
