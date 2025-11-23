use crate::{
    Block128,
    key::{Key, Subkey, Subkeys},
    sbox::SboxLookup,
};
use cipher_core::{BlockCipher, CipherError};

pub struct Aes {
    subkeys: Subkeys,
}

impl Aes {
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            subkeys: Subkeys::from_key(&key.into()),
        }
    }

    fn encryot_block(&self, mut state: Block128) -> Block128 {
        let mut keys = self.subkeys.chunks();
        state = add_round_key(state, keys.next().expect("Round key 0"));

        for _ in 1..10 {
            state = state.sub_bytes();
            state = state.shift_rows();
            state = state.mix_columns();
            state = add_round_key(state, keys.next().expect("Round key"));
        }

        // Final round: SubBytes, ShiftRows, AddRoundKey (no MixColumns)
        state = state.sub_bytes();
        state = state.shift_rows();
        state = add_round_key(state, keys.next().expect("Final Round key"));

        state
    }

    fn decryot_block(&self, mut state: Block128) -> Block128 {
        let mut keys = self.subkeys.chunks();
        state = add_round_key(state, keys.next().expect("Final round key"));

        for _ in 1..10 {
            state = state.inv_shift_rows();
            state = state.inv_sub_bytes();
            state = add_round_key(state, keys.next().expect("Round key"));
            state = state.inv_mix_columns();
        }

        // Final round: SubBytes, ShiftRows, AddRoundKey (no MixColumns)
        state = state.inv_shift_rows();
        state = state.inv_sub_bytes();
        state = add_round_key(state, keys.next().expect("Round key 0"));

        state
    }
}

impl BlockCipher for Aes {
    const BLOCK_SIZE: usize = 16;
    fn from_key(key: &[u8]) -> Self {
        Self::new(key)
    }

    fn transform_impl(
        &self,
        block: &[u8],
        action: cipher_core::CipherAction,
    ) -> cipher_core::CipherResult<cipher_core::Output> {
        let block_arr: [u8; Self::BLOCK_SIZE] = block
            .try_into()
            .map_err(|_| CipherError::invalid_block_size(Self::BLOCK_SIZE, block.len()))?;

        let block128 = Block128::from_be_bytes(block_arr);

        let result = match action {
            cipher_core::CipherAction::Encrypt => self.encryot_block(block128),
            cipher_core::CipherAction::Decrypt => self.decryot_block(block128),
        };

        Ok(result.into())
    }
}

const fn add_round_key(state: Block128, subkeys: &[Subkey; 4]) -> Block128 {
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

    const TEST_KEY: u128 = 0x2B7E_1516_28AE_D2A6_ABF7_1588_09CF_4F3C;

    #[rstest]
    #[case(0x0000_0000_0000_0000_0000_0000_0000_0000)]
    #[case(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF)]
    #[case(0x1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0)]
    fn add_round_key_roundtrip(#[case] plaintext: u128) {
        let aes = Aes::new(TEST_KEY);
        let state = Block128::new(plaintext);

        // Get first round key
        let mut keys = aes.subkeys.chunks();
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
