use cipher_core::BlockCipher;
use rstest::rstest;

const TEST_KEY: u128 = 0x2B7E_1516_28AE_D2A6_ABF7_1588_09CF_4F3C;
const TEST_PLAINTEXT: u128 = 0x6BC1_BEE2_2E40_9F96_E93D_7E11_7393_1728;
const TEST_CIPHERTEXT: u128 = 0x3AD7_7BB4_0D7A_3660_A89E_CAF3_2466_EF97;

// #[rstest]
// #[case(TEST_PLAINTEXT, TEST_CIPHERTEXT, TEST_KEY)]
// // NIST SP 800-38A ECB mode test vectors
// #[case(
//     0xAE2D_8A57_1E03_AC9C_9EB7_6FAC_45AF_8E51,
//     0xF5D3_D585_03B9_699D_E785_895A_96FD_BAAF,
//     0x2B7E_1516_28AE_D2A6_ABF7_1588_09CF_4F3C
// )]
// #[case(
//     0x30C8_1C46_A35C_E411_E5FB_C119_1A0A_52EF,
//     0x43B1_CD7F_598E_CE23_881B_00E3_ED03_0688,
//     0x2B7E_1516_28AE_D2A6_ABF7_1588_09CF_4F3C
// )]
// #[case(
//     0xF69F_2445_DF4F_9B17_AD2B_417B_E66C_3710,
//     0x7B0C_785E_27E8_AD3F_8223_2071_0472_5DD4,
//     0x2B7E_1516_28AE_D2A6_ABF7_1588_09CF_4F3C
// )]
// fn encrypt_decrypt_roundtrip(
//     #[case] plaintext: u128,
//     #[case] expected_ciphertext: u128,
//     #[case] key: u128,
// ) {
//     use aes::Aes;
//
//     let aes = Aes::new(key);
//     let pt_bytes = plaintext.to_be_bytes();
//
//     // Test Encrypt
//     let ciphertext = aes.encrypt(&pt_bytes).expect("Encryption failed");
//     let ciphertext_u128 = u128::from_be_bytes(ciphertext.as_slice().try_into().unwrap());
//
//     assert_eq!(
//         ciphertext_u128, expected_ciphertext,
//         "Encryption mismatch.\nExpected: 0x{expected_ciphertext:032X}\nGot:      0x{ciphertext_u128:032X}"
//     );
//
//     // Test Decrypt
//     let decrypted = aes.decrypt(&ciphertext).expect("Decryption failed");
//     let decrypted_u128 = u128::from_be_bytes(decrypted.as_slice().try_into().unwrap());
//
//     assert_eq!(
//         decrypted_u128, plaintext,
//         "Decryption mismatch.\nExpected: 0x{plaintext:032X}\nGot:      0x{decrypted_u128:032X}"
//     );
// }
