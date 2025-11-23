use crate::{
    block::{Block32, secret_block},
    sbox::SboxLookup,
};
use cipher_core::{BlockError, InputBlock};
use std::{
    slice::{ChunksExact, from_raw_parts, from_raw_parts_mut},
    str::FromStr,
};

secret_block! {
    pub struct Block128(u128, 128, 0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF);
}

impl InputBlock for Block128 {
    const BLOCK_SIZE: usize = 128;
    fn as_bytes(&self) -> &[u8] {
        unsafe { from_raw_parts((&raw const self.0).cast::<u128>().cast::<u8>(), 16) }
    }
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { from_raw_parts_mut((&raw mut self.0).cast::<u128>().cast::<u8>(), 16) }
    }
}

impl SboxLookup for Block128 {
    fn sbox_lookup(self) -> Self {
        Self(self.0.sbox_lookup())
    }
    fn inv_sbox_lookup(self) -> Self {
        Self(self.0.inv_sbox_lookup())
    }
}

impl Block128 {
    #[inline]
    #[must_use]
    pub const fn from_be_bytes(bytes: [u8; 16]) -> Self {
        Self(u128::from_be_bytes(bytes))
    }

    #[inline]
    #[must_use]
    pub const fn to_be_bytes(self) -> [u8; 16] {
        self.0.to_be_bytes()
    }

    #[inline]
    #[must_use]
    pub const fn to_le_bytes(self) -> [u8; 16] {
        self.0.to_le_bytes()
    }

    #[inline]
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub const fn as_block32_array(self) -> [Block32; 4] {
        let val = self.0;
        [
            Block32::from_u32((val >> 96) as u32),
            Block32::from_u32((val >> 64) as u32),
            Block32::from_u32((val >> 32) as u32),
            Block32::from_u32(val as u32),
        ]
    }

    #[must_use]
    pub const fn shift_rows(self) -> Self {
        let b = self.to_be_bytes();
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

        Self::from_be_bytes(out)
    }

    #[must_use]
    pub fn mix_columns(self) -> Self {
        let mut bytes = self.to_be_bytes();

        for col in 0..4 {
            let offset = col * 4;

            let c0 = bytes[offset];
            let c1 = bytes[offset + 1];
            let c2 = bytes[offset + 2];
            let c3 = bytes[offset + 3];

            // Matrix multiplication over GF(2^8):
            // [d0]   [2 3 1 1] [c0]
            // [d1] = [1 2 3 1] [c1]
            // [d2]   [1 1 2 3] [c2]
            // [d3]   [3 1 1 2] [c3]

            bytes[offset] = gmul(c0, 2) ^ gmul(c1, 3) ^ c2 ^ c3;
            bytes[offset + 1] = c0 ^ gmul(c1, 2) ^ gmul(c2, 3) ^ c3;
            bytes[offset + 2] = c0 ^ c1 ^ gmul(c2, 2) ^ gmul(c3, 3);
            bytes[offset + 3] = gmul(c0, 3) ^ c1 ^ c2 ^ gmul(c3, 2);
        }

        Self::from_be_bytes(bytes)
    }

    #[must_use]
    pub const fn inv_shift_rows(self) -> Self {
        let b = self.to_be_bytes();
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

        Self::from_be_bytes(out)
    }

    #[must_use]
    pub fn inv_mix_columns(self) -> Self {
        let mut bytes = self.to_be_bytes();

        // Process 4 columns independently
        for col in 0..4 {
            let offset = col * 4;
            let c0 = bytes[offset];
            let c1 = bytes[offset + 1];
            let c2 = bytes[offset + 2];
            let c3 = bytes[offset + 3];

            // Inverse matrix multiplication:
            // [14  11  13   9]
            // [ 9  14  11  13]
            // [13   9  14  11]
            // [11  13   9  14]

            bytes[offset] = gmul(c0, 14) ^ gmul(c1, 11) ^ gmul(c2, 13) ^ gmul(c3, 9);
            bytes[offset + 1] = gmul(c0, 9) ^ gmul(c1, 14) ^ gmul(c2, 11) ^ gmul(c3, 13);
            bytes[offset + 2] = gmul(c0, 13) ^ gmul(c1, 9) ^ gmul(c2, 14) ^ gmul(c3, 11);
            bytes[offset + 3] = gmul(c0, 11) ^ gmul(c1, 13) ^ gmul(c2, 9) ^ gmul(c3, 14);
        }

        Self::from_be_bytes(bytes)
    }

    #[inline]
    #[must_use]
    pub fn sub_bytes(self) -> Self {
        Self(self.0.sbox_lookup())
    }

