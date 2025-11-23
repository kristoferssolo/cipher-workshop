use crate::constants::{INV_S_BOXES, S_BOXES};

pub trait SboxLookup: Sized {
    fn sbox_lookup(self) -> Self;
    fn inv_sbox_lookup(self) -> Self;
}

macro_rules! impl_sbox_lookup {
    ($ty:ty, $bytes:expr) => {
        impl SboxLookup for $ty {
            fn sbox_lookup(self) -> Self {
                (0..$bytes).fold(0, |acc, idx| {
                    let shift = ($bytes - 1 - idx) * 8;
                    let byte = ((self >> shift) & 0xFF) as u8;
                    let row = (byte >> 4) as usize;
                    let col = (byte & 0xF) as usize;
                    acc | Self::from(S_BOXES[row][col]) << shift
                })
            }

            fn inv_sbox_lookup(self) -> Self {
                (0..$bytes).fold(0, |acc, idx| {
                    let shift = ($bytes - 1 - idx) * 8;
                    let byte = ((self >> shift) & 0xFF) as u8;
                    let row = (byte >> 4) as usize;
                    let col = (byte & 0xF) as usize;
                    acc | Self::from(INV_S_BOXES[row][col]) << shift
                })
            }
        }
    };
}

impl_sbox_lookup!(u8, 1);
impl_sbox_lookup!(u16, 2);
impl_sbox_lookup!(u32, 4);
impl_sbox_lookup!(u64, 8);
impl_sbox_lookup!(u128, 16);
