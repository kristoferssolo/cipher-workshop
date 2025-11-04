use crate::block::{lr::LR, secret_block};
use cipher_core::{BlockError, InputBlock};
use std::{
    slice::{from_raw_parts, from_raw_parts_mut},
    str::FromStr,
};

secret_block! {
    pub struct Block64(u64, 64, 0xFFFF_FFFF_FFFF_FFFF);
}

impl InputBlock for Block64 {
    const BLOCK_SIZE: usize = 64;
    fn as_bytes(&self) -> &[u8] {
        unsafe { from_raw_parts((&raw const self.0).cast::<u64>().cast::<u8>(), 8) }
    }
    fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { from_raw_parts_mut((&raw mut self.0).cast::<u64>().cast::<u8>(), 8) }
    }
}

impl Block64 {
    #[inline]
    #[must_use]
    pub const fn from_be_bytes(bytes: [u8; 8]) -> Self {
        Self(u64::from_be_bytes(bytes))
    }

    #[inline]
    #[must_use]
    pub const fn to_be_bytes(self) -> [u8; 8] {
        self.0.to_be_bytes()
    }

    #[inline]
    #[must_use]
    pub const fn to_le_bytes(self) -> [u8; 8] {
        self.0.to_le_bytes()
    }

    #[inline]
    #[must_use]
    pub fn split_lr(self) -> LR {
        self.into()
    }
}

impl FromStr for Block64 {
    type Err = BlockError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(parse_string_to_u64(s)?))
    }
}

fn parse_string_to_u64(s: &str) -> Result<u64, BlockError> {
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

    // 8-character ASCII string conversion to u64
    if trimmed.len() > 8 {
        return Err(BlockError::InvalidByteStringLength(trimmed.len()));
    }

    ascii_string_to_u64(trimmed)
}

fn parse_radix(s: &str, radix: u32) -> Result<u64, BlockError> {
    let trimmed = s.trim_start_matches('0');
    if trimmed.is_empty() {
        return Ok(0);
    }

    u64::from_str_radix(trimmed, radix).map_err(BlockError::from)
}

fn ascii_string_to_u64(s: &str) -> Result<u64, BlockError> {
    if !s.is_ascii() {
        return Err(BlockError::conversion_error(
            "u64",
            "String contains non-ASCII characters",
        ));
    }

    let mut bytes = [0; 8];
    for (idx, byte) in s.bytes().enumerate() {
        bytes[idx] = byte;
    }

    Ok(u64::from_be_bytes(bytes))
}

impl From<[u8; 8]> for Block64 {
    fn from(bytes: [u8; 8]) -> Self {
        Self::from_be_bytes(bytes)
    }
}

impl From<Block64> for LR {
    fn from(block: Block64) -> Self {
        let left = (block.0 >> 32) as u32;
        let right = (block.0 & 0xFFFF_FFFF) as u32;
        Self::new(left, right)
    }
}

impl From<&Block64> for LR {
    fn from(block: &Block64) -> Self {
        let left = (block.0 >> 32) as u32;
        let right = (block.0 & 0xFFFF_FFFF) as u32;
        Self::new(left, right)
    }
}

impl From<Block64> for Vec<u8> {
    fn from(value: Block64) -> Self {
        value.0.to_be_bytes().to_vec()
    }
}

impl From<&Block64> for Vec<u8> {
    fn from(value: &Block64) -> Self {
        value.0.to_be_bytes().to_vec()
    }
}
