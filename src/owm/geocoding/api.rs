#![allow(dead_code)]

use crate::owm::geocoding::structures::Coordinates;

pub async fn get_coordinates_by_location_name(city_name: String, api_key: String) -> Coordinates {
    let call: String = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}", 
        city_name, 
        api_key
    );
    let response: reqwest::Response = reqwest::get(&call).await.expect("Error");
    response.json().await.expect("Could not parse the JSON.")
}
