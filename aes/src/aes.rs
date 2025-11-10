use crate::key::Key;

pub struct Aes {}

impl Aes {
    pub fn new(_key: impl Into<Key>) -> Self {
        todo!()
    }
}

impl Aes {
    const BLOCK_SIZE: usize = 16;

    fn from_key(key: &[u8]) -> Self {
        Self::new(key)
    }
}
