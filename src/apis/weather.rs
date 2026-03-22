use crate::apis::geolocation::LocationResponse;
use reqwest::get;
use serde::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherResponse {
    #[serde(rename = "type")]
    pub type_field: String,
    pub properties: Properties,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geometry {
    #[serde(rename = "type")]
    pub type_field: String,
    pub coordinates: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    pub timeseries: Vec<Series>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Series {
    pub time: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub instant: Instant,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instant {
    pub details: Details,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Details {
    #[serde(rename = "air_pressure_at_sea_level")]
    pub air_pressure_at_sea_level: f64,
    #[serde(rename = "air_temperature")]
    pub air_temperature: f64,
    #[serde(rename = "cloud_area_fraction")]
    pub cloud_area_fraction: f64,
    #[serde(rename = "relative_humidity")]
    pub relative_humidity: f64,
    #[serde(rename = "wind_from_direction")]
    pub wind_from_direction: f64,
    #[serde(rename = "wind_speed")]
    pub wind_speed: f64,
}

const WEATHER_URL: &str = "https://api.met.no/weatherapi/locationforecast/2.0/compact?";

pub async fn get_weather(location: &LocationResponse) -> Option<String> {
    let url = format!("{WEATHER_URL}lat={}&lon={}", location.lat, location.lon);
    let weather: WeatherResponse = get(url).await.ok()?.json().await.ok()?;
    Some(format!(
        "{:0.1}C",
        weather
            .properties
            .timeseries
            .first()
            .unwrap()
            .data
            .instant
            .details
            .air_temperature,
    ))
}
