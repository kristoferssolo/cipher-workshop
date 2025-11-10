use cipher_core::BlockCipher;

use crate::key::Key;

pub struct Aes {}

impl Aes {
    pub fn new(_key: impl Into<Key>) -> Self {
        todo!()
    }
}

impl BlockCipher for Aes {
    const BLOCK_SIZE: usize = 16;
    fn from_key(key: &[u8]) -> Self {
        Self::new(key)
    }

    fn transform_impl(
        &self,
        _block: &[u8],
        _action: cipher_core::CipherAction,
    ) -> cipher_core::CipherResult<cipher_core::Output> {
        todo!()
    }
}
