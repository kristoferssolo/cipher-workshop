use crate::{
    Block128,
    block::Block32,
    key::{Key, Subkey, Subkeys},
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

        let round_key = add_round_key(
            *self.subkeys.first().unwrap(),
            *block128.as_block32_array().first().unwrap(),
        );
        todo!()
    }
}

fn add_round_key(subkey: Subkey, block: Block32) -> Block32 {
    block ^ subkey
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const TEST_MESSAGE: u128 = 0x0123_4567_89AB_CDEF_FEDC_BA98_7654_3210;

    #[rstest]
    #[case(0x0123_4567, 0x0F15_71C9, 0x0E36_34AE)]
    #[case(0x89AB_CDEF, 0x47D9_E859, 0xCE72_25B6)]
    #[case(0xFEDC_BA98, 0x1CB7_ADD6, 0xE26B_174E)]
    #[case(0x7654_3210, 0xAF7F_6798, 0xD92B_5588)]
    fn round_key(#[case] block: u32, #[case] subkey: u32, #[case] expected: u32) {
        let block = Block32::new(block);
        let subkey = Subkey::from_u32(subkey);

        let result = add_round_key(subkey, block);

        assert_eq!(result.as_u32(), expected);
    }
}
