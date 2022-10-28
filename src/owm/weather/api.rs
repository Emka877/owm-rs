#![allow(dead_code)]
use super::structures::WeatherData;
use crate::owm::constants::{OWM_URI, ENDPOINT_WEATHER};

pub async fn get_weather_for_coordinates(latitude: f32, longitude: f32, api_key: String) 
    -> WeatherData 
{
    let call: String = format!("{OWM_URI}{ENDPOINT_WEATHER}?lat={latitude}&lon={longitude}&appid={api_key}");
    let response: reqwest::Response = reqwest::get(&call).await.expect("Error");
    response.json().await.expect("Could not parse the JSON.")
}