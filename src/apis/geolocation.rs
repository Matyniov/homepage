use reqwest::get;
use serde::Deserialize;

const LOC_URL: &str = "https://get.geojs.io/v1/ip/geo.json";

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocationResponse {
    pub accuracy: i64,
    #[serde(rename = "area_code")]
    pub area_code: String,
    pub asn: i64,
    pub city: String,
    #[serde(rename = "continent_code")]
    pub continent_code: String,
    pub country: String,
    #[serde(rename = "country_code")]
    pub country_code: String,
    #[serde(rename = "country_code3")]
    pub country_code3: String,
    pub ip: String,
    pub latitude: String,
    pub longitude: String,
    pub organization: String,
    #[serde(rename = "organization_name")]
    pub organization_name: String,
    pub region: String,
    pub timezone: String,
}

pub async fn get_location() -> Option<LocationResponse> {
    get(LOC_URL).await.ok()?.json().await.ok()
}
