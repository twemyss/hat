pub mod short;

use crate::codes::*;

/// Check the first stage of code de-coding, which is to convert the code into 
/// a u128 value that represents the number (including CRC) encoded by
#[test]
fn convert_valid_code_to_number() {
    let number_from_valid_code = get_number_from_code("RFD-CAM".to_string()).unwrap();
    let expected_number: u128 = 508659723;
    assert_eq!(expected_number, number_from_valid_code);
}

/// Try to decode a code with an invalid character (a character not in the base 32 alphabet)
#[test]
fn convert_invalid_code_to_number() {
    assert_eq!(get_number_from_code("000-OOO".to_string()), None) 
}


/// Check the base array (that defines what number each character in a code
/// corresponds to) hasn't accidentally been tampered with (cat sitting 
/// on keyboard, etc). Only tests a single value, but that catches most obvious
/// issues. Other issues will hopefully be caught by checking code decoding.
#[test]
fn check_base_char_mapping() {
    assert_eq!(get_character_value('2'), Some(24))
}
