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
    let credentials: Credentials = read_credentials();
    let coords: Coordinates =
        get_coordinates_by_location_name(city_name, credentials.omw_api_key.clone())
            .await
            .expect("Could not retrieve coordinates");
    let weather: WeatherData =
        get_weather_for_coordinates(coords.get_latitude(), coords.get_longitude(), credentials.omw_api_key)
            .await
            .expect("Could not retrieve weather");
    let celsius: f32 = crate::utils::convert::kelvin_to_celsius(weather.main.temp);
    println!("Temp in Brussels: {:.2}°C", celsius);
}

#[tokio::test]
pub async fn attempt_weather_data_retrieval_with_shortcut_fn() {
    let city_name: String = "Brussels".into();
    let credentials: Credentials = read_credentials();
    let weather: Result<WeatherData, reqwest::Error> =
        get_weather_by_city(city_name, credentials.omw_api_key).await;
    match weather {
        Ok(w) => {
            println!("{:.2}", w.main.temp);
        },
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

#[test]
pub fn fetch_weather_by_city_name_blocking() {
    let city_name: String = "Brussels".into();
    let credentials: Credentials = read_credentials();
    let weather: Result<WeatherData, reqwest::Error> =
        crate::owm_api::blocking::get_weather_by_city(city_name, credentials.omw_api_key);
    match weather {
        Ok(w) => {
            println!("{:.2}", w.main.temp);
        },
        Err(err) => {
            println!("Error: {}", err);
        }
    }
}

// #[test]
// pub fn fetch_weather_by_coords_blocking() {
//     let city_name: String = "Brussels".into();
//     let credentials: Credentials = read_credentials();
//     let weather: Result<WeatherData, reqwest::Error> =
//         crate::owm_api::blocking::get_weather_by_city(city_name, credentials.omw_api_key);
//     match weather {
//         Ok(w) => {
//             println!("{:.2}", w.main.temp);
//         },
//         Err(err) => {
//             println!("Error: {}", err);
//         }
//     }
// }