    #[inline]
    #[must_use]
    pub fn inv_sub_bytes(self) -> Self {
        Self(self.0.inv_sbox_lookup())
    }
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

    // Unrolled loop for const context
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

impl FromStr for Block128 {
    type Err = BlockError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(parse_string_to_u128(s)?))
    }
}

fn parse_string_to_u128(s: &str) -> Result<u128, BlockError> {
    let trimmed = s.trim();

    if trimmed.is_empty() {
        return Err(BlockError::EmptyBlock);
    }

    // Hexadecimal with 0x/0X prefix
    if let Some(hex_str) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        return parse_radix(hex_str, 16);
    }
    // Binary with 0b/0B prefix
    if let Some(bin_str) = trimmed
        .strip_prefix("0b")
        .or_else(|| trimmed.strip_prefix("0B"))
    {
        return parse_radix(bin_str, 2);
    }

    ascii_string_to_u128(trimmed)
}

fn parse_radix(s: &str, radix: u32) -> Result<u128, BlockError> {
    let trimmed = s.trim_start_matches('0');
    if trimmed.is_empty() {
        return Ok(0);
    }

    u128::from_str_radix(trimmed, radix).map_err(BlockError::from)
}

fn ascii_string_to_u128(s: &str) -> Result<u128, BlockError> {
    if s.len() > 8 {
        return Err(BlockError::InvalidByteStringLength(s.len()));
    }

    if !s.is_ascii() {
        return Err(BlockError::conversion_error(
            "u64",
            "String contains non-ASCII characters",
        ));
    }

    let mut bytes = [0u8; 16];
    let offset = 16 - s.len();
    bytes[offset..].copy_from_slice(s.as_bytes());

    Ok(u128::from_be_bytes(bytes))
}

impl From<[u8; 16]> for Block128 {
    fn from(bytes: [u8; 16]) -> Self {
        Self::from_be_bytes(bytes)
    }
}

impl From<Block128> for [Block32; 4] {
    fn from(block: Block128) -> Self {
        block.as_block32_array()
    }
}

impl From<Block128> for Vec<u8> {
    fn from(value: Block128) -> Self {
        value.to_be_bytes().to_vec()
    }
}

impl From<&Block128> for Vec<u8> {
    fn from(value: &Block128) -> Self {
        value.to_be_bytes().to_vec()
    }
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
    fn shift_rows(#[case] input: u128, #[case] expected: u128) {
        let block = Block128::new(input);
        let result = block.shift_rows().as_u128();
        assert_eq!(
            result, expected,
            "Shift Rows failed. Expected 0x{expected:032X}, got 0x{result:032X}",
        );
    }

    #[rstest]
    #[case(
        0x6353_E08C_0960_E104_CD70_B751_BACA_D0E7,
        0x5F72_6415_57F5_BC92_F7BE_3B29_1DB9_F91A
    )]
    #[case(
        0xD4BF_5D30_D4BF_5D30_D4BF_5D30_D4BF_5D30,
        0x0466_81E5_0466_81E5_0466_81E5_0466_81E5
    )]
    fn mix_columns(#[case] input: u128, #[case] expected: u128) {
        let block = Block128::new(input);
        let result = block.mix_columns().as_u128();
        assert_eq!(
            result, expected,
            "Mix Columns failed. Expected 0x{expected:032X}, got 0x{result:032X}",
        );
    }

    #[rstest]
    #[case(0x63CA_B704_0953_D051_CD60_E0E7_BA70_E18C)]
    #[case(0x6353_E08C_0960_E104_CD70_B751_BACA_D0E7)]
    #[case(0xD4BF_5D30_D4BF_5D30_D4BF_5D30_D4BF_5D30)]
    fn inv_shift_rows_is_inverse(#[case] input: u128) {
        let block = Block128::new(input);
        let shifted = block.shift_rows();
        let unshifted = shifted.inv_shift_rows().as_u128();

        assert_eq!(
            unshifted, input,
            "InvShiftRows(ShiftRows(x)) != x. Expected 0x{input:032X}, got 0x{unshifted:032X}",
        );
    }

    #[rstest]
    #[case(0x63CA_B704_0953_D051_CD60_E0E7_BA70_E18C)]
    #[case(0x6353_E08C_0960_E104_CD70_B751_BACA_D0E7)]
    #[case(0xD4BF_5D30_D4BF_5D30_D4BF_5D30_D4BF_5D30)]
    fn inv_mix_columns_is_inverse(#[case] input: u128) {
        let block = Block128::new(input);
        let mixed = block.mix_columns();
        let unmixed = mixed.inv_mix_columns().as_u128();

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
