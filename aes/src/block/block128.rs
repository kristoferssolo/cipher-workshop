use crate::{
    block::{Block32, secret_block},
    sbox::SboxLookup,
};
use cipher_core::{parse_block_int, BlockError, InputBlock};
use std::{
    ops::BitXor,
    slice::{from_raw_parts, from_raw_parts_mut},
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
}

impl FromStr for Block128 {
    type Err = BlockError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(parse_block_int(s)?))
    }
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

impl BitXor<u128> for Block128 {
    type Output = Self;
    fn bitxor(self, rhs: u128) -> Self::Output {
        Self(self.0 ^ rhs)
    }
}
