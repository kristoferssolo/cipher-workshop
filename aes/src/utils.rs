/// Generic bit permutation for DES specification.
/// DES uses 1-based positioning where bit 1 is the leftmost (MSB) and bit 64 is the rightmost (LSB).
/// This function handles this correctly for arbitrary input/output sizes.
///
/// # Arguments
/// - `input` - The input value (treated as a bitfield of `input_bits` size)
/// - `input_bits` - Number of meaningful bits in the input (1-64)
/// - `output_bits` - Number of bits in the output (1-64)
/// - `position_table` - Table of 1-based positions (1 to `input_bits`) where each output bit comes from.
///   The table should have `output_bits` entries. For each entry i (0-based), `position_table`[i] indicates
///   which input bit (1-based) should go to the i-th output bit position (from MSB to LSB).
#[must_use]
pub fn permutate(
    input: u64,
    input_bit_amount: u64,
    output_bit_amount: u64,
    position_table: &[u8],
) -> u64 {
    position_table
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, &input_pos_1based)| {
            let input_bit_pos = input_bit_amount - u64::from(input_pos_1based);
            let bit_value = (input >> input_bit_pos) & 1;
            let output_bit_pos = output_bit_amount - 1 - (idx as u64);
            acc | (bit_value << output_bit_pos)
        })
}
