use crate::{
    Block128,
    block::Block32,
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

        let mut subkey_iter = self.subkeys.chunks();
        dbg!(&subkey_iter.count());

        // let foo = *subkey_iter.next().unwrap();
        // let round_key = add_round_key(
        //     *block128.as_block32_array().first().unwrap(),
        //     *subkey_iter.next().unwrap(),
        // );

        // for i in subkey_iter {}
        todo!()
    }
}

fn add_round_key(block: Block32, subkey: Subkey) -> Block32 {
    block ^ subkey
}

fn substitute_bytes(block: Block128) -> Block128 {
    block.sbox_lookup()
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

        let result = add_round_key(block, subkey);

        assert_eq!(result.as_u32(), expected);
    }

    #[rstest]
    #[case(
        0x0E36_34AE_CE72_25B6_E26B_174E_D92B_5588,
        0xAB05_18E4_8B40_3F4E_987F_F02F_35F1_FCC4
    )]
    fn byte_substitution(#[case] block: u128, #[case] expected: u128) {
        let block = Block128::new(block);

        let result = substitute_bytes(block);
        assert_eq!(result.as_u128(), expected);
    }
}
