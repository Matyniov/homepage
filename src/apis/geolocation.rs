use reqwest::get;
use serde::Deserialize;

use crate::apis::ip::ip_to_string;
const LOC_URL: &str = "https://api.techniknews.net/ipgeo/";

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationResponse {
    // pub status: String,
    // pub continent: String,
    pub country: String,
    pub country_code: String,
    // pub region_name: String,
    pub city: String,
    // pub zip: String,
    pub lat: f64,
    pub lon: f64,
    // pub timezone: String,
    // pub currency: String,
    // pub isp: String,
    // pub org: String,
    // #[serde(rename = "as")]
    // pub as_field: String,
    // pub reverse: String,
    // pub mobile: bool,
    // pub proxy: bool,
    // pub hosting: bool,
    // pub ip: String,
    // pub cached: bool,
}

pub async fn get_location(ip: (u8, u8, u8, u8)) -> Option<LocationResponse> {
    let url = format!("{LOC_URL}/{}", ip_to_string(ip));
    get(url).await.ok()?.json().await.ok()
}
