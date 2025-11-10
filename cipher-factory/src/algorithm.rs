use cipher_core::{BlockCipher, InputBlock};
use des::Des;
use std::fmt::Display;

#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Algorithm {
    Des,
    Aes,
}

impl Algorithm {
    #[must_use]
    pub fn get_cipher(&self, key: &impl InputBlock) -> impl BlockCipher {
        match self {
            Self::Des => Des::from_key(key.as_bytes()),
            Self::Aes => todo!("Must implement AES first"),
        }
    }
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Des => "Des",
            Self::Aes => "Aes",
        };
        f.write_str(s)
    }
}
