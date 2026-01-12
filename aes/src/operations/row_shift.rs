use crate::Block128;

/// Cyclically shifts rows of the state matrix (`ShiftRows` step).
///
/// Row 0: no shift, Row 1: shift left 1, Row 2: shift left 2, Row 3: shift left 3.
/// This provides diffusion across the columns.
#[must_use]
pub const fn shift_rows(block: Block128) -> Block128 {
    let b = block.to_be_bytes();
    let mut out = [0u8; 16];

    // Row 0: No shift (Indices 0, 4, 8, 12)
    out[0] = b[0];
    out[4] = b[4];
    out[8] = b[8];
    out[12] = b[12];

    // Row 1: Shift left 1 (Indices 1, 5, 9, 13 -> 5, 9, 13, 1)
    out[1] = b[5];
    out[5] = b[9];
    out[9] = b[13];
    out[13] = b[1];

    // Row 2: Shift left 2 (Indices 2, 6, 10, 14 -> 10, 14, 2, 6)
    out[2] = b[10];
    out[6] = b[14];
    out[10] = b[2];
    out[14] = b[6];

    // Row 3: Shift left 3 (Indices 3, 7, 11, 15 -> 15, 3, 7, 11)
    out[3] = b[15];
    out[7] = b[3];
    out[11] = b[7];
    out[15] = b[11];

    Block128::from_be_bytes(out)
}

/// Inverse of [`shift_rows`] - shifts rows right instead of left.
#[must_use]
pub const fn inv_shift_rows(block: Block128) -> Block128 {
    let b = block.to_be_bytes();
    let mut out = [0u8; 16];

    // Row 0 (Indices 0, 4, 8, 12): No shift
    out[0] = b[0];
    out[4] = b[4];
    out[8] = b[8];
    out[12] = b[12];

    // Row 1 (Indices 1, 5, 9, 13): Shift right 1 -> (13, 1, 5, 9)
    out[1] = b[13];
    out[5] = b[1];
    out[9] = b[5];
    out[13] = b[9];

    // Row 2 (Indices 2, 6, 10, 14): Shift right 2 -> (10, 14, 2, 6)
    out[2] = b[10];
    out[6] = b[14];
    out[10] = b[2];
    out[14] = b[6];

    // Row 3 (Indices 3, 7, 11, 15): Shift right 3 -> (7, 11, 15, 3)
    out[3] = b[7];
    out[7] = b[11];
    out[11] = b[15];
    out[15] = b[3];

    Block128::from_be_bytes(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        0x63CA_B704_0953_D051_CD60_E0E7_BA70_E18C,
        0x6353_E08C_0960_E104_CD70_B751_BACA_D0E7
    )]
    fn row_shift(#[case] input: u128, #[case] expected: u128) {
        let block = Block128::new(input);
        let result = shift_rows(block).as_u128();
        assert_eq!(
            result, expected,
            "Shift Rows failed. Expected 0x{expected:032X}, got 0x{result:032X}",
        );
    }

    #[rstest]
    #[case(0x63CA_B704_0953_D051_CD60_E0E7_BA70_E18C)]
    #[case(0x6353_E08C_0960_E104_CD70_B751_BACA_D0E7)]
    #[case(0xD4BF_5D30_D4BF_5D30_D4BF_5D30_D4BF_5D30)]
    fn inv_shift_rows_is_inverse(#[case] input: u128) {
        let block = Block128::new(input);
        let shifted = shift_rows(block);
        let unshifted = inv_shift_rows(shifted).as_u128();

        assert_eq!(
            unshifted, input,
            "InvShiftRows(ShiftRows(x)) != x. Expected 0x{input:032X}, got 0x{unshifted:032X}",
        );
    }
}
