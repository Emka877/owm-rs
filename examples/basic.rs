use owm_rs::prelude::*;

use ron::{self, de::from_reader};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Credentials {
    pub omw_api_key: String,
    pub city_name: String,
}

pub fn read_credentials() -> Credentials {
    let file_path: &'static str = "test_data/credentials.ron";
    let file = std::fs::File::open(file_path).expect("Failed opening credentials file.");
    let credentials: Credentials = match from_reader(file) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed opening credentials file: {}", e);
            std::process::exit(1);
        }
    };

    credentials
}

#[tokio::main]
async fn main() {
    let credentials = read_credentials();
    let coords_result = get_city_coordinates(credentials.city_name.clone(), credentials.omw_api_key.clone()).await;
    let coordinates = match coords_result {
        Ok(ok) => ok,
        Err(err) => {
            println!("Error trying to retrieve coordinates: {}", err);
            std::process::exit(1);
        }
    };

    let weather_result = get_weather_by_coordinates(coordinates.coord.lat, coordinates.coord.lon, credentials.omw_api_key).await;
    let weather = match weather_result {
        Ok(ok) => ok,
        Err(err) => {
            println!("Error trying to retrieve the weather: {}", err);
            std::process::exit(2);
        }
    };

    let temp: f32 = weather.main.temp;
    let temp_c: f32 = convert_fahrenheit_to_celsius(temp);
    println!("It is {}°C ({}°F) in {}.", temp_c, temp, credentials.city_name);
}
