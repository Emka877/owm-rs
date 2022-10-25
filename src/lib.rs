#[cfg(test)]
mod tests;
mod owm;
mod utils;

/// Gets the coordinates of a city by its name
/// 
/// # Arguments
/// * `city_name` - The city name to look for
/// 
/// * `api_key` - Your OWM API key
pub async fn get_city_coordinates(city_name: &'static str, api_key: String)
    -> owm::geocoding::structures::Coordinates 
{
    owm::geocoding::api
        ::get_coordinates_by_location_name(
            city_name.into(),
            api_key
        )
        .await
}

/// Gets the weather data given input coordinates.
/// 
/// # Arguments
/// `latitude` - The latitude of the target location
/// 
/// `longitude` - The longitude of the target location
/// 
/// `api_key` - Your API key
pub async fn get_weather_by_coordinates(latitude: f32, longitude: f32, api_key: String) 
    -> owm::weather::structures::WeatherData
{
    owm::weather::api
        ::get_weather_for_coordinates(latitude, longitude, api_key)
        .await
}

/// Converts a Fahrenheit value to Celsius
/// 
/// # Arguments
/// `fahrenheit` - The value to convert to °C
pub fn convert_fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    utils::convert::fahrenheit_to_celsius(fahrenheit)
}
