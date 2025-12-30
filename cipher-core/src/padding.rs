//! PKCS#7 padding for block ciphers.
//!
//! PKCS#7 pads data to a multiple of the block size by appending N bytes
//! of value N, where N is the number of padding bytes needed.
//!
//! # Example
//!
//! For a 16-byte block size:
//! - 15 bytes of data → add 1 byte of value `0x01`
//! - 14 bytes of data → add 2 bytes of value `0x02`
//! - 16 bytes of data → add 16 bytes of value `0x10` (full padding block)

use crate::CipherError;

/// Applies PKCS#7 padding to input data.
///
/// Pads the data to a multiple of `block_size` by appending N bytes of value N.
/// If data is already aligned, a full block of padding is added.
///
/// # Panics
///
/// Panics if `block_size` is 0 or greater than 255.
#[must_use]
pub fn pkcs7_pad(data: &[u8], block_size: usize) -> Vec<u8> {
    assert!(
        block_size > 0 && block_size <= 255,
        "block_size must be 1-255"
    );

    let padding_len = block_size - (data.len() % block_size);
    let mut padded = Vec::with_capacity(data.len() + padding_len);
    padded.extend_from_slice(data);

    #[allow(clippy::cast_possible_truncation)]
    let pad_byte = padding_len as u8;
    padded.resize(data.len() + padding_len, pad_byte);

    padded
}

/// Removes PKCS#7 padding from decrypted data.
///
/// Validates the padding and returns the unpadded data slice.
///
/// # Errors
///
/// Returns `CipherError::InvalidPadding` if:
/// - Data is empty
/// - Padding byte value is 0 or exceeds block size
/// - Padding bytes are inconsistent
/// - There aren't enough bytes for the claimed padding
pub fn pkcs7_unpad(data: &[u8], block_size: usize) -> Result<&[u8], CipherError> {
    if data.is_empty() {
        return Err(CipherError::InvalidPadding("data is empty".into()));
    }

    let last_byte = data[data.len() - 1];
    let padding_len = last_byte as usize;

    // Validate padding length
    if padding_len == 0 || padding_len > block_size {
        return Err(CipherError::InvalidPadding(format!(
            "invalid padding byte: 0x{last_byte:02X}"
        )));
    }

    if padding_len > data.len() {
        return Err(CipherError::InvalidPadding(format!(
            "padding length {padding_len} exceeds data length {}",
            data.len()
        )));
    }

    // Verify all padding bytes are consistent
    let padding_start = data.len() - padding_len;
    for (i, &byte) in data[padding_start..].iter().enumerate() {
        if byte != last_byte {
            return Err(CipherError::InvalidPadding(format!(
                "inconsistent padding at byte {i}: expected 0x{last_byte:02X}, got 0x{byte:02X}"
            )));
        }
    }

    Ok(&data[..padding_start])
}

#[cfg(test)]
mod tests {
    use super::*;
    use claims::{assert_err, assert_ok};

    #[test]
    fn pad_empty() {
        let padded = pkcs7_pad(&[], 16);
        assert_eq!(padded.len(), 16);
        assert!(padded.iter().all(|&b| b == 16));
    }

    #[test]
    fn pad_one_byte_short() {
        let data = [0u8; 15];
        let padded = pkcs7_pad(&data, 16);
        assert_eq!(padded.len(), 16);
        assert_eq!(padded[15], 1);
    }

    #[test]
    fn pad_aligned_adds_full_block() {
        let data = [0u8; 16];
        let padded = pkcs7_pad(&data, 16);
        assert_eq!(padded.len(), 32);
        assert!(padded[16..].iter().all(|&b| b == 16));
    }

    #[test]
    fn pad_partial_block() {
        let data = b"hello";
        let padded = pkcs7_pad(data, 16);
        assert_eq!(padded.len(), 16);
        assert_eq!(&padded[..5], b"hello");
        // 11 bytes of padding with value 0x0B
        assert!(padded[5..].iter().all(|&b| b == 11));
    }

    #[test]
    fn unpad_valid() {
        let data = [0u8, 0u8, 0u8, 3u8, 3u8, 3u8];
        let unpadded = assert_ok!(pkcs7_unpad(&data, 16));
        assert_eq!(unpadded, &[0u8, 0u8, 0u8]);
    }

    #[test]
    fn unpad_full_block_padding() {
        let mut data = vec![0u8; 16];
        data.extend([16u8; 16]);
        let unpadded = assert_ok!(pkcs7_unpad(&data, 16));
        assert_eq!(unpadded.len(), 16);
    }

    #[test]
    fn unpad_empty_fails() {
        assert_err!(pkcs7_unpad(&[], 16));
    }

    #[test]
    fn unpad_zero_padding_fails() {
        let data = [1u8, 2u8, 0u8];
        assert_err!(pkcs7_unpad(&data, 16));
    }

    #[test]
    fn unpad_padding_exceeds_block_size_fails() {
        let data = [1u8, 2u8, 17u8];
        assert_err!(pkcs7_unpad(&data, 16));
    }

    #[test]
    fn unpad_inconsistent_padding_fails() {
        let data = [1u8, 2u8, 3u8, 2u8];
        assert_err!(pkcs7_unpad(&data, 16));
    }

    #[test]
    fn roundtrip() {
        let original = b"The quick brown fox";
        let padded = pkcs7_pad(original, 16);
        let unpadded = assert_ok!(pkcs7_unpad(&padded, 16));
        assert_eq!(unpadded, original);
    }
}
