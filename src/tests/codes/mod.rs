pub mod long;
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

/// Check the formatting of the code error object - it should contain the string
/// which was originally used to create the error message
#[test]
fn check_error_formatting() {
    let error_message = "This is a test".to_string();
    let error = CodeError(error_message.clone());
    if !format!("{}", error).contains(&error_message) {
        panic!("The printed error message did not contain the message that was specified when creating the error.");
    }
}

/// Check the zero-padding in the code-encoding - the code that comes out should be
/// the correct length, and should decode to the same number which was originally 
/// encoded
#[test]
fn check_zero_padding() {
    let num = 50_u128;
    let encoded = get_code_from_number(num, 16);
    // Check the length
    assert_eq!(encoded.len(), 16);
    // Check decoding is successful
    let decoded = get_number_from_code(encoded.iter().collect::<String>());
    assert_eq!(decoded, Some(num));
}