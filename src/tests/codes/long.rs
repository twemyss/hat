use crate::codes::long::LongCode;
use crate::tests::get_test_longcode;
use crate::optotypes::{OptotypeArrangement, OptotypeDefinition, OptotypeRow};
use ux::u4;
use std::str::FromStr;

/// Check the parsing of a particular long code with known stored data.
#[test]
fn parse_code() {
    let parsed_longcode = LongCode::from_str("FFT7-CVBJ-8ZV8-ALWE").unwrap();
    let expected_value = get_test_longcode();
    assert_eq!(parsed_longcode, expected_value);
}

/// Parse an invalid code and check that it gives a CodeError.
#[test]
fn parse_invalid_code() {
    let parsed_longcode = LongCode::from_str("000-OOO0-OOOO-00OO");
    match parsed_longcode {
        Ok(_) => { panic!("A long code containing invalid characters was obtained."); },
        Err(e) => { assert_eq!(format!("{}", e), "Error: Failed to parse long code. It may have contained invalid characters.".to_string())}
    }
}

/// Attempt to parse a code encoding a number which is out of the range 
/// for the values encoded by a long code.
#[test]
fn parse_out_of_range_code() {
    let parsed_longcode = LongCode::from_str("9999-9999-9999-9999-9999");
    match parsed_longcode {
        Ok(_) => { panic!("A long code encoding an out-of-range value was successfully parsed."); },
        Err(e) => { assert_eq!(format!("{}", e), "Error: The long code encoded a value which was outside the allowed range.".to_string())}
    }
}

/// Attempt to parse a code with an invalid CRC.
#[test]
fn parse_invalid_crc() {
    let parsed_longcode = LongCode::from_str("FFT7-CVBL-8ZV8-ALWE");
    match parsed_longcode {
        Ok(_) => { panic!("A long code with an invalid CRC was successfully parsed."); },
        Err(e) => { assert_eq!(format!("{}", e), "Error: The long code was entered incorrectly (CRC mismatch. Code contained CRC 10884, but the calculated value was 54652).".to_string())}
    }
}

/// Check the CRC calculation for an example long code with a known CRC
#[test]
fn check_crc() {
    let parsed_longcode = get_test_longcode();
    assert_eq!(parsed_longcode.get_crc(), 10884 as u16);
}

/// Check an intermediary stage of the coding/decoding: the numerical representation
/// of the body of the message, which contains the optotypes
#[test]
fn check_body() {
    let expected_value: u64 = 186403593955270270;
    let actual_value = get_test_longcode().get_body();
    assert_eq!(actual_value, expected_value);
}

/// Make sure that headers are working
#[test]
fn check_header() {
    let expected_value: u4 = u4::new(0b0000);
    let actual_value = get_test_longcode().get_header_without_crc();
    assert_eq!(actual_value, expected_value);
}

/// Make sure that serialisation in working for long codes.
#[test]
fn check_serialisation() {
    let expected_value = "FFT7-CVBJ-8ZV8-ALWE".to_string();
    let serialised_code = get_test_longcode().to_string();
    assert_eq!(serialised_code, expected_value);
}

/// Check the debug serialisation
#[test]
fn check_debug() {
    println!("{:?}", get_test_longcode());
}

/// Check converting the longcode into an optotype arrangement
#[test]
fn check_longcode_into_optotype_arrangement() {
    let longcode = LongCode::from_str("FFT7-CVBJ-8ZV8-ALWE").unwrap();
    let expected_arrangement = expected_longcode_arrangement();
    assert_eq!(expected_arrangement, OptotypeArrangement::from(longcode));
}

// Check converting an OptotypeArrangement into a LongCode
#[test]
fn check_optotype_arrangement_into_longcode() {
    let longcode = LongCode::from(expected_longcode_arrangement());
    let expected_value = get_test_longcode();
    assert_eq!(longcode, expected_value);
}

/// Get the expected arrangement for default longcode (FFT7-CVBJ-8ZV8-ALWE)
pub fn expected_longcode_arrangement() -> OptotypeArrangement {
    OptotypeArrangement {
        code: "FFT7-CVBJ-8ZV8-ALWE".to_string(),
        optotype_definition: OptotypeDefinition::from(0),
        rows: vec![
            OptotypeRow {
                optotypes: vec![7, 0],
                text_size: 5.0,
                border_size: 1.0
            },
            OptotypeRow {
                optotypes: vec![9, 5, 0, 2],
                text_size: 4.0,
                border_size: 0.8
            },
            OptotypeRow {
                optotypes: vec![5, 0, 4, 2],
                text_size: 3.0,
                border_size: 0.6
            },
            OptotypeRow {
                optotypes: vec![0, 1, 8, 7],
                text_size: 2.0,
                border_size: 0.4
            },
            OptotypeRow {
                optotypes: vec![1, 7, 3, 6],
                text_size: 1.0,
                border_size: 0.2
            }
        ]
    }
}