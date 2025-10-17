use crate::{args::AlgorithmChoice, value::Value};
use cipher_core::BlockCipher;
use des::Des;

impl AlgorithmChoice {
    #[must_use]
    pub fn get_cipher(&self, key: Value) -> impl BlockCipher {
        let key = key.to_be_bytes();
        match self {
            Self::Des => Des::from_key(&key),
            Self::Aes => todo!("Must implement AES first"),
        }
    }
}
