use dotenvy::dotenv;
use serde::Deserialize;
use owm_rs::owm_api::{get_city_coordinates, get_weather_by_coordinates};

#[derive(Deserialize)]
pub struct Credentials {
    pub omw_api_key: String,
    pub city_name: String,
}

pub fn read_credentials() -> Credentials {
    let _ = dotenv().expect("Cannot load the .env file.");
    Credentials {
        omw_api_key: std::env::var("OWM_API_KEY").expect("Cannot load the OWM_API_KEY env var."),
        city_name: std::env::var("CITY").expect("Cannot load the CITY env var."),
    }
}

#[tokio::main]
async fn main() {
    let credentials = read_credentials();
    let coords_result = get_city_coordinates(
        credentials.city_name.clone(),
        credentials.omw_api_key.clone(),
    ).await;

    let coordinates = match coords_result {
        Ok(ok) => ok,
        Err(err) => {
            println!("Error trying to retrieve coordinates: {err}");
            std::process::exit(1);
        }
    };

    let weather_result = get_weather_by_coordinates(
        coordinates.get_latitude(),
        coordinates.get_longitude(),
        credentials.omw_api_key,
    ).await;

    let weather = match weather_result {
        Ok(ok) => ok,
        Err(err) => {
            println!("Error trying to retrieve the weather: {err}");
            std::process::exit(2);
        }
    };

    let temp: f32 = weather.main.temp;
    let temp_c: f32 = owm_rs::owm_utils::convert::kelvin_to_celsius(temp);
    println!(
        "It is {:.2}°C ({:.2}°F) in {}.",
        temp_c, temp, credentials.city_name
    );
}
