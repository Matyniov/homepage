use std::collections::HashMap;

use dioxus::prelude::*;

use crate::views::popup_widget::{BorderStyles, Popup};

const KEY: u8 = 123;

pub fn encode(s: &str) -> Vec<u8> {
    s.as_bytes().iter().map(|byte| byte ^ KEY).collect()
}

fn decode(v: &[u8]) -> String {
    let bytes: Vec<u8> = v.iter().map(|byte| byte ^ KEY).collect();
    String::from_utf8_lossy(&bytes).to_string()
}

const MACHINE: &str = "https://youtu.be/Yn-ZhiLrQZc?si=Mv_RBZ70mBab0lxk&t=9";

#[component]
pub fn Contact() -> Element {
    let mut human = use_signal(|| false);
    let mut credentials: Signal<HashMap<String, String>> = use_signal(HashMap::new);

    use_effect(move || {
        if *human.read() {
            let mut map = HashMap::new();
            // I did this so that webscrapers dont just get my email for free.
            map.insert(
                ">EMAIL".into(),
                decode(&[
                    22, 26, 15, 2, 21, 18, 20, 13, 59, 28, 22, 26, 18, 23, 85, 24, 20, 22,
                ]),
            );
            map.insert(
                ">TELEGRAM".into(),
                decode(&[
                    19, 15, 15, 11, 8, 65, 84, 84, 15, 85, 22, 30, 84, 22, 26, 15, 2, 21, 18, 20,
                    13,
                ]),
            );
            map.insert(
                ">DISCORD".into(),
                decode(&[59, 22, 26, 15, 2, 21, 18, 20, 13]),
            );
            map.insert(
                ">STEAM".into(),
                decode(&[
                    19, 15, 15, 11, 8, 65, 84, 84, 8, 15, 30, 26, 22, 24, 20, 22, 22, 14, 21, 18,
                    15, 2, 85, 24, 20, 22, 84, 18, 31, 84, 22, 26, 15, 2, 21, 18, 20, 13, 84,
                ]),
            );
            credentials.set(map);
        }
    });

    rsx! {
        div {
            class: "flex items-center justify-center lg:h-screen",
            class: if *human.read() { "hidden" },
            Popup {
                // bg-red-700
                // text-red-700
                // bg-white
                // text-white
                bg_col: "red-700",
                main_col: "white",
                border_style: BorderStyles::Double,
                title: "Identify your form",
                div { class: "flex flex-col justify-center items-center lg:p-20 text-xl",
                    p { "Are you of flesh and bone" }
                    p { "or steel and silicone?" }
                    p { class: "mt-10", "Cast below: MAN or MACHINE?" }
                    input {
                        placeholder: ">MAN / MACHINE<",
                        class: "mt-10 w-80 p-2 black_dropshadow text-center hover:bg-yellow-300 bg-white text-black",
                        oninput: move |value| {
                            let input: String = value.value();
                            if input.to_lowercase() == "man" {
                                human.set(true);
                            }
                            else if input.len() > 5 {
                                if let Some(window) = web_sys::window() {
                                    let _ = window.location().set_href(MACHINE);
                                }
                            }
                        },
                    }
                }
            
            }
        }
        div {
            class: if !*human.read() { "hidden" },
            class: "flex items-center justify-center lg:h-screen",
            Popup {
                // bg-green-950
                // text-green-950
                // bg-green-500
                // text-green-500
                bg_col: "green-950",
                main_col: "green-500",
                border_style: BorderStyles::Double,
                title: "__HTTP_200_ACCESS_GRANTED__",

                table { class: "[&_td]:p-2",
                    tr { class: "font-bold border-b-2",
                        td { "__COMM" }
                        td { "__PORT" }
                    }
                    for (k , v) in credentials.read().clone() {
                        tr {
                            td { {k} }
                            td { {v} }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::views::contact::{decode, encode};

    #[test]
    fn encode_run() {
        let steam = "https://steamcommunity.com/id/matyniov/";
        dbg!(encode(steam));
    }

    #[test]
    fn test_encoder_decoder() {
        let message: String =
            "my_very.;.;./!#@!#@!#!@##!@#!@secret_messageąðœąðąðąðœðćœęœπęπ".into();

        let encoded = encode(message.as_str());
        dbg!(&encoded);

        assert_eq!(message, decode(encoded.as_slice()));
    }
}
