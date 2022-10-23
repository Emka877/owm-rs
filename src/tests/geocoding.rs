use super::setup;
use super::super::owm::geocoding::structures::Coordinates;

// #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[tokio::test]
pub async fn get_city_coordinates() {
    let city_name: String = "Brussels".into();
    let credentials: setup::Credentials = setup::read_credentials();
    let call: String = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}", 
        city_name, 
        credentials.omw_api_key
    );
    let response: reqwest::Response = reqwest::get(&call).await.expect("");
    let coordinates: Coordinates = response.json().await.expect("Could not parse the JSON.");
    assert_eq!(coordinates.coord.lon, 4.3488);
    assert_eq!(coordinates.coord.lat, 50.8504);
    println!("{:?}", coordinates);
}
