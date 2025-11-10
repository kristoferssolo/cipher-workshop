use crate::key::secret_key;

secret_key! {
    /// A single AES round subkey
    pub struct Subkey(u32, 32, 0xFFFF_FFFF);
}

impl Subkey {
    /// Zero value.
    pub const fn zero() -> Self {
        Self(0)
    }
}
