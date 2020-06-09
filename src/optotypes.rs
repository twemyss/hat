use crate::codes::{NUM_ROWS, NUM_OPTOTYPES_ON_ROW, short::ShortCode};

/// Stores the name, numeric ID, and possible optotypes for a particular
/// group of optotypes.
/// 
/// The data in these groups are defined in a From<u32> trait (where the u32 is the ID)
/// and adding new optotypes requires recompiling the software at present. This is not
/// necessarily ideal, but the addition of new optotypes is expected to be extremely
/// rare. Future versions may shift optotype definitions into a configuration file and
/// parse that instead.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OptotypeDefinition {
    pub name: String,
    pub id: u8,
    pub optotypes: Vec<char>,
}

/// Obtains optotype definition from the numeric ID of that parameter
/// 
/// This function defines the Sloan optotypes as a default set of optotypes
/// for IDs which are not defined, because they are likely to be the most commonly
/// used form of optotypes. The (likely equally valid) alternative is to panic when
/// an unknown ID is given.
///
/// This is where new optotype definitions can be added. Please note that
/// these optotype definitions must also be matched by a new CSS class in 
/// `src/templates/answers.html.tera` defining a font-family with the same
/// name as the optotype definition has.
impl From<u32> for OptotypeDefinition {
    fn from(id: u32) -> Self {
        match id {
            0 => OptotypeDefinition { name: "aukland".to_string(), id: 0, optotypes:  vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']},
            1 => OptotypeDefinition { name: "sloan".to_string(), id: 1, optotypes:  vec!['C', 'D', 'H', 'K', 'N', 'O', 'R', 'S', 'V', 'Z']},
            _ => OptotypeDefinition::from(1)
        }
    }
}

/// Defines the ID of the default optotypes (used if a code specifies an optotype ID which
/// is not specified in the implementation above).
pub const DEFAULT_OPTOTYPES: u32 = 1;

/// An OptotypeRow defines a given row of the chart - both the optotype char's contained
/// within that row, and the font/border size.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OptotypeRow {
    pub text_size: f64,
    pub border_size: f64,
    pub optotypes: Vec<u8>
}

/// The OptotypeArrangement defines a given chart uniquely. It contains fields for each
/// row, a string representation of the code used to generate that chart, and the
/// definition of the optotypes used for that chart. This is enough information to
/// uniquely regenerate a chart.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct OptotypeArrangement {
    pub rows: Vec<OptotypeRow>,
    pub code: String,
    pub optotype_definition: OptotypeDefinition
}

/// Convert a ShortCode into an optotype arrangement for display
impl From<ShortCode> for OptotypeArrangement {
    fn from(short_code: ShortCode) -> Self {
        let code = short_code.to_string();
        let optotype_definition = short_code.optotype_definition;
        // Generate each row of the chart in turn
        let mut rows: Vec<OptotypeRow> = Vec::new();
        for row in (0..NUM_ROWS).rev() {
            // Get the indices of the optotypes on each row. These are indicies within the list of optotype characters given by the 
            // optotype definition which is being used for the Shortcode.
            let mut optotypes: Vec<u8> = Vec::new();
            if row == NUM_ROWS-1 {
                // The last row is calculated slightly differently to the others
                // We simply loop over all possible combinations (without repeated optotypes)
                // and then choose the nth one, where n is the start_row number.
                // There might be a better way to do this, so this is an area to investigate if
                // performance starts to be a concern.
                
                // We need to define a scale factor because there are far more valid optotype combinations generated in this
                // routine than there are possible values for the u8 start_row value. To make sure the u8 start_row is sampling over
                // the entire range of possible combinations, we multiply it by a scale such that the max start_row is now
                // approximately corresponding to the iteration of the loop which the last valid optotype arrangement calculated by this function occurs.
                let radix = optotype_definition.optotypes.len() as u32;
                let scale = ((f64::from(factorial(radix))/f64::from(factorial(radix-NUM_OPTOTYPES_ON_ROW[row]))) / f64::from(u8::MAX)).round() as u32;
                // Once scale is calculated, iteration through the permissible combinations can begin
                let mut valid_combination_iteration: u32 = 0;
                for i in 0..(radix.pow(NUM_OPTOTYPES_ON_ROW[row]) - 1) {
                    // Get each digit of the number, when encoded in base of the number of optotypes
                    let mut optotypes_on_potential_row: Vec<u8> = Vec::new();
                    // Now extract a list containing the position of each optotype character in the character array which defines that set of optotypes
                    // This is essentially converting into an arbirary base (base = the number of optotype characters in that set of optotypes)
                    let mut temp_for_conversion = i as u32;
                    for _ in 0..NUM_OPTOTYPES_ON_ROW[row] {
                        if temp_for_conversion == 0 {
                            optotypes_on_potential_row.push(0_u8);
                        } else {
                            optotypes_on_potential_row.push((temp_for_conversion % radix) as u8);
                            temp_for_conversion /= radix;
                        }
                    }
                    // Check there were no repeats of the same index/optotype on the row
                    let mut copy_of_optotypes = optotypes_on_potential_row.clone();
                    copy_of_optotypes.sort();
                    copy_of_optotypes.dedup();
                    if copy_of_optotypes.len() == (NUM_OPTOTYPES_ON_ROW[row] as usize) {
                        if valid_combination_iteration == u32::from(short_code.start_row)*scale {
                            // Reverse the row (for legacy compatibility - there's no particular reason that the row needs to be reversed,
                            // but the implemented which was investigated in the clinic calculated the row differently, and this reversal is needed
                            // to keep the codes from that edition working on this version of the codebase).
                            optotypes_on_potential_row.reverse();
                            optotypes = optotypes_on_potential_row;
                            break;
                        }
                        valid_combination_iteration += 1;
                    }
                } 
            } else {
                // All the other rows are calculated from the offset value for the current row and the reference row. The reference row is the 
                // bottom row of the chart, which has already been calculated by this point.
                let reference_row = &rows[0];
                let current_offset = u8::from(short_code.offsets[NUM_ROWS - row - 2]);
                // Calculate each optotype on the row by adding the offset number to the reference row optotype, then wrapping based on the number
                // of optotypes, so the calculated index of the new optotype is less than the size of the array of potential optotypes.
                for j in 0..NUM_OPTOTYPES_ON_ROW[row] {
                        optotypes.push((reference_row.optotypes[j as usize] + current_offset) % (optotype_definition.optotypes.len() as u8));
                }
            }
            // Insert the row

            let row_text_size = (NUM_ROWS - row) as f64;
            rows.push(OptotypeRow {
                text_size: row_text_size,
                border_size: row_text_size/5.0,
                optotypes: optotypes
            });
        };
        // Rows is calculated in reverse order (bottom first, top last) but for PDF generation, it makes sense to reverse them
        // and store them from the top to bottom of the page.
        rows.reverse();
        // Return the generated arrangement
        OptotypeArrangement {
            optotype_definition: optotype_definition,
            code: code,
            rows: rows
        }
    }
}

// This is just a helper function for use in calculating the number of potential optotype combinations that exist on a given row.
// It's a simple implementation of the factorial function
fn factorial(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => factorial(n - 1) * n,
    }
}