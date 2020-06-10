use std::fmt;
use std::error::Error;

pub mod short;
pub mod long;
pub mod crc;

/// NUM_ROWS defines how many rows are on the chart. However, adding more rows would also
/// require modifying the short and long code serialisation and deserialisation methods
/// because their conversion to user-facing codes always assumes that 5 rows are present.
/// In other words, it is not (yet) possible to add more rows by simply adjusting the two
/// variables below.
pub const NUM_ROWS: usize = 5;
pub const ROW_LOG_MAR: [f64; NUM_ROWS] = [44.0, 22.0, 11.0, 5.5, 2.75];
pub const NUM_OPTOTYPES_ON_ROW: [u32; NUM_ROWS] = [2, 4, 4, 4, 4];

/// Define a custom set of letters for use in base-32 codes. These are required becase
/// having "L" and "1" in the codes can get confusing, as do 'O' and '0'. The position 
/// in the array corresponds to the value of that char - i.e. 'E' is 4.
pub const BASE: [char; 32]= ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '2', '3', '4', '5', '6', '7', '8', '9'];

/// Convert a code string into the number it encodes, using u128 for the longer codes.
/// Returns null if the code is invalid or could not be parsed - for example, characters
/// with accents, ligatures, or characters not contained within the alphabet used for the
/// base 32 code generation.
pub fn get_number_from_code(code: String) -> Option<u128> {
    let sanitised_code = code.replace("-", "");
    let mut sum: u128 = 0;
    for (i, character) in sanitised_code.chars().enumerate() {
        if character == '-' { continue; };
        match get_character_value(character) {
            Some(current_val) => { 
                sum += (current_val as u128) * 32_u128.pow(sanitised_code.chars().count() as u32 - i as u32 - 1);
             },
            None => { return None; } 
        };
        
    }
    return Some(sum);
}

/// Convert a number into the characters in the base-32 encoded representation of that number.
/// This zero-pads the output to the length specified in len.
pub fn get_code_from_number(mut x: u128, len: usize) -> Vec<char> {
    let mut result = vec!['A'; len];
    for i in 0..len {
        if x > 0 {
            result[i] = BASE[(x % 32) as usize];
            x /= 32;
        } else {
            // Zero-pad the result to the required length
            result[i] = BASE[0];
        }
    }
    // The base-32 formatted representation was little-endian (LSB first) but the codes given to users
    // follow network byte order (big-endian, MSB first). Reversing the code vector allows this to take place.
    result.reverse();
    return result;
}

/// Finds the value of a particular character in the custom base 32 alphabet.
/// This is broken out into a separate function primarily to make the code more
/// idiomatic where this function is required.
pub fn get_character_value(character: char) -> Option<usize> {
    BASE.iter().position(|&c| c == character) 
}

/// This struct is used to represent errors when parsing codes given by users.
#[derive(Debug)]
pub struct CodeError(pub String);
impl fmt::Display for CodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}
impl Error for CodeError {}
