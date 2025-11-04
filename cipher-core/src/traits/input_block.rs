use std::ops::{Deref, DerefMut};

pub trait InputBlock: Sized {
    const BLOCK_SIZE: usize;

    fn as_bytes(&self) -> &[u8];
    fn as_bytes_mut(&mut self) -> &mut [u8];
}

#[derive(Debug, Clone)]
pub struct BlockParser<T: InputBlock>(pub T);

impl<T: InputBlock> Deref for BlockParser<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: InputBlock> DerefMut for BlockParser<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
