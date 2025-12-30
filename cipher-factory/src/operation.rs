use std::{convert::Infallible, fmt::Display, str::FromStr};

/// Cipher operation mode - encrypt or decrypt.
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OperationMode {
    /// Transform plaintext to ciphertext.
    #[default]
    Encrypt,
    /// Transform ciphertext to plaintext.
    Decrypt,
}

impl OperationMode {
    /// Returns the opposite operation mode.
    #[must_use]
    pub const fn invert(self) -> Self {
        match self {
            Self::Encrypt => Self::Decrypt,
            Self::Decrypt => Self::Encrypt,
        }
    }
}

impl Display for OperationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Encrypt => "Encrypt",
            Self::Decrypt => "Decrypt",
        };
        f.write_str(s)
    }
}

impl FromStr for OperationMode {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_ref() {
            "decrypt" => Ok(Self::Decrypt),
            _ => Ok(Self::Encrypt),
        }
    }
}
