use crate::codes::{NUM_ROWS, short::ShortCode};
use crate::optotypes::OptotypeDefinition;
use crate::tests::get_test_shortcode;
use ux::{u6,u30};
use std::str::FromStr;

/// This test just makes sure the random shortcode is not panicking and that
/// it can subsequently be converted into a string.
#[test]
fn random_short_code() {
    let new_shortcode = ShortCode::generate_random(OptotypeDefinition::from(1));
    // Check serialisation
    assert_eq!(new_shortcode.to_string().len(), 7);
    // Check the optotype definition is correct
    assert_eq!(new_shortcode.optotype_definition.id, 1);
    // Check the number of rows is correct
    assert_eq!(new_shortcode.offsets.len(), NUM_ROWS - 1);
}

/// Check the parsing of a particular shortcode with known stored data.
#[test]
fn parse_code() {
    let parsed_shortcode = ShortCode::from_str("RFD-CAM").unwrap();
    let expected_value = get_test_shortcode();
    assert_eq!(parsed_shortcode, expected_value);
}

/// Parse an invalid code and check that it gives a CodeError.
#[test]
fn parse_invalid_code() {
    let parsed_shortcode = ShortCode::from_str("000-OOO");
    match parsed_shortcode {
        Ok(_) => { panic!("A short code containing invalid characters was obtained."); },
        Err(e) => { assert_eq!(format!("{}", e), "Error: Failed to parse short code. It may have contained invalid characters.".to_string())}
    }
}

/// Attempt to parse a code encoding a number which is out of the range 
/// for the values encoded by a shortcode.
#[test]
fn parse_out_of_range_code() {
    let parsed_shortcode = ShortCode::from_str("9999999");
    match parsed_shortcode {
        Ok(_) => { panic!("A short code encoding an out-of-range value was successfully parsed."); },
        Err(e) => { assert_eq!(format!("{}", e), "Error: Code did not encode a number within a valid range.".to_string())}
    }
}

/// Attempt to parse a code which was constructed to have a row offset
/// specification which is invalid for charts with four rows.
#[test]
fn parse_bad_offsets_code() {
    let parsed_shortcode = ShortCode::from_str("LH9-98Y");
    match parsed_shortcode {
        Ok(_) => { panic!("A short code encoding an out-of-range value was successfully parsed."); },
        Err(e) => { assert_eq!(format!("{}", e), "Error: The row offsets number appeared to be invalid.".to_string())}
    }
}

/// Attempt to parse a code with an invalid CRC.
#[test]
fn parse_invalid_crc() {
    let parsed_shortcode = ShortCode::from_str("RFD-CAL");
    match parsed_shortcode {
        Ok(_) => { panic!("A short code encoding an out-of-range value was successfully parsed."); },
        Err(e) => { assert_eq!(format!("{}", e), "Error: The short code was entered incorrectly (CRC mismatch. Code contained CRC 10, but the calculated value was 11).".to_string())}
    }
}

/// Check the CRC calculation for an example shortcode with a known
/// CRC value of 11. This CRC value is used to decrease the chance of
/// codes getting changed by accident when they're read out over the 
/// phone and then typed in at the other end.
#[test]
fn check_crc() {
    let parsed_shortcode = get_test_shortcode();
    assert_eq!(parsed_shortcode.get_crc(), u6::new(11));
}


#[test]
fn check_numerical_representation() {
    let expected_value = u30::new(508659723);
    let actual_value = get_test_shortcode().get_numerical_representation();
    assert_eq!(actual_value, expected_value);
}

/// Make sure that serialisation in working for shortcodes - i.e. the
/// encoded information can be converted back into a shortcode that
/// encodes the information. This is required for displaying the short 
/// code of a particular PDF within that PDF.
#[test]
fn check_serialisation() {
    let expected_value = "RFD-CAM".to_string();
    let serialised_code = get_test_shortcode().to_string();
    assert_eq!(serialised_code, expected_value);
}