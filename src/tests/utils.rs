use crate::utils::convert;

#[test]
fn test_fahrenheit_to_celsius_conversion() {
    let result = convert::fahrenheit_to_celsius(32.0);
    assert_eq!(result, 0.0);
    let result2: f32 = convert::fahrenheit_to_celsius(50.0);
    assert_eq!(result2, 10.0);
}

#[test]
fn test_kelvin_to_celsius() {
    let test_zero = convert::kelvin_to_celsius(273.15);
    assert_eq!(test_zero, 0.0);
    let test_ten = convert::kelvin_to_celsius(283.15);
    assert_eq!(test_ten, 10.0);
}

#[test]
fn test_kelvin_to_fahrenheit() {
    let test_32 = convert::kelvin_to_fahrenheit(273.15);
    assert_eq!(test_32, 32.0);
    let test_50 = convert::kelvin_to_fahrenheit(283.15);
    assert_eq!(test_50, 50.0);
}
