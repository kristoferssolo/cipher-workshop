use crate::key::Subkeys;
use cipher_core::{CryptoResult, KeyLike};

#[derive(Debug)]
pub struct Des {
    subkeys: Subkeys,
}

impl Des {
    pub fn new(key: &impl KeyLike) -> CryptoResult<Self> {
        let subkeys = Subkeys::from_key(key)?;
        Ok(Self { subkeys })
    }
}
