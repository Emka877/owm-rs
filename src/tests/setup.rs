use dotenvy::dotenv;

pub struct Credentials {
    pub owm_api_key: String,
    pub city: String,
}

pub fn load_credentials() -> Credentials {
    let _ = dotenv().expect(".env file not found.");
    Credentials {
        owm_api_key: std::env::var("OWM_API_KEY").expect("Cannot load owm_api_key from .env"),
        city: std::env::var("CITY").expect("Cannot load city from .env"),
    }
}