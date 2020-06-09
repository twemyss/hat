use crate::optotypes::OptotypeDefinition;
use crate::codes::{NUM_ROWS, crc, CodeError};
use rand::Rng;
use ux::{u1,u4,u6,u24,u30};
use std::str::FromStr;
use std::str;

/// Represents the data encoded in a short HAT code. The purpose of a short HAT code is to encode
/// the data stored in an vision chart into a 6 character base-32 code which can easily be given
/// over the phone.
#[derive(Debug, PartialEq)]
pub struct ShortCode {
    /// Version is either 0 or 1. This determines what method was used to parse the code.
    /// At present, only version 0 is implemented, and so the version field is ignored for now.
    pub version: u1,
    /// Represents the set of optotypes encoded by a particular code. By default, this is either "child", or "adult"
    pub optotype_definition: OptotypeDefinition,
    /// Represents the "seed" used to generate the chart, which ranges from 0 to 255
    pub start_row: u8,
    /// Stores an offset for each row
    pub offsets: [u4; NUM_ROWS-1]
}

/// This serialises a short code (string) representation of the values stored within this ShortCode object. 
/// The convention is to have a "-" in the middle, because it splits the code into two sets of 3 digits, which
/// are easier to remember (via chunking, see https://doi.org/10.1037/h0043158 for a discussion).
impl ToString for ShortCode {
    fn to_string(&self) -> String {
        let mut x = u32::from(self.get_numerical_representation());
        let mut result = vec![];
        loop {
            result.push(super::BASE[(x % 32) as usize]);
            x /= 32;
            if x == 0 {
                break;
            }
        }
        result.insert(3, '-');
        return result.iter().rev().collect::<String>();
    }
}

/// Parses short codes and extract the fields they encode.
/// May return an error if the CRC fails or there are invalid characters. Code parsing is agnostic
/// to the '-' characters within the code - a user can enter as many or as few as they wish.
impl FromStr for ShortCode {
    type Err = CodeError;
    
    fn from_str(code: &str) -> Result<Self, Self::Err> {
        // Convert the base-32 code into the number it encodes
        let num = match super::get_number_from_code(code.to_string()) {
            Some(binary) => { binary },
            None => { return Err(super::CodeError("Failed to parse short code. It may have contained invalid characters.".into())); }
        };
        // Double check the encoded number is in the right range (0 to 2^30 - 1)
        if num > 1073741823 {
            return Err(super::CodeError("Code did not encode a number within a valid range.".into()));
        }
        // Calculate the fields via bitshifts - they're all fixed-width
        let version = u1::from(num & (1 << 29) != 0);
        let optotype_definition = OptotypeDefinition::from(u32::from(num & (1 << 28) != 0));
        let start_row = ((num & 255 << 20) >> 20) as u8;
        let mut combined_offsets = ((num & 16383 << 6) >> 6) as u16;
        let mut offsets :[u4; NUM_ROWS - 1] = [u4::new(0); NUM_ROWS - 1];
        for offset in offsets.iter_mut() {
            *offset = u4::new((combined_offsets % 10) as u8);
            combined_offsets /= 10;
        }
        offsets.reverse();
        // Do a sanity check to make sure the combined_offsets number wasn't too big. If it was, the 
        // code was likely entered incorrectly.
        if combined_offsets != 0 {
            return Err(super::CodeError("The row offsets number appeared to be invalid.".into()));
        }
        // Store all the fields in a ShortCode struct
        let processed_code = ShortCode {
            version: version,
            optotype_definition: optotype_definition,
            start_row: start_row,
            offsets: offsets,
        };
        // Check the CRC is correct
        let message_crc = u6::new((num & 63 << 0) as u8);
        let calculated_crc = processed_code.get_crc();
        if message_crc != calculated_crc {
            return Err(super::CodeError("The code was entered incorrectly (CRC mismatch).".into()));
        }
        Ok(processed_code)
    }
}

impl ShortCode {
    /// This returns a number representing the state stored within a ShortCode struct.
    /// This number is calculated through converting the fields into binary representations
    /// of their values, and then appending them to oneanother via bitshift operations.
    pub fn get_numerical_representation_without_crc(&self) -> u24 {
        // An u32 is used internally to avoid needing to do too many conversions into u24, which make the code less
        // idiomatic and harder to follow. The 8 most significant bytes are removed at the end of the function.
        let version: u32 = (bool::from(self.version) as u32) << 23;
        let optotype_id = (self.optotype_definition.id as u32) << 22;
        let start_row = (self.start_row as u32) << 14;
        // The offsets (from the start row) for each row are the individual digits of a base-10 number. This is unconventional
        // but reduces the number of bits needed to store the data. Storing 4 individual offsets (which range from 0 to 9)
        // requires 4 * 4 bits, i.e. 16 bits. Joining them to give a number (max = 9999) only needs 14 bits to store.
        // This frees up some more bytes in the code for the CRC, at the cost of slightly more difficult parsing.
        let mut combined_offsets: u32 = 0;
        for (i, offset) in self.offsets.iter().enumerate() {
            combined_offsets += u32::from(*offset) * 10_u32.pow((NUM_ROWS - 2 - i) as u32);
        }
        return u24::new(version | optotype_id | start_row | combined_offsets);
    }
    /// This function returns a truncated CRC-8 checksum. For compatibility with
    /// legacy code, this is returned as a u6 but it actually can only take values
    /// from 0 - 32 (i.e a u5). This might be changed in a future version to make 
    /// use of the wasted bit, which is currently set to 0.
    pub fn get_crc(&self) -> u6 {
        let mut crc: u8 = 0;
        for byte in format!("{:0>24b}", self.get_numerical_representation_without_crc()).chars() {
            crc = crc::CRC8_TABLE[((crc ^ byte as u8) & 0xff) as usize] & 0xff;
        }
        // Dividing and rounding is likely a bit slower than doing a bitshift would be,
        // but it maintains backwards compatibility with codes already issued in the older software.
        return u6::new(((crc as f32)/8_f32).round() as u8)
    }
    /// Returns the number representing the state stored within a ShortCode
    /// struct. This number includes a CRC, which is a slightly unconventional
    /// 5 bit length uint. The odd sizes are the result of attempting to fit all
    /// the data into a 6 letter code.
    pub fn get_numerical_representation(&self) -> u30 {
        let data_body = self.get_numerical_representation_without_crc();
        let crc = self.get_crc();
        return u30::new(u32::from(data_body << 6) | u32::from(crc << 0));
    }
    /// Generates a random new shortcode for the specificed optotypes
    pub fn generate_random(optotypes: OptotypeDefinition) -> ShortCode {
        // Generate row offsets
        let mut offsets: [u4; NUM_ROWS-1] = [u4::new(0); NUM_ROWS-1];
        for i in 0..NUM_ROWS-2 {
            loop {
                let new_offset = u4::new(rand::thread_rng().gen::<u8>() % 10);
                if !&offsets.contains(&new_offset) {
                    offsets[i] = new_offset;
                    break;
                }
            }
            
        }
        // Return representation of the shortcode
        ShortCode {
            version: u1::new(0),
            optotype_definition: optotypes,
            start_row: rand::thread_rng().gen(),
            offsets: offsets
        }
    }
}