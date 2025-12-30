use crate::BlockError;
use std::{any::type_name, num::ParseIntError};

/// Trait for integer types that can be parsed from block string formats.
///
/// Implemented for `u64` and `u128` to support DES (64-bit) and AES (128-bit) block parsing.
pub trait BlockInt: Sized + Copy {
    /// Number of bytes this integer type represents.
    const BYTE_SIZE: usize;

    /// Parse from string with given radix.
    ///
    /// # Errors
    /// Returns `ParseIntError` if the string contains invalid digits for the radix.
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError>;

    /// Construct from big-endian bytes (zero-padded on the left).
    fn from_be_bytes_padded(bytes: &[u8]) -> Self;
}

impl BlockInt for u64 {
    const BYTE_SIZE: usize = 8;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError> {
        Self::from_str_radix(s, radix)
    }

    fn from_be_bytes_padded(bytes: &[u8]) -> Self {
        let mut arr = [0u8; 8];
        let offset = 8 - bytes.len();
        arr[offset..].copy_from_slice(bytes);
        Self::from_be_bytes(arr)
    }
}

impl BlockInt for u128 {
    const BYTE_SIZE: usize = 16;

    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError> {
        Self::from_str_radix(s, radix)
    }

    fn from_be_bytes_padded(bytes: &[u8]) -> Self {
        let mut arr = [0u8; 16];
        let offset = 16 - bytes.len();
        arr[offset..].copy_from_slice(bytes);
        Self::from_be_bytes(arr)
    }
}

/// Parse a string into a block integer, supporting hex (0x), binary (0b), and ASCII formats.
///
/// # Formats
/// - `0x...` or `0X...`: Hexadecimal
/// - `0b...` or `0B...`: Binary
/// - Otherwise: ASCII string (right-aligned, zero-padded)
///
/// # Errors
/// Returns `BlockError` if the string is empty, contains invalid characters,
/// or exceeds the maximum byte length for the target type.
pub fn parse_block_int<T: BlockInt>(s: &str) -> Result<T, BlockError> {
    let trimmed = s.trim();

    if trimmed.is_empty() {
        return Err(BlockError::EmptyBlock);
    }

    // Hexadecimal with 0x/0X prefix
    if let Some(hex_str) = trimmed
        .strip_prefix("0x")
        .or_else(|| trimmed.strip_prefix("0X"))
    {
        return parse_radix::<T>(hex_str, 16);
    }

    // Binary with 0b/0B prefix
    if let Some(bin_str) = trimmed
        .strip_prefix("0b")
        .or_else(|| trimmed.strip_prefix("0B"))
    {
        return parse_radix::<T>(bin_str, 2);
    }

    parse_ascii::<T>(trimmed)
}

fn parse_radix<T: BlockInt>(s: &str, radix: u32) -> Result<T, BlockError> {
    let trimmed = s.trim_start_matches('0');
    if trimmed.is_empty() {
        return Ok(T::from_be_bytes_padded(&[]));
    }

    T::from_str_radix(trimmed, radix).map_err(BlockError::from)
}

fn parse_ascii<T: BlockInt>(s: &str) -> Result<T, BlockError> {
    if s.len() > T::BYTE_SIZE {
        return Err(BlockError::InvalidByteStringLength {
            max: T::BYTE_SIZE,
            actual: s.len(),
        });
    }

    if !s.is_ascii() {
        return Err(BlockError::conversion_error(
            type_name::<T>(),
            "String contains non-ASCII characters",
        ));
    }

    Ok(T::from_be_bytes_padded(s.as_bytes()))
}
