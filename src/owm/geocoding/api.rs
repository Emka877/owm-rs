#![allow(dead_code)]

use crate::owm::geocoding::structures::Coordinates;
use crate::owm::constants::{OWM_URI, ENDPOINT_WEATHER};

pub async fn get_coordinates_by_location_name(city_name: String, api_key: String) -> Coordinates {
    let call: String = format!("{OWM_URI}{ENDPOINT_WEATHER}?q={city_name}&appid={api_key}");
    let response: reqwest::Response = reqwest::get(&call).await.expect("Error");
    response.json().await.expect("Could not parse the JSON.")
}
