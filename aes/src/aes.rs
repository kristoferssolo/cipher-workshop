use crate::{
    Block128,
    key::{Key, Subkeys},
    operations::{
        add_round_key, inv_mix_columns, inv_shift_rows, inv_sub_bytes, mix_columns, shift_rows,
        sub_bytes,
    },
};
use cipher_core::{BlockCipher, CipherAction, CipherError};

pub struct Aes {
    subkeys: Subkeys,
}

impl Aes {
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            subkeys: Subkeys::from_key(&key.into()),
        }
    }

    #[cfg(test)]
    #[inline]
    #[must_use]
    pub const fn subkeys(&self) -> &Subkeys {
        &self.subkeys
    }

    fn encrypt_block(&self, mut state: Block128) -> Block128 {
        let mut keys = self.subkeys.chunks();
        state = add_round_key(state, keys.next().expect("Round key 0"));

        for _ in 1..10 {
            state = sub_bytes(state);
            state = shift_rows(state);
            state = mix_columns(state);
            state = add_round_key(state, keys.next().expect("Round key"));
        }

        // Final round: SubBytes, ShiftRows, AddRoundKey (no MixColumns)
        state = sub_bytes(state);
        state = shift_rows(state);
        state = add_round_key(state, keys.next().expect("Final Round key"));

        state
    }

    fn decrypt_block(&self, mut state: Block128) -> Block128 {
        let mut keys = self.subkeys.chunks_rev();
        state = add_round_key(state, keys.next().expect("Final round key"));

        for _ in 1..10 {
            state = inv_shift_rows(state);
            state = inv_sub_bytes(state);
            state = add_round_key(state, keys.next().expect("Round key"));
            state = inv_mix_columns(state);
        }

        // Final round: SubBytes, ShiftRows, AddRoundKey (no MixColumns)
        state = inv_shift_rows(state);
        state = inv_sub_bytes(state);
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
            CipherAction::Encrypt => self.encrypt_block(block128),
            CipherAction::Decrypt => self.decrypt_block(block128),
        };

        Ok(result.into())
    }
}
