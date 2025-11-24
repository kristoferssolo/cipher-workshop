use aes::Aes;
use cipher_core::BlockCipher;
use rstest::rstest;

const TEST_KEY: u128 = 0x2B7E_1516_28AE_D2A6_ABF7_1588_09CF_4F3C;
const TEST_PLAINTEXT: u128 = 0x6BC1_BEE2_2E40_9F96_E93D_7E11_7393_1728;
const TEST_CIPHERTEXT: u128 = 0x79BD_98A6_CB0F_D3AE_3D7D_C1A3_3CD3_6E2F;

// #[rstest]
// #[case(TEST_PLAINTEXT, TEST_CIPHERTEXT, TEST_KEY)]
// #[case(
//     0xAE2D_8A57_1E03_AC9C_9EB7_6FAC_45AF_8E51,
//     0xBC14_003D_01C9_B46C_AC63_D481_5210_E80B,
//     0xF5D3_D585_03B9_699D_E785_895A_96FD_BAAF
// )]
// #[case(
//     0x30C8_1C46_A35C_E411_E5FB_C119_1A0A_52EF,
//     0xB9AF_FEE2_98CD_0F4A_6708_44A6_D6CE_EF87,
//     0x43B1_CD7F_598E_CE23_881B_00E3_ED03_0688
// )]
// #[case(
//     0xF69F_2445_DF4F_9B17_AD2B_417B_E66C_3710,
//     0xA279_FA71_A91B_9FA9_213C_E13E_659D_5C3B,
//     0x7B0C_785E_27E8_AD3F_8223_2071_0472_5DD4
// )]
// fn encrypt_decrypt_roundtrip(
//     #[case] plaintext: u128,
//     #[case] expected_ciphertext: u128,
//     #[case] key: u128,
// ) {
//     let aes = Aes::new(key);
//     let pt_bytes = plaintext.to_be_bytes();
//
//     // Test Encrypt
//     let ciphertext = aes.encrypt(&pt_bytes).expect("Encryption failed");
//     let ciphertext_u128 = u128::from_be_bytes(ciphertext.as_slice().try_into().unwrap());
//
//     assert_eq!(
//         ciphertext_u128, expected_ciphertext,
//         "Encryption mismatch. Expected 0x{expected_ciphertext:032X}, got 0x{ciphertext_u128:032X}"
//     );
//
//     // Test Decrypt
//     let decrypted = aes.decrypt(&ciphertext).expect("Decryption failed");
//     let decrypted_u128 = u128::from_be_bytes(decrypted.as_slice().try_into().unwrap());
//
//     assert_eq!(
//         decrypted_u128, plaintext,
//         "Decryption mismatch. Expected 0x{plaintext:032X}, got 0x{decrypted_u128:032X}"
//     );
// }
