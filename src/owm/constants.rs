#![allow(dead_code)]

pub const OWM_URI: &'static str = "http://api.openweathermap.org";

// Weather endpoints
pub const ENDPOINT_WEATHER_FORECAST: &'static str = "/data/2.5/forecast";

/// Geocoding endpoints (Result caching is expected)
pub const ENDPOINT_GEOCODING_DIRECT: &'static str = "/geo/1.0/direct";
