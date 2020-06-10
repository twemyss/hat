use crate::optotypes::{OptotypeDefinition, OptotypeArrangement};
use crate::codes::{NUM_OPTOTYPES_ON_ROW, NUM_ROWS, crc, CodeError};
use ux::{u2, u4};
use std::str::FromStr;
use std::str;
use rand::Rng;

/// Represents the data encoded in a long HAT code. Long HAT codes are slightly different in format the short HAT codes.
/// The idea behind these codes is that they can be used by someone to check their own answers. They allow encoding more potential
/// permutations of optotypes than the short codes, so are better suited for regular tests.
/// 
/// The format of version 0 codes is given below. It is slightly unconventional in that the header is at the end of the message.
/// 
/// The v0 codes encode information in a custom base-32 alphabet, with some confusing letters missing (for example, O and 0 are omitted).
/// The codes are formatted as XXXX-XXYY-YYYY-HEAD, where HEAD is the header. XXXXXX and YYYYYY are the two fields of the body, which each
/// encode the identities of 9 letters/optotypes on the vision chart.
/// 
/// The header (20 bits, 4 code characters) consists of:
///     - a two bit version number (currently 0)
///     - a two bit identifier for an OptotypeDefinition
///     - a 16 bit CRC-16 checksum of the header fields above, and the body below. This checksum field is then inserted
///       after the header fields above, shifting the body fields along.
/// 
/// The body, which is 60 bits, consists of:
///     - 2 x 30 bit (6 code character) fields, which each encode 9 letters on the vision chart
///     - The idea behind this structure was that they can be added to the code as needed - if a further 9 optotypes are needed on the chart in the
///       future, another one of these fields can be added. The use of 30 bit codes also worked around the lack of support in JavaScript for integers of
///       >53 bits, which would have led to overflow issues were the fields combined (without the use of BigInt or js-ctypes).
#[derive(Debug, PartialEq)]
pub struct LongCode {
    /// The version field determines in which format the information within the code is encoded.
    /// At present, only version 0 is implemented, and so the version field is ignored for now.
    pub version: u2,
    /// Represents the set of potential optotypes encoded by a particular code
    pub optotype_definition: OptotypeDefinition,
    /// Represents the exact optotypes shown in the chart. This is just a list, starting at the bottom row
    /// of the chart and going from left to right, then wrapping up to the next row.
    pub optotypes: Vec<u8>
}

/// This serialises a long code (string) representation of the values stored within this ShortCode object. 
/// The convention is to format the field in blocks of four digits, because these are easier to remember and
/// check when typing them in (see ShortCode ToString implementation for more discussion of this).
impl ToString for LongCode {
    fn to_string(&self) -> String {
        // Join together the individual fields. The casts get a bit messy here - in the future, the author should review the cost vs benefits of using 
        // custom integer
        let num: u128 = ((self.get_body() as u128) << 20) + u128::from(u32::from(self.get_header_without_crc()) << 16) + u128::from(self.get_crc());
        let mut code = super::get_code_from_number(num, 16);
        // Add in the dashes
        for i in vec![4, 9, 14] {
            code.insert(i, '-');
        }
        return code.iter().collect::<String>();
    }
}

/// Parses long codes and extract the fields and OptotypeArrangement which is encoded within it.
/// May return an error if the CRC fails or there are invalid characters. Code parsing is agnostic
/// to the '-' characters within the code - a user can enter as many or as few as they wish.
impl FromStr for LongCode {
    type Err = CodeError;
    fn from_str(code: &str) -> Result<Self, Self::Err> {
        // Convert the base-32 code into the number it encodes
        let num = match super::get_number_from_code(code.to_string()) {
            Some(binary) => { binary },
            None => { return Err(CodeError("Failed to parse long code. It may have contained invalid characters.".into())); }
        };
        // Check the number encoded: it shouldn't exceed 2^80 - 1 unless something has gone very wrong
        if num > 2_u128.pow(80) - 1 {
            return Err(CodeError("The long code encoded a value which was outside the allowed range.".into()));
        }
        // Take out the fields
        let version = ((0b11 << 16) & num) as u8;
        let optotype_definition = OptotypeDefinition::from(((0b11 << 18) & num) as u32);
        // Work out how many 30-bit blocks of letter definitions the message contains. This should be 2, unless 
        // the optotype definitions are changed
        let num_letter_blocks = get_num_body_blocks();
        let mut optotype_list: Vec<u8> = Vec::new();
        let num_possible_optotypes = optotype_definition.optotypes.len() as u128;
        for block in 0..num_letter_blocks {
            // Parse the block and add it to the list
            let shift = 20 + block * 30;
            let mut block_total = (((2_u128.pow(30) - 1) << shift) & num) >> shift;
            for _ in 0..9 {
                if block_total > 0 {
                    let current_optotype_id = (block_total % num_possible_optotypes) as u8;
                    optotype_list.push(current_optotype_id);
                    block_total /= num_possible_optotypes;
                } else {
                    optotype_list.push(0);
                }
            }
            if block_total != 0 {
                return Err(CodeError("An unexpected value was encountered when decoding the optotypes encoded by this code.".into()));
            }
        }
        optotype_list.reverse();
        // Store all the fields
        let processed_code = LongCode {
            version: u2::new(version),
            optotype_definition: optotype_definition,
            optotypes: optotype_list
        };
        // Check the CRC is correct. The CRC is just the last 16 bits of the message, so simply casting the message to a u16 will
        // truncate the message to extract the CRC.
        let message_crc = num as u16;
        let calculated_crc = processed_code.get_crc();
        if message_crc != calculated_crc {
            return Err(super::CodeError(format!("The long code was entered incorrectly (CRC mismatch. Code contained CRC {}, but the calculated value was {}).", message_crc, calculated_crc)));
        }
        Ok(processed_code)
    }
}

