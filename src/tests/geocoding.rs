use super::setup;
use crate::owm::geocoding::api;

#[tokio::test]
pub async fn get_city_coordinates() {
    let city_name: String = "Brussels".into();
    let credentials: setup::Credentials = setup::read_credentials();
    let coords_result = api::get_coordinates_by_location_name(city_name, credentials.omw_api_key).await;
    match coords_result {
        Ok(coords) => {
            assert_eq!(coords.get_latitude(), 50.846558);
            assert_eq!(coords.get_longitude(), 4.351697);
            println!("{:?}", coords);
        }
        Err(err) => {
            println!("Error getting coordinates: {}", err);
        }
    }
}

#[test]
pub fn get_city_coordinates_blocking() {
    let city_name: String = "Brussels".into();
    let credentials: setup::Credentials = setup::read_credentials();
    let coords_result = crate::owm_api::blocking::get_city_coordinates(city_name, credentials.omw_api_key);
    match coords_result {
        Ok(coords) => {
            assert_eq!(coords.get_latitude(), 50.846558);
            assert_eq!(coords.get_longitude(), 4.351697);
            println!("{:?}", coords);
        }
        Err(err) => {
            println!("Error getting coordinates: {}", err);
        }
    }
}
