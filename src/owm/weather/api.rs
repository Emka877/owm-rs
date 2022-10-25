#![allow(dead_code)]

use super::structures::WeatherData;

pub async fn get_weather_for_coordinates(latitude: f32, longitude: f32, api_key: String) 
    -> WeatherData 
{
    let call: String = format!("https://api.openweathermap.org/data/2.5/weather?lat={latitude}&lon={longitude}&appid={api_key}");
    let response: reqwest::Response = reqwest::get(&call).await.expect("Error");
    response.json().await.expect("Could not parse the JSON.")
}