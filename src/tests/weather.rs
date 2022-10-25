use super::setup::read_credentials;
use crate::owm::weather::structures::WeatherData;
use crate::owm::weather::api::get_weather_for_coordinates;
use crate::owm::geocoding::structures::Coordinates;
use crate::owm::geocoding::api::get_coordinates_by_location_name;

#[tokio::test]
pub async fn attempt_weather_data_retrieval() {
    let city_name: String = "Brussels".into();
    let credentials = read_credentials();
    let coords: Coordinates = get_coordinates_by_location_name(city_name, credentials.omw_api_key.clone()).await;
    let weather: WeatherData = get_weather_for_coordinates(coords.coord.lat, coords.coord.lon, credentials.omw_api_key).await;
    println!("{:#?}", weather);
}
