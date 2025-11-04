use crate::args::AlgorithmChoice;
use cipher_core::{BlockCipher, InputBlock};
use des::Des;

impl AlgorithmChoice {
    #[must_use]
    pub fn get_cipher(&self, key: &impl InputBlock) -> impl BlockCipher {
        match self {
            Self::Des => Des::from_key(key.as_bytes()),
            Self::Aes => todo!("Must implement AES first"),
        }
    }
}
