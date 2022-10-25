use super::setup;
use crate::owm::geocoding::{api, structures::Coordinates};

#[tokio::test]
pub async fn get_city_coordinates() {
    let city_name: String = "Brussels".into();
    let credentials: setup::Credentials = setup::read_credentials();
    let coordinates: Coordinates = api::get_coordinates_by_location_name(city_name, credentials.omw_api_key).await;
    assert_eq!(coordinates.coord.lat, 50.8504);
    assert_eq!(coordinates.coord.lon, 4.3488);
    println!("{:?}", coordinates);
}
