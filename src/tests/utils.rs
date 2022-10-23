use crate::utils::convert;

#[test]
fn test_fahrenheit_to_celsuis_conversion() {
    let result = convert::fahrenheit_to_celsius(32.0);
    assert_eq!(result, 0.0);
    let result2: f32 = convert::fahrenheit_to_celsius(50.0);
    assert_eq!(result2, 10.0);
}