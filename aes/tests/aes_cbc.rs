use aes::{AesCbc, Iv};
use claims::assert_ok;

// NIST SP 800-38A test vectors for AES-128-CBC
const NIST_KEY: u128 = 0x2b7e_1516_28ae_d2a6_abf7_1588_09cf_4f3c;
const NIST_IV: u128 = 0x0001_0203_0405_0607_0809_0a0b_0c0d_0e0f;

// Test vector blocks (plaintext -> ciphertext)
const NIST_BLOCKS: [(u128, u128); 4] = [
    (
        0x6bc1_bee2_2e40_9f96_e93d_7e11_7393_172a,
        0x7649_abac_8119_b246_cee9_8e9b_12e9_197d,
    ),
    (
        0xae2d_8a57_1e03_ac9c_9eb7_6fac_45af_8e51,
        0x5086_cb9b_5072_19ee_95db_113a_9176_78b2,
    ),
    (
        0x30c8_1c46_a35c_e411_e5fb_c119_1a0a_52ef,
        0x73be_d6b8_e3c1_743b_7116_e69e_2222_9516,
    ),
    (
        0xf69f_2445_df4f_9b17_ad2b_417b_e66c_3710,
        0x3ff1_caa1_681f_ac09_120e_ca30_7586_e1a1,
    ),
];

#[test]
fn nist_single_block_encrypt() {
    let cipher = AesCbc::new(NIST_KEY, Iv::new(NIST_IV));
    let plaintext = NIST_BLOCKS[0].0.to_be_bytes();
    let expected = NIST_BLOCKS[0].1.to_be_bytes();

    let ciphertext = assert_ok!(cipher.encrypt(&plaintext));

    // 16 IV + 16 block + 16 padding = 48 bytes
    assert_eq!(ciphertext.len(), 48);
    // First 16 bytes are IV, next 16 are the ciphertext
    assert_eq!(&ciphertext[16..32], &expected);
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

    // 16 IV + 64 blocks + 16 padding = 96 bytes
    assert_eq!(ciphertext.len(), 96);
    // First 16 bytes are IV, then ciphertext blocks
    assert_eq!(&ciphertext[16..64], &expected[..48]);
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
    // 16 IV + 16 padding = 32 bytes
    assert_eq!(ciphertext.len(), 32);

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
