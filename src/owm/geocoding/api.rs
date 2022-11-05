#![allow(dead_code)]
use crate::owm::constants::{ENDPOINT_GEOCODING_DIRECT, OWM_URI};
use crate::owm::geocoding::structures::Coordinates;

pub async fn get_coordinates_by_location_name(
    city_name: String,
    api_key: String,
) -> Result<Coordinates, reqwest::Error> {
    let call: String = format!("{}{}?q={}&appid={}", OWM_URI, ENDPOINT_GEOCODING_DIRECT, city_name, api_key);
    let response: reqwest::Response = reqwest::get(&call).await.expect("Error");
    Ok(response.json().await?)
}
