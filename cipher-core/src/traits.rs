use crate::CryptoResult;

/// Minimal trait describing a fixed-size block-like value.
///
/// Concrete block types (e.g. `Block<const N: usize>`) should implement this.
pub trait BlockLike: Sized + Copy + Clone {
    /// Size of the block in bytes.
    const SIZE: usize;

    /// Create from exactly SIZE bytes.
    ///
    /// # Errors
    ///
    /// Returns a `CryptoError::InvalidBlockSize` (or equivalent) when
    /// `bytes.len() != Self::SIZE`.
    fn from_bytes(bytes: &[u8]) -> CryptoResult<Self>;

    /// Immutable view of the underlying bytes.
    fn as_bytes(&self) -> &[u8];

    /// Mutable view of underlying bytes.
    fn as_bytes_mut(&mut self) -> &mut [u8];

    /// Create a zeroed block value.
    fn zeroed() -> Self;
}

/// Minimal trait describing a fixed-size key-like value.
///
/// Concrete key types (e.g. `Secret<const N: usize>`) should implement this.
pub trait KeyLike: Sized {
    /// Size of the key in bytes.
    const SIZE: usize;

    /// Create key from exactly SIZE bytes.
    ///
    /// # Errors
    ///
    /// Returns a `CryptoError::InvalidKeySize` when `bytes.len() != Self::SIZE`.
    fn from_bytes(bytes: &[u8]) -> CryptoResult<Self>;
    /// Immutable view of the key bytes.
    fn as_bytes(&self) -> &[u8];
}

/// Core block-cipher trait using [`KeyLike`] and [`BlockLike`].
///
/// The primary (performance-oriented) methods are the in-place variants.
/// Default owned-returning methods are implemented in terms of the in-place
/// ones and therefore require `BlockLike: Copy`.
pub trait BlockCipher: Sized {
    /// Key and Block concrete associated types
    type Key: KeyLike;
    type Block: BlockLike;

    /// Construct a cipher instance from a key.
    ///
    /// # Errors
    ///
    /// Returns a `CryptoError` if the key is invalid for the cipher.
    fn new(key: Self::Key) -> CryptoResult<Self>;

    /// Encrypt in-place (primary implementation target).
    ///
    /// # Errors
    ///
    /// Returns a `CryptoError` on failure (e.g. internal state issues).
    fn encrypt_inplace(&self, block: &mut Self::Block) -> CryptoResult<()>;

    /// Decrypt in-place (primary implementation target).
    ///
    /// # Errors
    ///
    /// Returns a `CryptoError` on failure (e.g. invalid padding after decrypt).
    fn decrypt_inplace(&self, block: &mut Self::Block) -> CryptoResult<()>;

    /// Encrypt returning a new block.
    ///
    /// Default implementation copies the input block and calls
    /// `encrypt_inplace`.
    ///
    /// # Errors
    ///
    /// Propagates errors returned by `encrypt_inplace`.
    fn encrypt(&self, block: &Self::Block) -> CryptoResult<Self::Block> {
        let mut out = *block;
        self.encrypt_inplace(&mut out)?;
        Ok(out)
    }

    /// Decrypt returning a new block.
    ///
    /// Default implementation copies the input block and calls
    /// `decrypt_inplace`.
    ///
    /// # Errors
    ///
    /// Propagates errors returned by `decrypt_inplace`.
    fn decrypt(&self, block: &Self::Block) -> CryptoResult<Self::Block> {
        let mut out = *block;
        self.decrypt_inplace(&mut out)?;
        Ok(out)
    }
}

/// Helper trait: initialize a cipher from raw key bytes.
///
/// The default implementation converts the slice into `Self::Key` then calls
/// `Self::new`. Implementations may override to perform custom validation.
pub trait KeyInit: BlockCipher + Sized {
    /// Construct the cipher from raw key bytes.
    ///
    /// # Errors
    ///
    /// Returns `CryptoError::InvalidKeySize` if the slice length doesn't match
    fn new_from_slice(key_bytes: &[u8]) -> CryptoResult<Self> {
        let key = <Self::Key as KeyLike>::from_bytes(key_bytes)?;
        Self::new(key)
    }
}

/// Stream-like cipher/mode trait (CTR, OFB, stream ciphers).
///
/// Implementations apply keystream to arbitrary-length buffers in-place.
pub trait StreamCipher {
    /// XOR keystream with `data` in-place (encrypt == decrypt).
    fn apply_keystream(&mut self, data: &mut [u8]);
}

/// Small convenience wrapper that stores a cipher and forwards single-block
/// operations using the associated Block type.
pub struct CipherContext<C>
where
    C: BlockCipher,
{
    cipher: C,
}

impl<C> CipherContext<C>
where
    C: BlockCipher,
{
    /// Wrap an existing cipher instance.
    pub const fn new(cipher: C) -> Self {
        Self { cipher }
    }

    /// Encrypt a block, returning a new block.
    ///
    /// # Errors
    ///
    /// Propagates errors from the cipher.
    pub fn encrypt(&self, block: &C::Block) -> CryptoResult<C::Block> {
        self.cipher.encrypt(block)
    }

    /// Encrypt a block in-place.
    ///
    /// # Errors
    ///
    /// Propagates errors from the cipher.   /// Encrypt a block in-place.
    pub fn encrypt_inplace(&self, block: &mut C::Block) -> CryptoResult<()> {
        self.cipher.encrypt_inplace(block)
    }

    /// Decrypt a block, returning a new block.
    ///
    /// # Errors
    ///
    /// Propagates errors from the cipher.   /// Decrypt a block, returning a new block.
    pub fn decrypt(&self, block: &C::Block) -> CryptoResult<C::Block> {
        self.cipher.decrypt(block)
    }

    /// Decrypt a block in-place.
    ///
    /// # Errors
    ///
    /// Propagates errors from the cipher.  /// Decrypt a block in-place.
    pub fn decrypt_inplace(&self, block: &mut C::Block) -> CryptoResult<()> {
        self.cipher.decrypt_inplace(block)
    }

    /// Access underlying cipher
    pub const fn cipher(&self) -> &C {
        &self.cipher
    }

    /// Consume and return underlying cipher
    pub fn into_inner(self) -> C {
        self.cipher
    }
}
