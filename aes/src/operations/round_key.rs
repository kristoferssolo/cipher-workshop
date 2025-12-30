use crate::{Block128, key::Subkey};

/// XORs the state with a round key ([`AddRoundKey`] step).
///
/// Each round of AES combines the current state with a derived subkey
/// using bitwise XOR. This operation is its own inverse.
#[must_use]
pub fn add_round_key(state: Block128, subkeys: &[Subkey; 4]) -> Block128 {
    let [k0, k1, k2, k3] = [subkeys[0], subkeys[1], subkeys[2], subkeys[3]];
    let key_block = (k0 << 96) | (k1 << 64) | (k2 << 32) | k3;
    state ^ key_block
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const TEST_KEY: u128 = 0x0F15_71C9_47D9_E859_1CB7_ADD6_AF7F_6798;
    const TEST_MESSAGE: u128 = 0x0123_4567_89AB_CDEF_FEDC_BA98_7654_3210;

    #[rstest]
    #[case(
        0x0123_4567_89AB_CDEF_FEDC_BA98_7654_3210,
        [0x0F15_71C9, 0x47D9_E859, 0x1CB7_ADD6, 0xAF7F_6798],
        0x0E36_34AE_CE72_25B6_E26B_174E_D92B_5588
    )]
    fn round_key_addition(#[case] input: u128, #[case] subkeys: [u32; 4], #[case] expected: u128) {
        let block = Block128::new(input);
        let subkeys = subkeys.map(Subkey::from_u32);
        let result = add_round_key(block, &subkeys).as_u128();
        assert_eq!(
            result, expected,
            "Adding Round Key failed. Expected 0x{expected:032X}, got 0x{result:032X}",
        );
    }

    #[rstest]
    #[case(TEST_MESSAGE)]
    #[case(0x0000_0000_0000_0000_0000_0000_0000_0000)]
    #[case(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF)]
    fn add_round_key_roundtrip(#[case] plaintext: u128) {
        use crate::Aes;

        let aes = Aes::new(TEST_KEY);
        let state = Block128::new(plaintext);

        // Get first round key
        let mut keys = aes.subkeys().chunks();
        let first_key = keys.next().expect("First round key");

        // AddRoundKey twice should return to original
        let xored_once = add_round_key(state, first_key);
        let xored_twice = add_round_key(xored_once, first_key);

        assert_eq!(
            xored_twice.as_u128(),
            plaintext,
            "AddRoundKey should be self-inverse (double XOR returns to original)"
        );
    }
}
