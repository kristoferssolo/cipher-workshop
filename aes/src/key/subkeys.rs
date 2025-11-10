use crate::{
    constants::{RCON, S_BOXES},
    key::{Key, expanded::ExpandedKey, subkey::Subkey},
};
use std::{
    fmt::Debug,
    iter::Rev,
    ops::Index,
    slice::{Iter, IterMut},
};

const SUBKEY_COUNT: usize = 44;

// #[derive(Default)]
pub struct Subkeys([Subkey; SUBKEY_COUNT]);

impl Subkeys {
    /// Generates 44 round subkeys from the given key.
    #[must_use]
    pub fn from_key(key: &Key) -> Self {
        let mut subkeys = [const { Subkey::zero() }; 44];

        // Load initial key
        for (idx, &key) in key.as_2d().iter().enumerate() {
            subkeys[idx] = Subkey::from_u32(u32::from_be_bytes(key));
        }

        for (round, &rcon) in RCON.iter().enumerate() {
            let idx = round * 4 + 4;

            subkeys[idx] = subkeys[idx - 4] ^ expand(subkeys[idx - 1], rcon);
            subkeys[idx + 1] = subkeys[idx] ^ subkeys[idx - 3];
            subkeys[idx + 2] = subkeys[idx + 1] ^ subkeys[idx - 2];
            subkeys[idx + 3] = subkeys[idx + 2] ^ subkeys[idx - 1];
        }

        Self(subkeys)
    }

    /// Returns an iterator over the subkeys.
    pub fn iter(&self) -> Iter<'_, Subkey> {
        self.0.iter()
    }

    /// Returns a reverse iterator over the subkeys.
    pub fn iter_rev(&self) -> Rev<Iter<'_, Subkey>> {
        self.0.iter().rev()
    }

    /// Returns a mutable iterator over the subkeys.
    pub fn iter_mut(&mut self) -> IterMut<'_, Subkey> {
        self.0.iter_mut()
    }
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

const fn expand(subkey: Subkey, rcon: u32) -> ExpandedKey {
    let word = subkey.rotate_left(8).as_u32();

    let b0 = sbox_lookup(word >> 24);
    let b1 = sbox_lookup(word >> 16);
    let b2 = sbox_lookup(word >> 8);
    let b3 = sbox_lookup(word);
    let substituted = (b0 << 24) | (b1 << 16) | (b2 << 8) | b3;
    ExpandedKey::from_u32(substituted ^ rcon)
}

const fn sbox_lookup(byte: u32) -> u32 {
    const MASK: u32 = 0xFF;
    let row = ((byte & MASK) as usize) >> 4;
    let col = ((byte & MASK) as usize) & 0xF;

    S_BOXES[row][col] as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_KEY: u128 = 0x0F15_71C9_47D9_E859_1CB7_ADD6_AF7F_6798;

    impl PartialEq<[[u8; 4]; 44]> for Subkeys {
        fn eq(&self, other: &[[u8; 4]; 44]) -> bool {
            self.iter()
                .zip(other)
                .all(|(a, &b)| a.as_u32() == u32::from_be_bytes(b))
        }
    }

    #[test]
    fn from_key() {
        let key = Key::from(TEST_KEY);
        let subkeys = Subkeys::from_key(&key);
        assert_eq!(
            subkeys,
            [
                [0x0F, 0x15, 0x71, 0xC9],
                [0x47, 0xD9, 0xE8, 0x59],
                [0x1C, 0xB7, 0xAD, 0xD6],
                [0xAF, 0x7F, 0x67, 0x98],
                [0xDC, 0x90, 0x37, 0xB0],
                [0x9B, 0x49, 0xDF, 0xE9],
                [0x87, 0xFE, 0x72, 0x3F],
                [0x28, 0x81, 0x15, 0xA7],
                [0xD2, 0xC9, 0x6B, 0x84],
                [0x49, 0x80, 0xB4, 0x6D],
                [0xCE, 0x7E, 0xC6, 0x52],
                [0xE6, 0xFF, 0xD3, 0xF5],
                [0xC0, 0xAF, 0x8D, 0x0A],
                [0x89, 0x2F, 0x39, 0x67],
                [0x47, 0x51, 0xFF, 0x35],
                [0xA1, 0xAE, 0x2C, 0xC0],
                [0x2C, 0xDE, 0x37, 0x38],
                [0xA5, 0xF1, 0x0E, 0x5F],
                [0xE2, 0xA0, 0xF1, 0x6A],
                [0x43, 0x0E, 0xDD, 0xAA],
                [0x97, 0x1F, 0x9B, 0x22],
                [0x32, 0xEE, 0x95, 0x7D],
                [0xD0, 0x4E, 0x64, 0x17],
                [0x93, 0x40, 0xB9, 0xBD],
                [0xBE, 0x49, 0xE1, 0xFE],
                [0x8C, 0xA7, 0x74, 0x83],
                [0x5C, 0xE9, 0x10, 0x94],
                [0xCF, 0xA9, 0xA9, 0x29],
                [0x2D, 0x9A, 0x44, 0x74],
                [0xA1, 0x3D, 0x30, 0xF7],
                [0xFD, 0xD4, 0x20, 0x63],
                [0x32, 0x7D, 0x89, 0x4A],
                [0x52, 0x3D, 0x92, 0x57],
                [0xF3, 0x00, 0xA2, 0xA0],
                [0x0E, 0xD4, 0x82, 0xC3],
                [0x3C, 0xA9, 0x0B, 0x89],
                [0x9A, 0x16, 0x35, 0xBC],
                [0x69, 0x16, 0x97, 0x1C],
                [0x67, 0xC2, 0x15, 0xDF],
                [0x5B, 0x6B, 0x1E, 0x56],
                [0xD3, 0x64, 0x84, 0x85],
                [0xBA, 0x72, 0x13, 0x99],
                [0xDD, 0xB0, 0x06, 0x46],
                [0x86, 0xDB, 0x18, 0x10],
            ]
        );
    }
}
