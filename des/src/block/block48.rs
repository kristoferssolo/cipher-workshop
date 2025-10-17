use crate::{block::Block6, key::Subkey, secret_block};
use std::{array, ops::BitXor};

secret_block! {
    pub struct Block48(u64, 48, 0xFFFF_FFFF_FFFF);
}

impl Block48 {
    #[must_use]
    pub fn as_block6_array(self) -> [Block6; 8] {
        array::from_fn(|idx| {
            let start_bit = 42 - (u8::try_from(idx).expect("8-bit number") * 6); // S-box 0: bit 42, S-box 7: bit 5
            let six_bits = u8::try_from((self.0 >> start_bit) & 0x3F).expect("6-bit number");
            Block6::new(six_bits)
        })
    }
}

impl BitXor for Block48 {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXor<Subkey> for Block48 {
    type Output = Self;
    fn bitxor(self, rhs: Subkey) -> Self::Output {
        Self(self.0 ^ rhs.as_u64())
    }
}

impl BitXor<&Subkey> for Block48 {
    type Output = Self;
    fn bitxor(self, rhs: &Subkey) -> Self::Output {
        Self(self.0 ^ rhs.as_u64())
    }
}

impl BitXor<Subkey> for &Block48 {
    type Output = Block48;
    fn bitxor(self, rhs: Subkey) -> Self::Output {
        Block48(self.0 ^ rhs.as_u64())
    }
}

impl BitXor<&Subkey> for &Block48 {
    type Output = Block48;
    fn bitxor(self, rhs: &Subkey) -> Self::Output {
        Block48(self.0 ^ rhs.as_u64())
    }
}
