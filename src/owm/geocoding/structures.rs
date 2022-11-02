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

impl Coordinates {
    pub fn get_longitude(&self) -> f32 {
        self.coord.lon
    }

    pub fn get_latitude(&self) -> f32 {
        self.coord.lat
    }
}