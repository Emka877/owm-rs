#![allow(dead_code)]
use super::structures::WeatherData;
use crate::owm::constants::{ENDPOINT_WEATHER, OWM_URI};
use anyhow::Result;

pub async fn get_weather_for_coordinates(
    latitude: f32,
    longitude: f32,
    api_key: String,
) -> Result<WeatherData> {
    let call: String = format!(
        "{}{}?lat={}&lon={}&appid={}",
        OWM_URI, ENDPOINT_WEATHER, latitude, longitude, api_key
    );
    let response: reqwest::Response = reqwest::get(&call).await.expect("Error");
    Ok(response.json().await?)
}
