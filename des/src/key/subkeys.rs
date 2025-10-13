use crate::key::Subkey;
use cipher_core::{CryptoError, CryptoResult, KeyLike};
use std::fmt::Debug;

/// Key Permutation table (64 to 56 bits).
const PC1: [u8; 56] = [
    57, 49, 41, 33, 25, 17, 9, 1, 58, 50, 42, 34, 26, 18, 10, 2, 59, 51, 43, 35, 27, 19, 11, 3, 60,
    52, 44, 36, 63, 55, 47, 39, 31, 23, 15, 7, 62, 54, 46, 38, 30, 22, 14, 6, 61, 53, 45, 37, 29,
    21, 13, 5, 28, 20, 12, 4,
];

/// Compression Permutation table (56 to 48 bits).
const PC2: [u8; 48] = [
    14, 17, 11, 24, 1, 5, 3, 28, 15, 6, 21, 10, 23, 19, 12, 4, 26, 8, 16, 7, 27, 20, 13, 2, 41, 52,
    31, 37, 47, 55, 30, 40, 51, 45, 33, 48, 44, 49, 39, 56, 34, 53, 46, 42, 50, 36, 29, 32,
];

/// Number of Key Bits Shifted per Round
const ROUND_ROTATIONS: [u8; 16] = [1, 1, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 1];

/// Container for all 16 round subkeys; zeroized on drop.
pub struct Subkeys([Subkey; 16]);

impl Subkeys {
    #[inline]
    #[must_use]
    pub const fn new_empty() -> Self {
        Self([const { Subkey::zero() }; 16])
    }

    #[inline]
    #[must_use]
    pub const fn as_array(&self) -> &[Subkey; 16] {
        &self.0
    }

    #[inline]
    pub fn set(&mut self, idx: usize, sk: Subkey) {
        self.0[idx] = sk;
    }

    #[inline]
    #[must_use]
    pub fn get(&self, idx: usize) -> Option<&Subkey> {
        self.0.get(idx)
    }

    pub fn from_key(key: &impl KeyLike) -> CryptoResult<Self> {
        let key_bytes = key.as_bytes();
        let key_len = key_bytes.len();

        if key_len != 8 {
            return Err(CryptoError::invalid_key_size(8, key_len));
        }

        // Produce 56 bits after PC-1 as two 28-bit halves C0 and D0 (MSB-first bit order)
        let (c_bits, d_bits) = PC1.iter().enumerate().fold(
            ([0; 28], [0; 28]),
            |(mut c_bits, mut d_bits), (idx, &pos)| {
                let bit = get_bit_be(key_bytes, pos);
                if idx < 28 {
                    c_bits[idx] = bit;
                } else {
                    d_bits[idx - 28] = bit;
                }
                (c_bits, d_bits)
            },
        );

        let mut c = bits_to_u28(&c_bits);
        let mut d = bits_to_u28(&d_bits);

        let out = ROUND_ROTATIONS.iter().enumerate().fold(
            [const { Subkey::zero() }; 16],
            |mut acc, (round, &shifts)| {
                c = rotate_left_28(c, shifts.into());
                d = rotate_left_28(d, shifts.into());
                let sub48 = pc2_from_cd(c, d);
                acc[round] = Subkey::from(sub48);
                acc
            },
        );

        Ok(Self(out))
    }

    pub(crate) fn as_u64_array(&self) -> [u64; 16] {
        self.0
            .iter()
            .enumerate()
            .fold([0; 16], |mut out, (idx, sk)| {
                out[idx] = sk.as_int();
                out
            })
    }
}

fn pc2_from_cd(c: u32, d: u32) -> u64 {
    let combined: [u8; 56] = (0..28)
        .flat_map(|idx| {
            let bit_idx = 27 - idx;
            [((c >> bit_idx) & 1) as u8, ((d >> bit_idx) & 1) as u8]
        })
        .collect::<Vec<_>>()
        .try_into()
        .expect("");

    let out = PC2.iter().fold(0, |acc, &pos| {
        let bit = u64::from(combined[(pos as usize).saturating_sub(1)]);
        (acc << 1) | bit
    });

    out & 0xFFFF_FFFF_FFFF
}

const U28_MASK: u32 = 0x0FFF_FFFF;

#[inline]
#[must_use]
const fn get_bit_be(bytes: &[u8], pos: u8) -> u8 {
    let p = (pos as usize).saturating_sub(1);
    let byte_idx = p / 8;
    let bit_idx = 7 - (p % 8);
    (bytes[byte_idx] >> bit_idx) & 1
}

#[inline]
#[must_use]
fn bits_to_u28(bits: &[u8; 28]) -> u32 {
    let mut v = 0;
    for &b in bits {
        v = (v << 1) | (u32::from(b));
    }
    v & U28_MASK
}

#[inline]
#[must_use]
const fn rotate_left_28(v: u32, n: u32) -> u32 {
    let v = v & U28_MASK;
    ((v << n) | (v >> (28 - n))) & U28_MASK
}

impl Debug for Subkeys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Subkeys[REDACTED]")
    }
}

impl Default for Subkeys {
    fn default() -> Self {
        Self::new_empty()
    }
}
