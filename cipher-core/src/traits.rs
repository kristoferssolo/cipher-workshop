use crate::{CipherAction, CipherError, CipherResult};

pub trait BlockCipher: Sized {
    const BLOCK_SIZE: usize;

    fn transform_impl(&self, block: &[u8], action: CipherAction) -> CipherResult<Vec<u8>>;

    fn transform(&self, block: &[u8], action: CipherAction) -> CipherResult<Vec<u8>> {
        if block.len() != Self::BLOCK_SIZE {
            return Err(CipherError::invalid_block_size(
                Self::BLOCK_SIZE,
                block.len(),
            ));
        }
        self.transform_impl(block, action)
    }

    fn encrypt(&self, plaintext: &[u8]) -> CipherResult<Vec<u8>> {
        self.transform(plaintext, CipherAction::Encrypt)
    }
    fn decrypt(&self, ciphertext: &[u8]) -> CipherResult<Vec<u8>> {
        self.transform(ciphertext, CipherAction::Decrypt)
    }
}