/// This parsing is slightly different to shortcodes because uses the concept of a header and data body, the latter
/// of which can potentially have variable length, if the data type is changed from a u64 and the number of optotypes
/// on each row is altered in the optotypes configuration file.
impl LongCode {
    pub fn get_header_without_crc(&self) -> u4 {
        // Due to the implementation of u2 in the package used to provide the u2 type, the conversion must occur via
        // u16 intermediates, which must then be cast into u8 for conversion into the u4 which is returned.
        return u4::new((u16::from(self.version) << 2 | (self.optotype_definition.id as u16)) as u8);
    }
    /// Obtain the body of the LongCode, which is that part that encodes the optotypes shown on the page
    pub fn get_body(&self) -> u64 {
        let mut body: u64 = 0;
        let mut optotype_idx = 0;
        for block_num in (0..get_num_body_blocks()).rev() {
            // For each block, sum the optotype indices
            let mut sum_block: u64 = 0;
            for i in (0..9).rev() {
                sum_block += (self.optotypes[optotype_idx] as u64) * 10_u64.pow(i);
                optotype_idx += 1;
            }
            // Then add the new block into the body
            body = body | (sum_block << (30*block_num))
        }
        return body;
    }
    /// This is a CRC-16/ARC of the body (optotypes) and the header of the long code.
    pub fn get_crc(&self) -> u16 {
        let mut crc: u16 = 0;
        // For compatibility reasons, we first parse this into a string representation of the binary
        // When the test is transitioned to v1 shortcodes, this should be be changed to just take each
        // byte of the actual underlying number (the u24 numerical representation), rather than doing
        // a checksum of the characters in the string representation of the binary.
        for byte in format!("{:0>60b}{:0>4b}", self.get_body(), self.get_header_without_crc()).chars() {
            crc = (crc::CRC16_TABLE[((crc ^ (byte as u16)) & 0xff) as usize]  ^ (crc >> 8)) & 0xffff;
        }
        // Dividing and rounding is likely a bit slower than doing a bitshift would be,
        // but it maintains backwards compatibility with codes already issued in the older software.
        return (crc as f32).round() as u16
    }
    /// Generates a random new shortcode for the specificed optotypes
    pub fn generate_random(optotypes: OptotypeDefinition) -> LongCode {
        // Generate row offsets
        let mut optotype_list: Vec<u8> = Vec::new();
        for row in 0..NUM_ROWS {
            let mut row_optotypes: Vec<u8> = Vec::new();
            let mut num_unique_optotypes_on_row = 0;
            while num_unique_optotypes_on_row < NUM_OPTOTYPES_ON_ROW[row] {
                // Generate a random optotype
                let optotype = rand::thread_rng().gen_range(0, optotypes.optotypes.len()) as u8;
                // Check if it was already on row
                if !row_optotypes.contains(&optotype) {
                    // If not, then append it and move on
                    row_optotypes.push(optotype);
                    num_unique_optotypes_on_row += 1;
                }
                // Otherwise, generate a new optotype to try
            }
            optotype_list.append(&mut row_optotypes);
        }
        optotype_list.reverse();
        // Return representation of the shortcode
        LongCode {
            version: u2::new(0),
            optotype_definition: optotypes,
            optotypes: optotype_list
        }
    }
}

/// This function is a helper to calculate the number of body blocks (each storing 9 optotypes) needed to store
/// the arrangement of optotypes that the user has specified.
pub fn get_num_body_blocks() -> usize {
    ((NUM_OPTOTYPES_ON_ROW.iter().sum::<u32>() as f64) / 9.0).round() as usize
}

/// Convert an OptotypeArrangement into the LongCode which encodes it. Every possible
/// arrangement of optotypes has a corresponding LongCode, so this is always possible - unlike
/// for the shorter ("telephone") codes which can only represent a subset of possible arrangements.
impl From<OptotypeArrangement> for LongCode {
    fn from(optotype_arrangement: OptotypeArrangement) -> Self {
        // Move all the optotypes in the arrangement into a list
        let mut optotype_list: Vec<u8> = Vec::new();
        for optotype_row in optotype_arrangement.rows {
            for optotype in optotype_row.optotypes.iter().rev() {
                optotype_list.push(*optotype);
            }
        }
        optotype_list.reverse();
        // Then obtain the LongCode object
        LongCode {
            version: u2::new(0_u8),
            optotype_definition: optotype_arrangement.optotype_definition,
            optotypes: optotype_list
        }
    }
}
