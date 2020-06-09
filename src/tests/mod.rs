use ux::{u1, u4};
use crate::optotypes::OptotypeDefinition;
use crate::codes::short::ShortCode;

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