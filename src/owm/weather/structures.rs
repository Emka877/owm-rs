use super::super::geocoding::structures::CoordinatesValues;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WeatherData {
    pub coord: CoordinatesValues,
    pub weather: Vec<WeatherNode>,
    pub base: String,
    pub main: WeatherMainNode,
    pub visibility: i32,
    pub wind: WeatherWindNode,
    #[serde(default)]
    pub rain: WeatherRainNode,
    pub clouds: WeatherCloudNode,
    pub dt: i32,
    pub sys: WeatherSystemNode,
    pub timezone: i32,
    pub id: i32,
    pub name: String,
    pub cod: i32,
}

#[derive(Deserialize, Debug)]
pub struct WeatherNode {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize, Debug)]
pub struct WeatherMainNode {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: i32,
    pub humidity: i32,
    #[serde(default)]
    pub sea_level: i32,
    #[serde(default)]
    pub grnd_level: i32,
}

#[derive(Deserialize, Debug)]
pub struct WeatherWindNode {
    pub speed: f32,
    pub deg: i32,
    #[serde(default)]
    pub gust: f32,
}

#[derive(Deserialize, Debug, Default)]
pub struct WeatherRainNode {
    #[serde(rename = "1h")]
    pub one_hour: f32,
}

#[derive(Deserialize, Debug)]
pub struct WeatherCloudNode {
    pub all: i32,
}

#[derive(Deserialize, Debug)]
pub struct WeatherSystemNode {
    #[serde(rename = "type")]
    pub sys_type: i32,
    pub id: i32,
    pub country: String,
    pub sunrise: i32,
    pub sunset: i32,
}
