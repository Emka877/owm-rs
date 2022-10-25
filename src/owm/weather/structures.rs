#![allow(dead_code)]
use serde::Deserialize;
use super::super::geocoding::structures::CoordinatesValues;

#[derive(Deserialize, Debug)]
pub struct WeatherData {
    coord: CoordinatesValues,
    weather: Vec<WeatherNode>,
    base: String,
    main: WeatherMainNode,
    visibility: i32,
    wind: WeatherWindNode,
    #[serde(default)]
    rain: WeatherRainNode,
    clouds: WeatherCloudNode,
    dt: i32,
    sys: WeatherSystemNode,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32,
}

#[derive(Deserialize, Debug)]
pub struct WeatherNode {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Deserialize, Debug)]
pub struct WeatherMainNode {
    temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: i32,
    humidity: i32,
    #[serde(default)]
    sea_level: i32,
    #[serde(default)]
    grnd_level: i32,
}

#[derive(Deserialize, Debug)]
pub struct WeatherWindNode {
    speed: f32,
    deg: i32,
    #[serde(default)]
    gust: f32,
}

#[derive(Deserialize, Debug)]
pub struct WeatherRainNode {
    #[serde(rename = "1h")]
    one_hour: f32,
}

impl Default for WeatherRainNode {
    fn default() -> Self {
        Self { one_hour: Default::default() }
    }
}

#[derive(Deserialize, Debug)]
pub struct WeatherCloudNode {
    all: i32,
}

#[derive(Deserialize, Debug)]
pub struct WeatherSystemNode {
    #[serde(rename = "type")]
    sys_type: i32,
    id: i32,
    country: String,
    sunrise: i32,
    sunset: i32,
}
