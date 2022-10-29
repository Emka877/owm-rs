mod owm;
#[cfg(test)]
mod tests;
mod utils;

pub mod owm_api {
    use crate::{owm, utils};
    use anyhow::Result;

    /// Gets the coordinates of a city by its name
    ///
    /// # Arguments
    /// * `city_name` - The city name to look for
    ///
    /// * `api_key` - Your OWM API key
    pub async fn get_city_coordinates(
        city_name: &'static str,
        api_key: String,
    ) -> Result<owm::geocoding::structures::Coordinates> {
        owm::geocoding::api::get_coordinates_by_location_name(city_name.into(), api_key).await
    }

    /// Gets the weather data given input coordinates.
    ///
    /// # Arguments
    /// * `latitude` - The latitude of the target location
    ///
    /// * `longitude` - The longitude of the target location
    ///
    /// * `api_key` - Your API key
    pub async fn get_weather_by_coordinates(
        latitude: f32,
        longitude: f32,
        api_key: String,
    ) -> Result<owm::weather::structures::WeatherData> {
        owm::weather::api::get_weather_for_coordinates(latitude, longitude, api_key).await
    }

    /// Combines searching for the coordinates of a city by name + Fetching the weather data for the location
    ///
    /// # Arguments
    /// * `city_name` - The city name to look for
    ///
    /// * `api_key` - Your OWM API key
    pub async fn get_weather_by_city(
        city_name: String,
        api_key: String,
    ) -> Result<owm::weather::structures::WeatherData> {
        let coordinates: owm::geocoding::structures::Coordinates =
            owm::geocoding::api::get_coordinates_by_location_name(city_name, api_key.clone()).await?;
        owm::weather::api::get_weather_for_coordinates(
            coordinates.coord.lat,
            coordinates.coord.lon,
            api_key,
        )
            .await
    }
}

pub mod owm_utils {
    use crate::utils;

    /// Converts a Fahrenheit value to Celsius
    ///
    /// # Arguments
    /// * `fahrenheit` - The value to convert to °C
    pub fn convert_fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
        utils::convert::fahrenheit_to_celsius(fahrenheit)
    }
}

pub mod prelude {
    pub use crate::owm_api::*;
    pub use crate::owm_utils::*;
}
