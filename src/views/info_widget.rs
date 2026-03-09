use dioxus::prelude::*;

use crate::{apis::{
    geolocation::{LocationResponse, get_location},
    ip::{get_ip, ip_to_string},
    weather::get_weather,
}, views::popup_widget::{BorderStyles, Popup}};

#[derive(Debug)]
struct Infos {
    pub ip: Option<(u8, u8, u8, u8)>,
    pub loc: Option<LocationResponse>,
    pub weather: Option<String>,
}

async fn get_infos() -> Infos {
    let ip = get_ip().await;
    let loc: Option<LocationResponse> = if let Some(ip) = ip {
        get_location(ip).await
    } else {
        None
    };
    let weather: Option<String> = if let Some(loc) = &loc {
        get_weather(loc).await
    } else {
        None
    };

    Infos { ip, loc, weather }
}

#[component]
fn ip_element(ip: (u8, u8, u8, u8)) -> Element {
    let ip = ip_to_string(ip);
    rsx! {
        p { "__IP: {ip}" }
    }
}

#[component]
fn loc_element(loc: LocationResponse) -> Element {
    let coords_format = format!("__CORDS=LAT_{}xLON_{}", loc.lat, loc.lon);
    let place_format = format!("__PLACE={}@{}", loc.city, loc.country);
    rsx! {
        div {
            p { {coords_format} }
            p { {place_format} }
        }
    }
}

#[component]
fn weather_emelent(weather: String) -> Element {
    rsx! {
        p { "__LOCAL_TEMP: {weather}" }
    }
}

#[component]
pub fn InfoWidget() -> Element {
    let resource = use_resource(get_infos);

    let loading_screen = rsx! {
        img { src: asset!("/assets/gif/matrix.gif") }
    };

    let content = rsx! {
        div { class: "flex flex-col items-center justify-center gap-5",
            img { src: asset!("/assets/gif/neo.gif") }
            div {
                p { class: "italic text-white", "Follow the white rabbit..." }
                match &*resource.read_unchecked() {
                    Some(Infos { ip, loc, weather }) => {
                        rsx! {
                            if let Some(ip) = *ip {
                                ip_element { ip }
                            }
                            if let Some(loc) = loc.clone() {
                                loc_element { loc }
                            }
                            if let Some(weather) = weather.clone() {
                                weather_emelent { weather }
                            }
                        }
                    }
                    None => loading_screen,
                }
            }
        
        }
    };

    rsx! {
        Popup {
            // bg-green-950
            // text-green-950
            // bg-green-500
            // text-green-500
            bg_col: "green-950",
            main_col: "green-500",
            border_style: BorderStyles::Double,
            title: "__HTTP_500_ERR__",

            {content}
        }
    }
}
