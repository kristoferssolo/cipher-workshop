use crate::{Block128, key::Subkey};

pub const fn add_round_key(state: Block128, subkeys: &[Subkey; 4]) -> Block128 {
    let k0 = subkeys[0].as_u128();
    let k1 = subkeys[1].as_u128();
    let k2 = subkeys[2].as_u128();
    let k3 = subkeys[3].as_u128();
    let key_block = (k0 << 96) | (k1 << 64) | (k2 << 32) | k3;
    Block128::new(state.as_u128() ^ key_block)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const TEST_KEY: u128 = 0x0F15_71C9_47D9_E859_1CB7_ADD6_AF7F_6798;

    #[rstest]
    #[case(0x0000_0000_0000_0000_0000_0000_0000_0000)]
    #[case(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF)]
    #[case(0x1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0)]
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
