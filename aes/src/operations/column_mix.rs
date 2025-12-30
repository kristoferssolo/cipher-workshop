use crate::Block128;

/// Mixes each column using matrix multiplication in GF(2^8) ([`MixColumns`] step).
///
/// Each column is treated as a polynomial and multiplied by a fixed polynomial
/// modulo x^4 + 1. This provides diffusion across the rows.
#[must_use]
pub fn mix_columns(block: Block128) -> Block128 {
    let mut bytes = block.to_be_bytes();

    for col in 0..4 {
        let offset = col * 4;
        let [c0, c1, c2, c3] = [
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ];

        // Matrix multiplication over GF(2^8):
        // [d0]   [2 3 1 1] [c0]
        // [d1] = [1 2 3 1] [c1]
        // [d2]   [1 1 2 3] [c2]
        // [d3]   [3 1 1 2] [c3]

        bytes[offset] = MIX_2[c0 as usize] ^ MIX_3[c1 as usize] ^ c2 ^ c3;
        bytes[offset + 1] = c0 ^ MIX_2[c1 as usize] ^ MIX_3[c2 as usize] ^ c3;
        bytes[offset + 2] = c0 ^ c1 ^ MIX_2[c2 as usize] ^ MIX_3[c3 as usize];
        bytes[offset + 3] = MIX_3[c0 as usize] ^ c1 ^ c2 ^ MIX_2[c3 as usize];
    }

    Block128::from_be_bytes(bytes)
}

/// Inverse of [`mix_columns`] using the inverse matrix.
#[must_use]
pub fn inv_mix_columns(block: Block128) -> Block128 {
    let mut bytes = block.to_be_bytes();

    // Process 4 columns independently
    for col in 0..4 {
        let offset = col * 4;
        let [c0, c1, c2, c3] = [
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ];

        // Inverse matrix multiplication:
        // [14  11  13   9]
        // [ 9  14  11  13]
        // [13   9  14  11]
        // [11  13   9  14]

        bytes[offset] =
            MIX_14[c0 as usize] ^ MIX_11[c1 as usize] ^ MIX_13[c2 as usize] ^ MIX_9[c3 as usize];
        bytes[offset + 1] =
            MIX_9[c0 as usize] ^ MIX_14[c1 as usize] ^ MIX_11[c2 as usize] ^ MIX_13[c3 as usize];
        bytes[offset + 2] =
            MIX_13[c0 as usize] ^ MIX_9[c1 as usize] ^ MIX_14[c2 as usize] ^ MIX_11[c3 as usize];
        bytes[offset + 3] =
            MIX_11[c0 as usize] ^ MIX_13[c1 as usize] ^ MIX_9[c2 as usize] ^ MIX_14[c3 as usize];
    }

    Block128::from_be_bytes(bytes)
}

/// Galois Field multiplication by 2 (xtime).
/// If the high bit is set, XOR with the irreducible polynomial 0x1B.
const fn xtime(x: u8) -> u8 {
    if x & 0x80 != 0 {
        return (x << 1) ^ 0x1b;
    }
    x << 1
}

/// General Galois Field multiplication.
/// Implemented using "peasant's algorithm" (shift and add).
const fn gmul(mut a: u8, mut b: u8) -> u8 {
    let mut p = 0;
    let mut i = 0;

    while i < 8 {
        if (b & 1) != 0 {
            p ^= a;
        }
        a = xtime(a);
        b >>= 1;
        i += 1;
    }
    p
}

#[must_use]
#[allow(clippy::cast_possible_truncation)]
const fn make_gmul_table(factor: u8) -> [u8; 256] {
    let mut table = [0; 256];
    let mut i = 0;
    while i < 256 {
        table[i] = gmul(i as u8, factor);
        i += 1;
    }
    table
}

const MIX_2: [u8; 256] = make_gmul_table(2);
const MIX_3: [u8; 256] = make_gmul_table(3);
const MIX_9: [u8; 256] = make_gmul_table(9);
const MIX_11: [u8; 256] = make_gmul_table(11);
const MIX_13: [u8; 256] = make_gmul_table(13);
const MIX_14: [u8; 256] = make_gmul_table(14);

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        0x6353_E08C_0960_E104_CD70_B751_BACA_D0E7,
        0x5F72_6415_57F5_BC92_F7BE_3B29_1DB9_F91A
    )]
    #[case(
        0xD4BF_5D30_D4BF_5D30_D4BF_5D30_D4BF_5D30,
        0x0466_81E5_0466_81E5_0466_81E5_0466_81E5
    )]
    fn columns_mix(#[case] input: u128, #[case] expected: u128) {
        let block = Block128::new(input);
        let result = mix_columns(block).as_u128();
        assert_eq!(
            result, expected,
            "Mix Columns failed. Expected 0x{expected:032X}, got 0x{result:032X}",
        );
    }

    #[rstest]
    #[case(0x63CA_B704_0953_D051_CD60_E0E7_BA70_E18C)]
    #[case(0x6353_E08C_0960_E104_CD70_B751_BACA_D0E7)]
    #[case(0xD4BF_5D30_D4BF_5D30_D4BF_5D30_D4BF_5D30)]
    fn inv_mix_columns_is_inverse(#[case] input: u128) {
        let block = Block128::new(input);
        let mixed = mix_columns(block);
        let unmixed = inv_mix_columns(mixed).as_u128();

        assert_eq!(
            unmixed, input,
            "InvMixColumns(MixColumns(x)) != x. Expected 0x{input:032X}, got 0x{unmixed:032X}",
        );
    }

    #[rstest]
    #[case(0x57, 0x13, 0xFE)] // Example from FIPS-197 4.2.1
    #[case(0x57, 0x01, 0x57)] // Identity
    #[case(0x57, 0x02, 0xAE)] // x2 (xtime)
    #[case(0x57, 0x04, 0x47)] // x4
    #[case(0x57, 0x08, 0x8E)] // x8
    #[case(0x57, 0x10, 0x07)] // x16
    fn galois_multiplication(#[case] a: u8, #[case] b: u8, #[case] expected: u8) {
        let res = gmul(a, b);
        assert_eq!(res, expected, "gmul({a:02x}, {b:02x}) failed");
    }
}
