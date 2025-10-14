use crate::{
    key::{key56::Key56, subkey::Subkey},
    utils::permutate,
};
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

        let key_arr = key_bytes
            .try_into()
            .map_err(|_| CryptoError::invalid_key_size(8, key_len))?;

        let key_be = u64::from_be_bytes(key_arr);

        let cd_56 = pc1(key_be); // 56-bit: C0 + D0
        let (c, d) = cd_56.split();

        let subkeys = ROUND_ROTATIONS
            .iter()
            .map(|&shift_amount| {
                let cn = c.rotate_left(shift_amount); // C_(n-1) -> C_n
                let dn = d.rotate_left(shift_amount); // D_(n-1) -> D_n
                let combined = [cn, dn].into();
                pc2(&combined)
            })
            .collect::<Vec<Subkey>>()
            .try_into()
            .expect("Exactly 16 subkeys expected");

        Ok(Self(subkeys))
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

fn pc1(key: u64) -> Key56 {
    permutate(key, 64, 56, &PC1).into()
}

fn pc2(key: &Key56) -> Subkey {
    permutate(key.as_int(), 56, 48, &PC2).into()
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

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const TEST_KEY: u64 = 0x1334_5779_9BBC_DFF1;
    const TEST_PC1_RESULT: u64 = 0x00F0_CCAA_F556_678F;

    #[rstest]
    #[case(TEST_KEY, TEST_PC1_RESULT)]
    fn pc1_permutaion_correct(#[case] key: u64, #[case] expected: u64) {
        let result = pc1(key);

        assert_eq!(
            result.as_int(),
            expected,
            "PC1 permutation failed. Expected {expected:08X}, got {:08X}",
            result.as_int()
        );
    }

    #[rstest]
    #[case(0x00F0_CCAA_F556_678F, 0xCB3D_8B0E_17F5)] // K_0
    #[case(0x00E1_9955_FAAC_CF1E, 0x1B02_EFFC_7072)] // K_1
    #[case(0x00C3_32AB_F559_9E3D, 0x79AE_D9DB_C9E5)] // K_2
    #[case(0x000C_CAAF_F566_78F5, 0x55FC_8A42_CF99)] // K_3
    #[case(0x0033_2ABF_C599_E3D5, 0x72AD_D6DB_351D)] // K_4
    #[case(0x00CC_AAFF_0667_8F55, 0x7CEC_07EB_53A8)] // K_5
    #[case(0x0032_ABFC_399E_3D55, 0x63A5_3E50_7B2F)] // K_6
    #[case(0x00CA_AFF0_C678_F556, 0xEC84_B7F6_18BC)] // K_7
    #[case(0x002A_BFC3_39E3_D559, 0xF78A_3AC1_3BFB)] // K_8
    #[case(0x0055_7F86_63C7_AAB3, 0xE0DB_EBED_E781)] // K_9
    #[case(0x0055_FE19_9F1E_AACC, 0xB1F3_47BA_464F)] // K_10
    #[case(0x0057_F866_5C7A_AB33, 0x215F_D3DE_D386)] // K_11
    #[case(0x005F_E199_51EA_ACCF, 0x7571_F594_67E9)] // K_12
    #[case(0x007F_8665_57AA_B33C, 0x97C5_D1FA_BA41)] // K_13
    #[case(0x00FE_1995_5EAA_CCF1, 0x5F43_B7F2_E73A)] // K_14
    #[case(0x00F8_6655_7AAB_33C7, 0xBF91_8D3D_3F0A)] // K_15
    #[case(0x00F0_CCAA_F556_678F, 0xCB3D_8B0E_17F5)] // K_16
    fn pc2_permutaion(#[case] before: u64, #[case] after: u64) {
        let result = pc2(&before.into());
        assert_eq!(result.as_int(), after, "PC2 permutation failed");
    }
}
