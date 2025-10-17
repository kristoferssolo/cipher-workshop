use crate::{
    constants::{PC1, PC2, ROUND_ROTATIONS},
    key::{Key, cd56::CD56, key56::Key56, subkey::Subkey},
    utils::permutate,
};
use cipher_core::CipherResult;
use std::{
    fmt::Debug,
    iter::Rev,
    ops::Index,
    slice::{Iter, IterMut},
};

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
    #[must_use]
    pub fn get(&self, idx: usize) -> Option<&Subkey> {
        self.0.get(idx)
    }

    /// # Errors
    /// # Panics
    pub fn from_key(key: &Key) -> CipherResult<Self> {
        let mut cd56 = pc1(key).split(); // 56-bit: C0 + D0

        let subkeys = ROUND_ROTATIONS
            .iter()
            .map(|&shift_amount| {
                cd56.rotate_left(shift_amount);
                pc2(&cd56)
            })
            .collect::<Vec<Subkey>>()
            .try_into()
            .expect("Exactly 16 subkeys expected");

        Ok(Self(subkeys))
    }

    /// Borrowing forward iterator.
    pub fn iter(&self) -> Iter<'_, Subkey> {
        self.0.iter()
    }

    /// Borrowing reverse iterator.
    pub fn iter_rev(&self) -> Rev<Iter<'_, Subkey>> {
        self.0.iter().rev()
    }

    /// Mutable iterator if you need it.
    pub fn iter_mut(&mut self) -> IterMut<'_, Subkey> {
        self.0.iter_mut()
    }

    /// Consume `self` and return a new `Subkeys` with reversed order.
    #[must_use]
    pub const fn reversed(mut self) -> Self {
        self.0.reverse();
        self
    }
}

#[inline]
#[must_use]
fn pc1(key: &Key) -> Key56 {
    permutate(key.as_u64(), 64, 56, &PC1).into()
}

#[inline]
#[must_use]
fn pc2(cd: &CD56) -> Subkey {
    let key56 = Key56::from(cd);
    permutate(key56.as_int(), 56, 48, &PC2).into()
}

impl<'a> IntoIterator for &'a Subkeys {
    type Item = &'a Subkey;
    type IntoIter = Iter<'a, Subkey>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a> IntoIterator for &'a mut Subkeys {
    type Item = &'a mut Subkey;
    type IntoIter = IterMut<'a, Subkey>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl Index<usize> for Subkeys {
    type Output = Subkey;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
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

    #[rstest]
    #[case(TEST_KEY, 0x00F0_CCAA_F556_678F)]
    fn pc1_permutaion_correct(#[case] key: u64, #[case] expected: u64) {
        let result = pc1(&key.into()).as_int();
        assert_eq!(
            result, expected,
            "PC1 permutation failed. Expected {expected:08X}, got {result:08X}",
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
    fn pc2_permutaion(#[case] before: u64, #[case] expected: u64) {
        let key56 = Key56::from(before).split();
        let result = pc2(&key56).as_int();
        assert_eq!(
            result, expected,
            "PC2 permutation failed. Expected {expected:016X}, got {result:016X}"
        );
    }
}
