use ux::{u1, u2, u4};
use crate::optotypes::OptotypeDefinition;
use crate::codes::{short::ShortCode, long::LongCode};

pub mod codes;
pub mod optotypes;

#[test]
fn always_passes() {
    assert_eq!(2 + 2, 4);
}

/// Use this shortcode ("RFD-CAM") for testing - defined centrally to avoid duplicating
pub fn get_test_shortcode() -> ShortCode {
    ShortCode {
        version: u1::new(0),
        optotype_definition: OptotypeDefinition::from(1),
        start_row: 229,
        offsets: [u4::new(1), u4::new(5), u4::new(6), u4::new(8)],
    }
}

// The test longcode is FFT7-CVBJ-8ZV8-ALWE, which is Aukland optotypes with a CRC of 10884
pub fn get_test_longcode() -> LongCode {
    LongCode {
        version: u2::new(0),
        optotype_definition: OptotypeDefinition::from(0),
        optotypes: vec![1, 7, 3, 6, 0, 1, 8, 7, 5, 0, 4, 2, 9, 5, 0, 2, 7, 0]
    }
}