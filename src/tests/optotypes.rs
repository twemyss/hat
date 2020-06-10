use crate::optotypes::{OptotypeRow, OptotypeDefinition, OptotypeArrangement, DEFAULT_OPTOTYPES};
use crate::tests::get_test_shortcode;

pub fn get_known_arrangement() -> OptotypeArrangement {
    OptotypeArrangement {
        rows: vec![ 
            OptotypeRow { text_size: 5.0, border_size: 1.0, optotypes: vec![7, 8]},
            OptotypeRow { text_size: 4.0, border_size: 0.8, optotypes: vec![5, 6, 3, 9]},
            OptotypeRow { text_size: 3.0, border_size: 0.6, optotypes: vec![4, 5, 2, 8]},
            OptotypeRow { text_size: 2.0, border_size: 0.4, optotypes: vec![0, 1, 8, 4]},
            OptotypeRow { text_size: 1.0, border_size: 0.2, optotypes: vec![9, 0, 7, 3]},
        ],
        code: "RFD-CAM".to_string(),
        optotype_definition: OptotypeDefinition::from(1)
    }
}

#[test]
fn check_arrangement() {
    let obtained_arrangement = OptotypeArrangement::from(get_test_shortcode());
    let known_arrangement = get_known_arrangement();
    assert_eq!(known_arrangement, obtained_arrangement);
}

/// Check the fallback into default optotypes, when the optotype ID is not given
#[test]
fn check_fallback() {
    assert_eq!(OptotypeDefinition::from(50), OptotypeDefinition::from(DEFAULT_OPTOTYPES));
}

/// Check the debug trait on OptotypeDefinitions
#[test]
fn check_debug() {
    println!("{:?}", OptotypeDefinition::from(50));
}

/// Check the PartialEq trait on OptotypeRows, which is required to check if 
/// two rows are the same
#[test]
fn check_compare_optotype_row() {
    let row_one = OptotypeRow {
        text_size: 1.0,
        border_size: 1.0,
        optotypes: vec![1, 2, 3]
    };
    let row_two = OptotypeRow {
        text_size: 1.0,
        border_size: 1.0,
        optotypes: vec![1, 2, 3]
    };
    if row_one != row_two {
        panic!("Two identical rows did not appear to be the same when compared.");
    }
}