use crate::constants::{INV_S_BOXES, S_BOXES};

pub trait SboxLookup: Sized {
    fn sbox_lookup(self) -> Self;
    fn inv_sbox_lookup(self) -> Self;
}

macro_rules! impl_sbox_lookup {
    ($($ty:ty),*) => {
        $(
            impl SboxLookup for $ty {
                fn sbox_lookup(self) -> Self {
                    let mut bytes = self.to_le_bytes();
                    for b in bytes.iter_mut() {
                        let row = (*b >> 4) as usize;
                        let col = (*b & 0x0F) as usize;
                        *b = S_BOXES[row][col];
                    }
                    Self::from_le_bytes(bytes)
                }

                fn inv_sbox_lookup(self) -> Self {
                    let mut bytes = self.to_le_bytes();
                    for b in bytes.iter_mut() {
                        let row = (*b >> 4) as usize;
                        let col = (*b & 0x0F) as usize;
                        *b = INV_S_BOXES[row][col];
                    }
                    Self::from_le_bytes(bytes)
                }
            }
        )*
    };
}

impl_sbox_lookup!(u8, u16, u32, u64, u128);
