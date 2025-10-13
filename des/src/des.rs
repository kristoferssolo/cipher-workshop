use crate::key::subkeys::Subkeys;
use cipher_core::KeyLike;

#[derive(Debug)]
pub struct Des {
    subkeys: Subkeys,
}

impl Des {
    #[must_use]
    pub fn new(_key: impl KeyLike) -> Self {
        todo!()
    }
}
