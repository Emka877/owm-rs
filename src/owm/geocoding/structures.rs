use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CoordinatesValues {
    pub lon: f32,
    pub lat: f32,
}

#[derive(Deserialize, Debug)]
pub struct Coordinates {
    pub coord: CoordinatesValues,
}
