#![allow(dead_code)]

/// Converts a Fahrenheit value to Celsius
///
/// # Arguments
/// * `fahrenheit` - The value to convert to °C
pub fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

/// Converts Kelvin to Celsius
///
/// # Arguments
/// `kelvin` - The Kelvin value to convert
pub fn kelvin_to_celsius(kelvin: f32) -> f32 {
    kelvin - 273.15
}

/// Converts Kelvin to Fahrenheit
///
/// # Arguments
/// `kelvin` - The Kelvin value to convert
pub fn kelvin_to_fahrenheit(kelvin: f32) -> f32 {
    (kelvin - 273.15) * (9.0 / 5.0) + 32.0
}
