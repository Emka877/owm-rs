#![allow(dead_code)]

/// OMW base URI
pub const OWM_URI: &str = "https://api.openweathermap.org";
/// Geocoding endpoints (Result caching is expected)
pub const ENDPOINT_GEOCODING_DIRECT: &str = "/geo/1.0/direct";
/// Current weather endpoint
pub const ENDPOINT_WEATHER: &str = "/data/2.5/weather";
/// Weather forecast endpoint
pub const ENDPOINT_WEATHER_FORECAST: &str = "/data/2.5/forecast";
