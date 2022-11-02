use super::setup::read_credentials;
use crate::owm_api::get_weather_by_city;
use crate::owm::geocoding::api::get_coordinates_by_location_name;
use crate::owm::geocoding::structures::Coordinates;
use crate::owm::weather::api::get_weather_for_coordinates;
use crate::owm::weather::structures::WeatherData;
use crate::tests::setup::Credentials;

#[tokio::test]
pub async fn attempt_weather_data_retrieval() {
    let city_name: String = "Brussels".into();
    let credentials = read_credentials();
    let coords: Coordinates =
        get_coordinates_by_location_name(city_name, credentials.omw_api_key.clone())
            .await
            .expect("Could not retrieve coordinates");
    let weather: WeatherData =
        get_weather_for_coordinates(coords.get_latitude(), coords.get_longitude(), credentials.omw_api_key)
            .await
            .expect("Could not retrieve weather");
    // println!("{:#?}", weather);
    let celsius: f32 = crate::utils::convert::fahrenheit_to_celsius(weather.main.temp);
    println!("Temp in Brussels: {}°C", celsius);
}

#[tokio::test]
pub async fn attempt_weather_data_retrieval_with_shortcut_fn() {
    let city_name: String = "Brussels".into();
    let credentials: Credentials = read_credentials();
    let weather: anyhow::Result<WeatherData> =
        get_weather_by_city(city_name, credentials.omw_api_key).await;
    match weather {
        Ok(_) => {}
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}
