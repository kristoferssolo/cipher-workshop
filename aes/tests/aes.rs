use aes::Aes;
use cipher_core::BlockCipher;
use rstest::rstest;

const TEST_KEY: u128 = 0x0F15_71C9_47D9_E859_1CB7_ADD6_AF7F_6798;
const TEST_PLAINTEXT: u128 = 0x0123_4567_89AB_CDEF_FEDC_BA98_7654_3210;
const TEST_CIPHERTEXT: u128 = 0x34D3_F0EE_CB4D_FA16_CB8B_F07F_29A0_CB79;

#[rstest]
#[case(TEST_PLAINTEXT, TEST_CIPHERTEXT, TEST_KEY)]
fn encrypt_decrypt_roundtrip(
    #[case] plaintext: u128,
    #[case] expected_ciphertext: u128,
    #[case] key: u128,
) {
    let aes = Aes::new(key);

    // Encrypt
    let ciphertext = aes
        .encrypt(&plaintext.to_be_bytes())
        .expect("Encryption failed");
    let ciphertext_u128 = u128::from_be_bytes(ciphertext.as_slice().try_into().unwrap());

    assert_eq!(
        ciphertext_u128, expected_ciphertext,
        "Encryption failed. Expected 0x{expected_ciphertext:032X}, got 0x{ciphertext_u128:032X}"
    );

    // Decrypt
    let decrypted = aes.decrypt(&ciphertext).expect("Decryption failed");
    let decrypted_u128 = u128::from_be_bytes(decrypted.as_slice().try_into().unwrap());

    assert_eq!(
        decrypted_u128, plaintext,
        "Decryption failed. Expected 0x{plaintext:032X}, got 0x{decrypted_u128:032X}"
    );
}
