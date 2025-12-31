//! NIST SP 800-38A AES-128-CBC test vectors.

use aes::{AesCbc, Iv};
use claims::assert_ok;

// NIST SP 800-38A test vectors for AES-128-CBC
const NIST_KEY: u128 = 0x2b7e1516_28aed2a6_abf71588_09cf4f3c;
const NIST_IV: u128 = 0x00010203_04050607_08090a0b_0c0d0e0f;

// Test vector blocks (plaintext -> ciphertext)
const NIST_BLOCKS: [(u128, u128); 4] = [
    (
        0x6bc1bee2_2e409f96_e93d7e11_7393172a,
        0x7649abac_8119b246_cee98e9b_12e9197d,
    ),
    (
        0xae2d8a57_1e03ac9c_9eb76fac_45af8e51,
        0x5086cb9b_507219ee_95db113a_917678b2,
    ),
    (
        0x30c81c46_a35ce411_e5fbc119_1a0a52ef,
        0x73bed6b8_e3c1743b_7116e69e_22229516,
    ),
    (
        0xf69f2445_df4f9b17_ad2b417b_e66c3710,
        0x3ff1caa1_681fac09_120eca30_7586e1a1,
    ),
];

#[test]
fn nist_single_block_encrypt() {
    let cipher = AesCbc::new(NIST_KEY, Iv::new(NIST_IV));
    let plaintext = NIST_BLOCKS[0].0.to_be_bytes();
    let expected = NIST_BLOCKS[0].1.to_be_bytes();

    let ciphertext = assert_ok!(cipher.encrypt(&plaintext));

    // Result includes PKCS#7 padding (16 bytes padding for aligned input)
    assert_eq!(ciphertext.len(), 32);
    assert_eq!(&ciphertext[..16], &expected);
}

#[test]
fn nist_single_block_decrypt() {
    let cipher = AesCbc::new(NIST_KEY, Iv::new(NIST_IV));
    let plaintext = NIST_BLOCKS[0].0.to_be_bytes();

    // First encrypt to get valid padded ciphertext
    let ciphertext = assert_ok!(cipher.encrypt(&plaintext));
    let decrypted = assert_ok!(cipher.decrypt(&ciphertext));

    assert_eq!(decrypted, plaintext);
}

#[test]
fn nist_multi_block_encrypt() {
    let cipher = AesCbc::new(NIST_KEY, Iv::new(NIST_IV));

    // Concatenate all 4 plaintext blocks (64 bytes)
    let mut plaintext = Vec::with_capacity(64);
    for (pt, _) in &NIST_BLOCKS {
        plaintext.extend_from_slice(&pt.to_be_bytes());
    }

    // Concatenate expected ciphertext blocks
    let mut expected = Vec::with_capacity(64);
    for (_, ct) in &NIST_BLOCKS {
        expected.extend_from_slice(&ct.to_be_bytes());
    }

    let ciphertext = assert_ok!(cipher.encrypt(&plaintext));

    // Result includes padding (64 + 16 = 80 bytes)
    assert_eq!(ciphertext.len(), 80);
    // First 3 blocks should match NIST vectors exactly
    assert_eq!(&ciphertext[..48], &expected[..48]);
}

#[test]
fn nist_multi_block_roundtrip() {
    let cipher = AesCbc::new(NIST_KEY, Iv::new(NIST_IV));

    let mut plaintext = Vec::with_capacity(64);
    for (pt, _) in &NIST_BLOCKS {
        plaintext.extend_from_slice(&pt.to_be_bytes());
    }

    let ciphertext = assert_ok!(cipher.encrypt(&plaintext));
    let decrypted = assert_ok!(cipher.decrypt(&ciphertext));

    assert_eq!(decrypted, plaintext);
}

#[test]
fn empty_plaintext() {
    let cipher = AesCbc::new(NIST_KEY, Iv::new(NIST_IV));

    let ciphertext = assert_ok!(cipher.encrypt(&[]));
    // Empty input gets full block of padding
    assert_eq!(ciphertext.len(), 16);

    let decrypted = assert_ok!(cipher.decrypt(&ciphertext));
    assert!(decrypted.is_empty());
}

#[test]
fn arbitrary_length_plaintext() {
    let cipher = AesCbc::new(NIST_KEY, Iv::new(NIST_IV));

    let plaintext = b"This is a test message with arbitrary length!";
    let ciphertext = assert_ok!(cipher.encrypt(plaintext));
    let decrypted = assert_ok!(cipher.decrypt(&ciphertext));

    assert_eq!(decrypted, plaintext);
}
